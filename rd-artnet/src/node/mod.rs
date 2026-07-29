use std::{
    collections::HashMap,
    io,
    net::{Ipv4Addr, SocketAddrV4, UdpSocket},
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

use crate::{
    ArtDmx, ArtPoll, ArtPollReply, DiagnosticPriority, FailsafeState, FixedString, GoodInput,
    GoodOutputA, GoodOutputB, IndicatorState, Packet, PacketPayload, PortAddress, PortProtocol,
    PortType, ProgrammingAuthority, Status1, Status2, Status3, Universe, UniverseId,
    node::port::PortManager,
};

mod config;
mod port;

pub use config::*;

const ART_POLL_REPLY_TIMEOUT: Duration = Duration::from_secs(3);

pub struct Node {
    _inner: Arc<Inner>,
    stop_tx: Option<flume::Sender<()>>,
    poller_handle: Option<JoinHandle<()>>,
    receiver_handle: Option<JoinHandle<()>>,
    port_manager: Option<PortManager>,
}

impl Node {
    pub fn new(config: NodeConfig) -> crate::Result<Self> {
        log::info!("Initializing Art-Net Node...");

        let network_details = NetworkDetails::try_from(config.network_config().clone())?;

        let addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, crate::PORT);
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

            state: Mutex::new(NodeState::default()),
            sequence_values: Mutex::new(HashMap::new()),
            nodes: Mutex::new(NodeRegistry::new()),
        });

        let (stop_tx, stop_rx) = flume::unbounded();

        log::debug!("Spawning poller and receiver background threads");
        let poller_handle = start_poller(Arc::clone(&inner), stop_rx.clone());
        let receiver_handle = start_receiver(Arc::clone(&inner), stop_rx.clone());

        let port_manager = PortManager::new(Arc::clone(&inner), stop_rx.clone());

        log::info!("Art-Net Node running");
        Ok(Self {
            _inner: inner,
            stop_tx: Some(stop_tx),
            poller_handle: Some(poller_handle),
            receiver_handle: Some(receiver_handle),
            port_manager: Some(port_manager),
        })
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

        if let Some(port_manager) = self.port_manager.take() {
            for worker in port_manager.workers {
                match worker {
                    port::WorkerHandle::Input { port_address, handle } => {
                        log::debug!(
                            "Waiting for input worker thread for port {} to join...",
                            port_address
                        );
                        let _ = handle
                            .join()
                            .map_err(|_| log::error!("Failed to join input worker thread cleanly"));
                    }
                    port::WorkerHandle::Output { port_address, handle } => {
                        log::debug!(
                            "Waiting for output worker thread for port {} to join...",
                            port_address
                        );
                        let _ = handle.join().map_err(|_| {
                            log::error!("Failed to join output worker thread cleanly")
                        });
                    }
                }
            }
        }

        log::info!("Art-Net Node shut down");
    }
}

struct Inner {
    network_details: NetworkDetails,
    config: NodeConfig,

    socket: UdpSocket,

    state: Mutex<NodeState>,
    sequence_values: Mutex<HashMap<PortAddress, u8>>,
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
        let broadcast_ip = if self.network_details.ip.is_loopback() {
            self.network_details.ip
        } else {
            let ip_octets = self.network_details.ip.octets();
            let mask_octets = self.network_details.mask.octets();
            Ipv4Addr::new(
                ip_octets[0] | !mask_octets[0],
                ip_octets[1] | !mask_octets[1],
                ip_octets[2] | !mask_octets[2],
                ip_octets[3] | !mask_octets[3],
            )
        };

        log::trace!("Broadcast IP: {}", broadcast_ip);
        self.send_packet(payload, broadcast_ip)
    }

    fn send_dmx(
        &self,
        universe: Universe,
        port_address: PortAddress,
        physical: u8,
    ) -> crate::Result<()> {
        let sequence = {
            let mut sequence_values_guard = self.sequence_values.lock().unwrap();
            let sequence = sequence_values_guard.entry(port_address).or_insert(0);
            *sequence = sequence.wrapping_add(1);
            *sequence
        };

        let subscribers = {
            let mut registry = self.nodes.lock().unwrap();
            registry.get_subscribers(port_address)
        };

        if subscribers.is_empty() {
            return Ok(());
        }

        let mut art_dmx = ArtDmx::new();
        art_dmx.set_port_address(port_address);
        art_dmx.set_data(universe.as_bytes().to_vec());
        art_dmx.set_sequence(sequence);
        art_dmx.set_physical(physical);

        for ip in subscribers {
            self.send_packet(art_dmx.clone(), ip)?;
        }

        Ok(())
    }

    fn register_node(&self, art_poll_reply: ArtPollReply) {
        let mut node_registry_guard = self.nodes.lock().unwrap();
        node_registry_guard.update(art_poll_reply);
    }
}

