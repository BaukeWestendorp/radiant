use std::{
    collections::HashMap,
    io,
    net::{Ipv4Addr, SocketAddrV4, UdpSocket},
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

use crate::{ArtPoll, ArtPollReply, FixedString, Packet, PacketPayload};

pub enum NetworkConfig {
    Default { interface_name: Option<String> },
    Custom { ip: Ipv4Addr, mask: Ipv4Addr },
}

impl NetworkConfig {
    pub fn resolve_network_details(&self) -> Option<(Ipv4Addr, Ipv4Addr)> {
        match self {
            NetworkConfig::Custom { ip, mask } => {
                log::info!("Using custom network configuration: IP {}, Mask {}", ip, mask);
                Some((*ip, *mask))
            }
            NetworkConfig::Default { interface_name } => {
                log::debug!("Resolving default network configuration from interfaces");
                let interfaces = if_addrs::get_if_addrs().ok()?;
                let mut ipv4_interfaces = interfaces
                    .into_iter()
                    .filter(|iface| matches!(iface.addr, if_addrs::IfAddr::V4(_)));
                let target_interface = match interface_name {
                    Some(name) => {
                        log::debug!("Searching for requested interface: {}", name);
                        ipv4_interfaces.find(|iface| iface.name == *name)
                    }
                    None => {
                        log::debug!(
                            "No interface specified. Selecting first non-loopback IPv4 interface"
                        );
                        ipv4_interfaces.find(|iface| !iface.is_loopback())
                    }
                };

                let iface = target_interface.or_else(|| {
                    log::warn!("Failed to find a network interface match");
                    None
                })?;

                if let if_addrs::IfAddr::V4(v4_addr) = iface.addr {
                    log::info!(
                        "Network interface '{}' to IP: {}, Mask: {}",
                        iface.name,
                        v4_addr.ip,
                        v4_addr.netmask
                    );
                    Some((v4_addr.ip, v4_addr.netmask))
                } else {
                    None
                }
            }
        }
    }
}

pub struct Source {
    _inner: Arc<Inner>,
    stop_tx: Option<flume::Sender<()>>,
    poller_handle: Option<JoinHandle<()>>,
    receiver_handle: Option<JoinHandle<()>>,
}

impl Source {
    pub fn new(network_config: NetworkConfig) -> crate::Result<Self> {
        log::info!("Initializing Art-Net Source...");
        let (bind_ip, mask) = network_config
            .resolve_network_details()
            .ok_or(crate::Error::Network("Could not resolve network details".to_string()))?;

        // FIXME: Check with Art-Net spec if UNSPECIFIED is allowed/the correct thing to use here.
        let listen_ip = if bind_ip.is_loopback() { bind_ip } else { Ipv4Addr::UNSPECIFIED };

        let addr = SocketAddrV4::new(listen_ip, crate::PORT);
        log::debug!("Creating UDP socket bound to target address: {}", addr);

        let socket = socket2::Socket::new(
            socket2::Domain::IPV4,
            socket2::Type::DGRAM,
            Some(socket2::Protocol::UDP),
        )?;

        socket.set_reuse_address(true)?;
        #[cfg(not(windows))]
        socket.set_reuse_port(true)?;
        socket.set_broadcast(true)?;
        // Set a read timeout so the receiver thread can cleanly exit
        // when checking stop_rx, otherwise it blocks forever on read.
        socket.set_read_timeout(Some(Duration::from_millis(500)))?;
        socket.bind(&socket2::SockAddr::from(addr))?;
        let socket: UdpSocket = socket.into();

        let inner =
            Arc::new(Inner { bind_ip, mask, socket, nodes: Mutex::new(NodeRegistry::new()) });

        let (stop_tx, stop_rx) = flume::unbounded();

        log::debug!("Spawning poller and receiver background threads");
        let poller_handle = start_poller(Arc::clone(&inner), stop_rx.clone());
        let receiver_handle = start_receiver(Arc::clone(&inner), stop_rx);

        log::info!("Art-Net Source running");
        Ok(Self {
            _inner: inner,
            stop_tx: Some(stop_tx),
            poller_handle: Some(poller_handle),
            receiver_handle: Some(receiver_handle),
        })
    }
}

impl Drop for Source {
    fn drop(&mut self) {
        log::info!("Art-Net Source shutting down gracefully...");

        if let Some(tx) = self.stop_tx.take() {
            log::debug!("Sending termination signal to background threads");
            drop(tx);
        }

        if let Some(handle) = self.poller_handle.take() {
            log::debug!("Waiting for poller thread to join...");
            let _ = handle.join().map_err(|_| log::error!("Failed to join poller thread cleanly"));
        }

        if let Some(handle) = self.receiver_handle.take() {
            log::debug!("Waiting for receiver thread to join...");
            let _ =
                handle.join().map_err(|_| log::error!("Failed to join receiver thread cleanly"));
        }

        log::info!("Art-Net Source shut down");
    }
}

struct Inner {
    bind_ip: Ipv4Addr,
    mask: Ipv4Addr,

    socket: UdpSocket,

    nodes: Mutex<NodeRegistry>,
}

impl Inner {
    pub fn send_packet(
        &self,
        payload: impl Into<PacketPayload>,
        ip: Ipv4Addr,
    ) -> crate::Result<()> {
        let payload = payload.into();
        let packet = Packet::new(payload);
        let buf = packet.encode();

        log::trace!("Sending packet ({} bytes) to {}", buf.len(), ip);
        self.socket.send_to(&buf, SocketAddrV4::new(ip, crate::PORT))?;
        Ok(())
    }

    pub fn broadcast_packet(&self, payload: impl Into<PacketPayload>) -> crate::Result<()> {
        let ip_octets = self.bind_ip.octets();
        let mask_octets = self.mask.octets();
        let broadcast_ip = Ipv4Addr::new(
            ip_octets[0] | !mask_octets[0],
            ip_octets[1] | !mask_octets[1],
            ip_octets[2] | !mask_octets[2],
            ip_octets[3] | !mask_octets[3],
        );

        log::trace!("Broadcast IP: {}", broadcast_ip);
        self.send_packet(payload, broadcast_ip)
    }

    fn register_node(&self, art_poll_reply: ArtPollReply) {
        let mut node_registry_guard = self.nodes.lock().unwrap();
        node_registry_guard.register_if_absent(art_poll_reply);
    }
}

fn start_poller(inner: Arc<Inner>, stop_rx: flume::Receiver<()>) -> JoinHandle<()> {
    thread::spawn(move || {
        const POLL_INTERVAL: Duration = Duration::from_millis(2500);
        log::debug!("Started poller thread. Polling interval: {:?}", POLL_INTERVAL);

        let poll = || {
            log::trace!("Executing periodic ArtPoll broadcast");

            let mut art_poll = ArtPoll::new();
            art_poll.set_reply_on_change_enabled(true);
            art_poll.set_oem(0xFFFF);
            art_poll.set_esta_man(0xFFFF);

            if let Err(err) = inner.broadcast_packet(art_poll) {
                log::error!("Failed to broadcast ArtPoll packet: {}", err);
            }
        };

        let mut next_tick = Instant::now();

        loop {
            match stop_rx.try_recv() {
                Ok(()) | Err(flume::TryRecvError::Disconnected) => {
                    break;
                }
                _ => {}
            }

            poll();

            next_tick += POLL_INTERVAL;
            let now = Instant::now();

            if let Some(to_sleep) = next_tick.checked_duration_since(now) {
                thread::sleep(to_sleep);
            } else {
                log::warn!("Poller cycle overrun detected. Skipping sleep interval");
                next_tick = now;
            }
        }

        log::debug!("Stopped poller thread");
    })
}

fn start_receiver(inner: Arc<Inner>, stop_rx: flume::Receiver<()>) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut buf = [0u8; 1024];
        log::debug!("Started receiver thread. Waiting for inbound packets...");

        loop {
            match stop_rx.try_recv() {
                Ok(()) | Err(flume::TryRecvError::Disconnected) => {
                    break;
                }
                _ => {}
            }

            let (size, src) = match inner.socket.recv_from(&mut buf) {
                Ok(result) => result,
                Err(e)
                    if e.kind() == io::ErrorKind::WouldBlock
                        || e.kind() == io::ErrorKind::TimedOut =>
                {
                    continue;
                }
                Err(err) => {
                    log::error!("Failed to receive raw packet data: {}", err);
                    continue;
                }
            };

            log::trace!("Read successful: received {} bytes from {}", size, src);

            let bytes = bytes::Bytes::copy_from_slice(&buf[..size]);
            let packet = match Packet::decode(&bytes) {
                Ok(packet) => packet,
                Err(err) => {
                    log::error!("Could not decode packet from {}: {err:#}", src);
                    continue;
                }
            };

            if let Err(err) = handle_packet(&inner, packet) {
                log::error!("Failed to handle packet from {}: {err:#}", src);
            }
        }

        log::debug!("Stopped receiver thread");
    })
}

