//! An sACN Source.
//!
//! Responsible for sending sACN packets.

use super::packet::{DataFraming, Dmp, Packet, PacketError, Pdu};
use super::{ComponentIdentifier, DEFAULT_PORT, Universe};
use socket2::{Domain, SockAddr, Socket, Type};
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, Shutdown, SocketAddr};
use std::sync::Mutex;
use std::time::{Duration, Instant};

const DMX_SEND_INTERVAL: Duration = Duration::from_millis(44);
const UNIVERSE_DISCOVERY_INTERVAL: Duration = Duration::from_secs(10);

/// Error type returned by a [Source].
#[derive(Debug, thiserror::Error)]
pub enum SourceError {
    /// An [std::io::Error] wrapper.
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// An [PacketError] wrapper.
    #[error(transparent)]
    Packet(#[from] PacketError),
}

/// An sACN Source.
///
/// Responsible for sending sACN packets.
pub struct Source {
    config: SourceConfig,

    socket: Socket,
    addr: SockAddr,
    sequence_numbers: Mutex<HashMap<u16, u8>>,
    last_universe_discovery_time: Mutex<Option<Instant>>,
}

impl Source {
    /// Creates a new [Source].
    pub fn new(config: SourceConfig) -> Result<Self, SourceError> {
        let domain = if config.ip.is_ipv4() { Domain::IPV4 } else { Domain::IPV6 };
        let socket = Socket::new(domain, Type::DGRAM, None)?;
        let addr: SockAddr = SocketAddr::new(config.ip, config.port).into();

        Ok(Source {
            config,
            socket,
            addr,
            sequence_numbers: Mutex::new(HashMap::new()),
            last_universe_discovery_time: Mutex::new(None),
        })
    }

    /// Returns the [SourceConfig] for this [Source].
    pub fn config(&self) -> &SourceConfig {
        &self.config
    }

    /// Returns the [SourceConfig] for this [Source].
    pub fn config_mut(&mut self) -> &mut SourceConfig {
        &mut self.config
    }

    /// Shut down this [Source].
    pub fn shutdown(&self) -> Result<(), SourceError> {
        log::info!("shutting down sACN source");
        self.socket.shutdown(Shutdown::Both)?;
        Ok(())
    }

    /// Returns the port of the socket used by the [Source].
    ///
    /// Returns `None` if the socket is not bound.
    pub fn socket_port(&self) -> Option<u16> {
        Some(self.socket.local_addr().ok()?.as_socket()?.port())
    }

    pub fn send_universe_data_packet(&self, universe: Universe) -> Result<(), SourceError> {
        let sequence_number = self.next_sequence_number_for_universe(universe.number);

        let packet = {
            let dmp = Dmp::new(universe.slots());
            let data_framing = DataFraming::from_source_config(
                &self.config,
                sequence_number,
                false,
                universe.number,
                dmp,
            )?;
            let pdu = Pdu::DataFraming(data_framing);
            Packet::new(self.config.cid, pdu)
        };

        let bytes = packet.encode();
        self.socket.send_to(&bytes, &self.addr)?;

        Ok(())
    }

    fn next_sequence_number_for_universe(&self, universe_number: u16) -> u8 {
        let mut seq_nums = self.sequence_numbers.lock().unwrap();
        let current = seq_nums.get(&universe_number).copied().unwrap_or_default();
        let next = current.wrapping_add(1);
        seq_nums.insert(universe_number, next);
        next
    }
}

impl Drop for Source {
    fn drop(&mut self) {
        self.shutdown().ok();
    }
}

/// Configuration for a [Source].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceConfig {
    /// [ComponentIdentifier] for the source.
    pub cid: ComponentIdentifier,
    /// Name of the source.
    pub name: String,

    /// IP address the source should send to.
    pub ip: IpAddr,
    /// Port number the source should send to.
    pub port: u16,

    /// The priority of the data packets sent by the source.
    pub priority: u8,
    /// Whether the source should send preview data.
    ///
    /// The preview data flag indicates that the data sent is
    /// intended for use in visualization or media server preview
    /// applications and shall not be used to generate live output.
    pub preview_data: bool,
    /// The synchronization universe of the source.
    pub synchronization_address: u16,
    /// Indicates whether to lock or revert to an
    /// unsynchronized state when synchronization is lost.
    ///
    /// When set to `false`, components that had been operating in a
    /// synchronized state will not update with any new packets until
    /// synchronization resumes.
    ///
    /// When set to `true` once synchronization has been lost, components that
    /// had been operating in a synchronized state don't have to wait for a
    /// new synchronization packet in order to update to the next data packet.
    pub force_synchronization: bool,
}

impl Default for SourceConfig {
    fn default() -> Self {
        Self {
            cid: ComponentIdentifier::new_v4(),
            name: "New sACN Source".to_string(),

            ip: Ipv4Addr::UNSPECIFIED.into(),
            port: DEFAULT_PORT,

            priority: 100,
            preview_data: false,
            synchronization_address: 0,
            force_synchronization: false,
        }
    }
}