fn start_poller(inner: Arc<Inner>, stop_rx: flume::Receiver<()>) -> JoinHandle<()> {
    thread::spawn(move || {
        const POLL_INTERVAL: Duration = Duration::from_millis(2500);
        log::debug!("Started poller thread. Polling interval: {:?}", POLL_INTERVAL);

        let poll = || {
            log::debug!("Executing periodic ArtPoll broadcast");

            let mut art_poll = ArtPoll::new();
            art_poll.flags_mut().set_send_reply_on_change(true);
            art_poll.set_oem(inner.config.oem_code());
            art_poll.set_esta_man(inner.config.esta_man());
            // FIXME: Get this from config.
            art_poll.set_diag_priority(DiagnosticPriority::DpAll);

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

            let (size, source) = match inner.socket.recv_from(&mut buf) {
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

            log::trace!("Read successful: received {} bytes from {}", size, source);

            let bytes = bytes::Bytes::copy_from_slice(&buf[..size]);
            let packet = match Packet::decode(&bytes) {
                Ok(packet) => packet,
                Err(err) => {
                    log::error!("Could not decode packet from {}: {err:#}", source);
                    continue;
                }
            };

            let source_ip = match source {
                std::net::SocketAddr::V4(addr) => *addr.ip(),
                std::net::SocketAddr::V6(_) => {
                    log::warn!("Received packet from IPv6 address {}. Ignoring.", source);
                    continue;
                }
            };

            if let Err(err) = handle_packet(&inner, packet, source_ip) {
                log::error!("Failed to handle packet from {}: {err:#}", source_ip);
            }
        }

        log::debug!("Stopped receiver thread");
    })
}

fn handle_packet(inner: &Arc<Inner>, packet: Packet, source_ip: Ipv4Addr) -> crate::Result<()> {
    match packet.payload {
        PacketPayload::ArtPoll(_) => {
            log::debug!("Handling incoming ArtPoll from {}", source_ip);

            // FIXME: Art-Net 4 specifies Targeted Mode (Flags bit 5).
            // If enabled, we must check if any of our ports fall between
            // TargetPortAddress Top and Bottom before responding.

            let state = inner.state.lock().unwrap().clone();
            let reply_strategy = inner.config.poll_reply_strategy();

            let send_reply = |bound_node: &BoundNodeConfig| -> crate::Result<()> {
                let mut reply = ArtPollReply::new();
                reply.set_ip_address(inner.network_details.ip);
                reply.set_vers_info(inner.config.version_info());
                reply.set_net_switch(bound_node.net());
                reply.set_sub_switch(bound_node.sub_net());
                reply.set_oem(inner.config.oem_code());
                reply.set_ubea_version(inner.config.ueba_version());
                *reply.status1_mut() = Status1::new()
                    .with_indicator_state(state.indicator_state)
                    .with_programming_authority(state.programming_authority)
                    .with_booted_from_rom(state.booted_from_rom)
                    .with_rdm_capable(false) // FIXME: Implement RDM support.
                    .with_ubea_present(state.ubea_present);
                reply.set_esta_man(inner.config.esta_man());
                reply.set_port_name(inner.config.port_name().clone());
                reply.set_long_name(inner.config.long_name().clone());
                // FIXME: Implement Node Reports.
                reply.set_node_report(FixedString::default());
                reply.set_num_ports(bound_node.port_count() as u16);
                for (ix, port) in bound_node.ports().iter().enumerate() {
                    match port {
                        Some(port) => {
                            reply.port_types_mut()[ix] = PortType::new()
                                .with_can_input_artnet(port.input().is_some())
                                .with_can_output_artnet(port.output().is_some())
                                // NOTE: This always should be DMX512, as our DMX provider only allows for updating a simple
                                // DMX Universe with 512 channels and not via any other protocols.
                                .with_protocol(PortProtocol::Dmx512);
                            reply.good_input_mut()[ix] = GoodInput::new()
                                .with_data_received(false) // FIXME: Make library internal state.
                                .with_includes_test_packets(false)
                                .with_includes_sips(false)
                                .with_includes_text_packets(false)
                                .with_input_disabled(false) // FIXME: Implement disabling inputs and outputs.
                                .with_receive_errors_detected(false) // FIXME: Make library internal state.
                                .with_convert_to_sacn(false);
                            reply.good_output_a_mut()[ix] = GoodOutputA::new()
                                .with_data_being_output(false) // FIXME: Make library internal state.
                                .with_includes_test_packets(false)
                                .with_includes_sips(false)
                                .with_includes_text_packets(false)
                                .with_merging_artnet(false) // FIXME: Implement merging.
                                .with_short_detected(false) // FIMXE: Imlplement short detection.
                                .with_merge_mode_is_ltp(false) // FIXME: Implement merging.
                                .with_convert_from_sacn(false);
                            reply.good_output_b_mut()[ix] = GoodOutputB::new()
                                .with_rdm_disabled(true) // FIXME: Implement RDM.
                                .with_output_style_is_continuous(true) // FIXME: Implement different output styles.
                                .with_discovery_not_running(false)
                                .with_bg_discovery_disabled(false);

                            reply.sw_in_mut()[ix] = port
                                .input()
                                .unwrap_or(PortAddress::from_absolute(0).unwrap())
                                .universe();
                            reply.sw_out_mut()[ix] = port
                                .output()
                                .unwrap_or(PortAddress::from_absolute(0).unwrap())
                                .universe();
                        }
                        None => {
                            reply.port_types_mut()[ix] = PortType::new()
                                .with_can_input_artnet(false)
                                .with_can_output_artnet(false)
                                .with_protocol(PortProtocol::Dmx512);
                            reply.good_input_mut()[ix] = GoodInput::new();
                            reply.good_output_a_mut()[ix] = GoodOutputA::new();
                            reply.good_output_b_mut()[ix] = GoodOutputB::new();
                            reply.sw_in_mut()[ix] =
                                UniverseId::new(0).expect("0 Should be a valid UniverseId");
                            reply.sw_out_mut()[ix] =
                                UniverseId::new(0).expect("0 Should be a valid UniverseId");
                        }
                    }
                }
                reply.set_acn_priority(bound_node.sacn_priority());
                *reply.sw_macro_mut() = bound_node.macros();
                *reply.sw_remote_mut() = bound_node.remotes();
                reply.set_style(bound_node.style());
                reply.set_mac(inner.network_details.mac_address);
                reply.set_bind_ip(inner.network_details.ip);
                reply.set_bind_index(bound_node.bind_index());
                *reply.status2_mut() = Status2::new()
                    .with_supports_rdm_control(false) // FIXME: Implement RDM.
                    .with_supports_output_style_switching(false)
                    .with_squawking(false) // FIXME: Connect to internal squawk state.
                    .with_able_to_switch_artnet_sacn(false)
                    .with_dhcp_capable(inner.config.network_config().dhcp_capable())
                    .with_ip_dhcp_configured(inner.network_details.assignment == IpAssignment::Dhcp)
                    .with_supports_web_browser_configuration(
                        inner.config.supports_web_browser_configuration(),
                    );
                *reply.status3_mut() = Status3::new()
                    .with_failsafe_state(FailsafeState::HoldLastState) // FIXME: Implement Failsafe states.
                    .with_supports_programmable_failsafe(false) // FIXME: Implement Failsafe states.
                    .with_supports_llrp(false)
                    .with_supports_switching_port_direction(false) // FIXME: Implement switching port direction.
                    .with_supports_rdmnet(false) // FIXME: Implement RDMNet
                    .with_bg_discovery_can_be_disabled(false);
                reply.set_default_resp_uid(bound_node.default_resp_uid());
                reply.set_user(bound_node.user_data());
                reply.set_refresh_rate(bound_node.frame_scheduler().refresh_rate());
                *reply.bg_queue_policy_mut() = bound_node.bg_queue_policy();

                match reply_strategy {
                    PollReplyStrategy::Unicast => {
                        log::debug!(
                            "Unicasting ArtPollReply for bound node with index {} to {}",
                            bound_node.bind_index(),
                            source_ip
                        );
                        inner.send_packet(reply, source_ip)?;
                    }
                    PollReplyStrategy::Broadcast => {
                        log::debug!(
                            "Broadcasting ArtPollReply for bound node with index {} (compatibility mode)",
                            bound_node.bind_index()
                        );
                        inner.broadcast_packet(reply)?;
                    }
                }

                Ok(())
            };

            send_reply(inner.config.bound_node_config())?;

            for bound_node in inner.config.bound_nodes() {
                send_reply(bound_node)?;
            }
        }
        PacketPayload::ArtPollReply(art_poll_reply) => {
            log::debug!("Handling ArtPollReply");
            inner.register_node(art_poll_reply);
        }
        PacketPayload::ArtDmx(_) => {
            // FIXME: Route incoming DMX to the correct PortManager Input Worker.
            // We need to check if the PortAddress matches any of our `Input` ports,
            // and if so, send the data over a channel to the worker for merging and state updates.
        }
    }

    Ok(())
}

#[derive(Clone)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "facet", facet(tag = "type"))]
#[repr(C)]
pub enum FrameScheduler {
    Internal {
        refresh_rate: u16,
    },
    External {
        refresh_rate: u16,
        #[cfg_attr(feature = "facet", facet(opaque))]
        notifier: Arc<Box<dyn Fn(flume::Sender<()>) + Send + Sync>>,
    },
}

impl FrameScheduler {
    pub fn refresh_rate(&self) -> u16 {
        match self {
            FrameScheduler::Internal { refresh_rate, .. } => *refresh_rate,
            FrameScheduler::External { refresh_rate, .. } => *refresh_rate,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NodeState {
    pub indicator_state: IndicatorState,
    pub programming_authority: ProgrammingAuthority,
    pub booted_from_rom: bool,
    pub ubea_present: bool,
}

impl Default for NodeState {
    fn default() -> Self {
        Self {
            indicator_state: IndicatorState::Normal,
            programming_authority: ProgrammingAuthority::NotUsed,
            booted_from_rom: false,
            ubea_present: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RegisteredNode {
    pub reply: ArtPollReply,
    pub last_seen: Instant,
}

#[derive(Debug, Clone)]
struct NodeRegistry {
    nodes: HashMap<(Ipv4Addr, u8), RegisteredNode>,
}

impl NodeRegistry {
    pub fn new() -> Self {
        Self { nodes: HashMap::new() }
    }

    pub fn update(&mut self, reply: ArtPollReply) {
        let key = (*reply.ip_address(), reply.bind_index());

        if !self.nodes.contains_key(&key) {
            log::info!(
                "Discovered new Art-Net node: {} (Bind Index: {})",
                reply.ip_address(),
                reply.bind_index()
            );
        }

        self.nodes.insert(key, RegisteredNode { reply, last_seen: Instant::now() });
    }

    pub fn get_subscribers(&mut self, port_address: PortAddress) -> Vec<Ipv4Addr> {
        let now = Instant::now();

        self.nodes.retain(|_, node| now.duration_since(node.last_seen) < ART_POLL_REPLY_TIMEOUT);

        let mut subscribers = Vec::new();

        for node in self.nodes.values() {
            let net = node.reply.net_switch();
            let sub_net = node.reply.sub_switch();

            if net == port_address.net() && sub_net == port_address.sub_net() {
                let mut is_subscribed = false;

                for (i, port_type) in node.reply.port_types().iter().enumerate() {
                    let sw_out_match = port_type.can_output_artnet()
                        && node.reply.sw_out()[i] == port_address.universe();
                    let sw_in_match = port_type.can_input_artnet()
                        && node.reply.sw_in()[i] == port_address.universe();

                    if sw_out_match || sw_in_match {
                        is_subscribed = true;
                        break;
                    }
                }

                if is_subscribed && !subscribers.contains(node.reply.ip_address()) {
                    subscribers.push(*node.reply.ip_address());
                }
            }
        }

        subscribers
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
struct NetworkDetails {
    ip: Ipv4Addr,
    mask: Ipv4Addr,
    mac_address: [u8; 6],
    assignment: IpAssignment,
}

impl TryFrom<NodeNetworkConfig> for NetworkDetails {
    type Error = crate::Error;

    fn try_from(value: NodeNetworkConfig) -> Result<Self, Self::Error> {
        match value {
            NodeNetworkConfig::Custom { ip, mask, mac_address, dhcp_capable, .. } => {
                log::info!("Using custom network configuration: IP {}, Mask {}", ip, mask);
                Ok(Self {
                    ip,
                    mask,
                    mac_address,
                    assignment: IpAssignment::Static { dhcp_capable },
                })
            }
            NodeNetworkConfig::Interface { name, assignment, .. } => {
                log::debug!("Resolving default network configuration from interfaces");
                let interfaces = if_addrs::get_if_addrs()?;
                let mut ipv4_interfaces = interfaces
                    .into_iter()
                    .filter(|iface| matches!(iface.addr, if_addrs::IfAddr::V4(_)));
                let target_interface = match name {
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
                        log::warn!(
                            "Failed to retrieve MAC address for interface '{}'. Defaulting to zeroed MAC address",
                            iface.name
                        );
                        [0x00; 6]
                    }
                };

                if let if_addrs::IfAddr::V4(v4_addr) = iface.addr {
                    log::info!(
                        "Network interface '{}' to IP: {}, Mask: {}",
                        iface.name,
                        v4_addr.ip,
                        v4_addr.netmask
                    );
                    Ok(Self { ip: v4_addr.ip, mask: v4_addr.netmask, mac_address, assignment })
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