fn handle_packet(inner: &Arc<Inner>, packet: Packet) -> crate::Result<()> {
    match packet.payload {
        PacketPayload::ArtPoll(_) => {
            log::debug!("Handling incoming ArtPoll");

            let mut reply = ArtPollReply::new();
            // FIXME: Get these from a config.
            reply.set_port_name(FixedString::try_from_str("Art-Net Node")?);
            reply.set_long_name(FixedString::try_from_str("Art-Net Source Node")?);
            reply.set_esta_man(0x7F00);
            reply.set_oem(0x000);
            reply.set_vers_info(0x00);
            reply.set_net_switch(0x00);
            reply.set_sub_switch(0x00);

            reply.set_ip_address(inner.bind_ip);
            reply.set_bind_ip(inner.bind_ip);
            reply.set_bind_index(1);

            log::debug!("Broadcasting ArtPollReply in response to ArtPoll");
            inner.broadcast_packet(reply)?;
        }
        PacketPayload::ArtPollReply(art_poll_reply) => {
            log::debug!("Handling ArtPollReply");
            inner.register_node(art_poll_reply);
        }
    }

    Ok(())
}

struct NodeRegistry {
    nodes: HashMap<(Ipv4Addr, (u8, u8)), NodeRegistry>,
}

impl NodeRegistry {
    pub fn new() -> Self {
        Self { nodes: HashMap::new() }
    }

    pub fn register_if_absent(&mut self, art_poll_reply: ArtPollReply) {
        let key = (
            *art_poll_reply.ip_address(),
            (art_poll_reply.net_switch(), art_poll_reply.sub_switch()),
        );
        if !self.nodes.contains_key(&key) {
            log::info!(
                "Registering new Art-Net node: {} at {}",
                art_poll_reply.long_name(),
                art_poll_reply.ip_address()
            );
            self.nodes.insert(key, NodeRegistry { nodes: HashMap::new() });
        } else {
            log::debug!(
                "Art-Net node already registered: {} at {}",
                art_poll_reply.long_name(),
                art_poll_reply.ip_address()
            );
        }
    }
}
