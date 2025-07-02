//! An sACN Source.
//!
//! Responsible for sending sACN packets.

use super::{
    ComponentIdentifier, DEFAULT_PORT, Universe,
    packet::{DataFraming, DiscoveryFraming, Dmp, Packet, PacketError, Pdu, UniverseDiscovery},
};
use socket2::{Domain, SockAddr, Socket, Type};
use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr, Shutdown, SocketAddr},
    sync::Mutex,
    time::{Duration, Instant},
};

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
    config: Mutex<SourceConfig>,

    socket: Socket,
    addr: SockAddr,
    sequence_numbers: Mutex<HashMap<u16, u8>>,
    last_universe_discovery_time: Mutex<Option<Instant>>,

    universes: Mutex<Vec<Universe>>,
}

impl Source {
    /// Creates a new [Source].
    pub fn new(config: SourceConfig) -> Result<Self, SourceError> {
        let domain = if config.ip.is_ipv4() { Domain::IPV4 } else { Domain::IPV6 };
        let socket = Socket::new(domain, Type::DGRAM, None)?;
        let addr: SockAddr = SocketAddr::new(config.ip, config.port).into();

        Ok(Source {
            config: Mutex::new(config),
            socket,
            addr,
            sequence_numbers: Mutex::new(HashMap::new()),
            last_universe_discovery_time: Mutex::new(None),
            universes: Mutex::new(Vec::new()),
        })
    }

    /// Sets the universe data for this [Source].
    ///
    /// This method updates the universe data for the specified universe ID.
    /// If the universe ID does not exist, it will be created.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sacn::{source::{SourceConfig, Source}, Universe};
    /// let source = Source::new(SourceConfig::default()).unwrap();
    /// source.set_universe(Universe::new(1));
    /// ```
    pub fn set_universe(&self, universe: Universe) {
        self.universes.lock().unwrap().push(universe);
    }

    /// Removes the universe data for the specified universe ID.
    ///
    /// This method removes the universe data for the specified universe ID.
    /// If the universe ID does not exist, it will do nothing.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sacn::{source::{SourceConfig, Source}, Universe};
    /// let source = Source::new(SourceConfig::default()).unwrap();
    /// source.set_universe(Universe::new(1));
    /// source.remove_universe(1);
    /// ```
    pub fn remove_universe(&self, number: u16) {
        self.universes.lock().unwrap().retain(|u| u.number != number);
    }

    /// Removes all universe data.
    ///
    /// This method removes all universe data.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sacn::{source::{SourceConfig, Source}, Universe};
    /// let source = Source::new(SourceConfig::default()).unwrap();
    /// source.set_universe(Universe::new(1));
    /// source.clear_universes();
    /// ```
    pub fn clear_universes(&self) {
        self.universes.lock().unwrap().clear();
    }

    /// Returns the [SourceConfig] for this [Source].
    pub fn config(&self) -> SourceConfig {
        self.config.lock().unwrap().clone()
    }

    /// Sets the configuration for this [Source].
    pub fn set_config(&self, config: SourceConfig) {
        *self.config.lock().unwrap() = config;
    }

    /// Sets the CID for this [Source].
    pub fn set_cid(&self, cid: ComponentIdentifier) {
        self.config.lock().unwrap().cid = cid;
    }

    /// Sets the name of this [Source].
    pub fn set_name(&self, name: String) {
        self.config.lock().unwrap().name = name;
    }

    /// Sets the priority of this [Source].
    pub fn set_priority(&self, priority: u8) {
        self.config.lock().unwrap().priority = priority;
    }

    /// Sets the preview data flag for this [Source].
    pub fn set_preview_data(&self, enabled: bool) {
        self.config.lock().unwrap().preview_data = enabled;
    }

    /// Sets the synchronization address for this [Source].
    pub fn set_synchronization(&self, address: u16) {
        self.config.lock().unwrap().synchronization_address = address;
    }

    /// Sets the force synchronization flag for this [Source].
    pub fn set_forced_synchronization(&self, enabled: bool) {
        self.config.lock().unwrap().force_synchronization = enabled;
    }

    /// Starts the [Source].
    pub fn start(&self) -> Result<(), SourceError> {
        let config = self.config.lock().unwrap();
        log::info!("Starting sACN Source on {}:{}", config.ip, config.port);
        drop(config);

        self.send_discovery_packet()?;

        loop {
            spin_sleep::sleep(DMX_SEND_INTERVAL);
            let data = self.universes.lock().unwrap().clone();

            for universe in data {
                self.send_universe_data_packet(universe)?;
            }

            if self.should_send_discovery_packet() {
                self.send_discovery_packet()?;
            }
        }
    }

    /// Shut down this [Source].
    pub fn shutdown(&self) -> Result<(), SourceError> {
        log::info!("Shutting down sACN Source");
        self.socket.shutdown(Shutdown::Both)?;
        Ok(())
    }

    /// Returns the port of the socket used by the [Source].
    ///
    /// Returns `None` if the socket is not bound.
    pub fn socket_port(&self) -> Option<u16> {
        Some(self.socket.local_addr().ok()?.as_socket()?.port())
    }

    fn send_universe_data_packet(&self, universe: Universe) -> Result<(), SourceError> {
        log::debug!("Sending sACN data packet for universe {}", universe.number);

        let sequence_number = self.next_sequence_number_for_universe(universe.number);

        let packet = {
            let config = self.config.lock().unwrap();
            let dmp = Dmp::new(universe.slots());
            let data_framing = DataFraming::from_source_config(
                &config,
                sequence_number,
                false,
                universe.number,
                dmp,
            )?;
            let pdu = Pdu::DataFraming(data_framing);
            Packet::new(config.cid, pdu)
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

    fn should_send_discovery_packet(&self) -> bool {
        let last_discovery_time = self.last_universe_discovery_time.lock().unwrap();
        match last_discovery_time.as_ref() {
            Some(last_time) => {
                Instant::now().duration_since(*last_time) > UNIVERSE_DISCOVERY_INTERVAL
            }
            _ => false,
        }
    }

    fn send_discovery_packet(&self) -> Result<(), SourceError> {
        log::info!("Sending sACN discovery packet");

        let mut last_time = self.last_universe_discovery_time.lock().unwrap();
        *last_time = Some(Instant::now());

        if self.universes.lock().unwrap().is_empty() {
            return Ok(());
        }

        let create_and_send_packet = |page, last, list_of_universes| -> Result<(), SourceError> {
            let packet = {
                let config = self.config.lock().unwrap();
                let pdu = Pdu::DiscoveryFraming(DiscoveryFraming::from_source_config(
                    &config,
                    UniverseDiscovery::new(page, last, list_of_universes),
                )?);
                Packet::new(config.cid, pdu)
            };

            let bytes = packet.encode();
            self.socket.send_to(&bytes, &self.addr)?;

            Ok(())
        };

        let universe_numbers = {
            let data = self.universes.lock().unwrap();
            data.iter().map(|u| u.number).collect::<Vec<_>>()
        };

        let pages = universe_numbers.chunks(512).take(u8::MAX as usize);

        let last_page = (pages.len() - 1) as u8;
        for (ix, list_of_universes) in pages.enumerate() {
            create_and_send_packet(ix as u8, last_page, list_of_universes.to_vec())?;
        }

        Ok(())
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
    /// When set to `false`, components that had been operating in a synchronized state
    /// will not update with any new packets until synchronization resumes.
    ///
    /// When set to `true` once synchronization has been lost, components that had been
    /// operating in a synchronized state don't have to wait for a
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
