use std::{
    collections::HashMap,
    io,
    net::{Ipv4Addr, SocketAddrV4, UdpSocket},
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

use crate::{
    ArtDmx, ArtPoll, ArtPollReply, DiagnosticPriority, FixedString, Packet, PacketPayload, Universe,
};

mod config;
mod port;

pub use config::*;
pub use port::*;

pub struct Node {
    inner: Arc<Inner>,
    stop_tx: Option<flume::Sender<()>>,
    poller_handle: Option<JoinHandle<()>>,
    receiver_handle: Option<JoinHandle<()>>,
}

impl Node {
    pub fn new(config: NodeConfig) -> crate::Result<Self> {
        log::info!("Initializing Art-Net Node...");

        let network_details = NetworkDetails::try_from(config.network.clone())?;
        let listen_ip = if network_details.ip.is_loopback() {
            network_details.ip
        } else {
            Ipv4Addr::UNSPECIFIED
        };

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
        // FIXME: This feels a bit hacky as it will take 500ms to shut down the Node.
        socket.set_read_timeout(Some(Duration::from_millis(500)))?;
        socket.bind(&socket2::SockAddr::from(addr))?;
        let socket: UdpSocket = socket.into();

        let inner = Arc::new(Inner {
            config,
            network_details,
            socket,
            nodes: Mutex::new(NodeRegistry::new()),
        });

        let (stop_tx, stop_rx) = flume::unbounded();

        log::debug!("Spawning poller and receiver background threads");
        let poller_handle = start_poller(Arc::clone(&inner), stop_rx.clone());
        let receiver_handle = start_receiver(Arc::clone(&inner), stop_rx);

        log::info!("Art-Net Node running");
        Ok(Self {
            inner,
            stop_tx: Some(stop_tx),
            poller_handle: Some(poller_handle),
            receiver_handle: Some(receiver_handle),
        })
    }

    pub fn send_dmx(&self, universe: Universe) -> crate::Result<()> {
        self.inner.send_dmx(universe)
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        log::info!("Art-Net Node shutting down gracefully...");

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

        log::info!("Art-Net Node shut down");
    }
}

struct Inner {
    network_details: NetworkDetails,
    config: NodeConfig,

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
        let ip_octets = self.network_details.ip.octets();
        let mask_octets = self.network_details.mask.octets();
        let broadcast_ip = Ipv4Addr::new(
            ip_octets[0] | !mask_octets[0],
            ip_octets[1] | !mask_octets[1],
            ip_octets[2] | !mask_octets[2],
            ip_octets[3] | !mask_octets[3],
        );

        log::trace!("Broadcast IP: {}", broadcast_ip);
        self.send_packet(payload, broadcast_ip)
    }

    fn send_dmx(&self, universe: Universe) -> crate::Result<()> {
        let mut art_dmx = ArtDmx::new();
        art_dmx.set_port_address(PortAddress::new(
            NetId::new(0).unwrap(),
            SubNetId::new(0).unwrap(),
            UniverseId::new(1).unwrap(),
        ));
        art_dmx.set_data(universe.as_bytes().to_vec());

        self.send_packet(art_dmx, "127.0.0.1".parse().unwrap())?;

        Ok(())
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
            art_poll.set_oem(inner.config.oem_code);
            art_poll.set_esta_man(inner.config.esta_man);
            art_poll.set_diag_priority(DiagnosticPriority::DpLow);

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
            reply.set_long_name(inner.config.name.clone());
            reply.set_esta_man(inner.config.esta_man);
            reply.set_oem(inner.config.oem_code);
            reply.set_vers_info(inner.config.version_info);

            reply.set_mac(inner.network_details.mac_address);
            reply.set_ip_address(inner.network_details.ip);

            reply.set_port_name(FixedString::try_from_str("FIXME: From Conf")?);
            reply.set_net_switch(NetId::new(0x00)?);
            reply.set_sub_switch(SubNetId::new(0x00)?);
            reply.set_bind_ip(inner.network_details.ip);
            reply.set_bind_index(1);

            log::debug!("Broadcasting ArtPollReply in response to ArtPoll");
            inner.broadcast_packet(reply)?;
        }
        PacketPayload::ArtPollReply(art_poll_reply) => {
            log::debug!("Handling ArtPollReply");
            inner.register_node(art_poll_reply);
        }
        PacketPayload::ArtDmx(_) => {}
    }

    Ok(())
}

struct NodeRegistry {
    nodes: HashMap<(Ipv4Addr, (NetId, SubNetId)), NodeRegistry>,
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

struct NetworkDetails {
    ip: Ipv4Addr,
    mask: Ipv4Addr,
    mac_address: [u8; 6],
}

impl TryFrom<NodeNetworkConfig> for NetworkDetails {
    type Error = crate::Error;

    fn try_from(value: NodeNetworkConfig) -> Result<Self, Self::Error> {
        match value {
            NodeNetworkConfig::Custom { ip, mask, mac_address } => {
                log::info!("Using custom network configuration: IP {}, Mask {}", ip, mask);
                Ok(Self { ip, mask, mac_address })
            }
            NodeNetworkConfig::Interface { interface_name } => {
                log::debug!("Resolving default network configuration from interfaces");
                let interfaces = if_addrs::get_if_addrs()?;
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

                let iface = match target_interface {
                    Some(iface) => iface,
                    None => {
                        return Err(crate::Error::Network(
                            "Failed to find a network interface match".to_string(),
                        ));
                    }
                };

                let mac_address = match mac_address::mac_address_by_name(&iface.name)
                    .ok()
                    .flatten()
                    .map(|mac| mac.bytes())
                {
                    Some(mac) => mac,
                    None => {
                        return Err(crate::Error::Network(format!(
                            "Failed to retrieve MAC address for interface '{}'.",
                            iface.name
                        )));
                    }
                };

                if let if_addrs::IfAddr::V4(v4_addr) = iface.addr {
                    log::info!(
                        "Network interface '{}' to IP: {}, Mask: {}",
                        iface.name,
                        v4_addr.ip,
                        v4_addr.netmask
                    );
                    Ok(Self { ip: v4_addr.ip, mask: v4_addr.netmask, mac_address })
                } else {
                    Err(crate::Error::Network(format!(
                        "Interface '{}' does not have an IPv4 address.",
                        iface.name
                    )))
                }
            }
        }
    }
}
