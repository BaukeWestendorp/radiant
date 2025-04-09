//! An sACN Source.
//!
//! Responsible for sending sACN packets.

use crate::{
    ComponentIdentifier, DEFAULT_PORT, Error,
    packet::{DataPacket, UniverseDiscoveryPacket},
};
use dmx::{Multiverse, Universe, UniverseId};
use socket2::{Domain, SockAddr, Socket, Type};
use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr, Shutdown, SocketAddr},
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

const DMX_SEND_INTERVAL: Duration = Duration::from_millis(44);
const UNIVERSE_DISCOVERY_INTERVAL: Duration = Duration::from_secs(10);

/// An sACN Source.
///
/// Responsible for sending sACN packets.
pub struct Source {
    inner: Arc<Inner>,
}

impl Source {
    /// Creates a new [Source].
    pub fn new(config: SourceConfig) -> Result<Self, Error> {
        let domain = if config.ip.is_ipv4() { Domain::IPV4 } else { Domain::IPV6 };
        let socket = Socket::new(domain, Type::DGRAM, None)?;

        Ok(Source { inner: Arc::new(Inner::new(config, socket, Mutex::new(None))) })
    }

    /// Returns the [SourceConfig] for this [Source].
    pub fn config(&self) -> SourceConfig {
        self.inner.config.lock().unwrap().clone()
    }

    /// Sets the configuration for this [Source].
    pub fn set_config(&mut self, config: SourceConfig) {
        *self.inner.config.lock().unwrap() = config;
    }

    /// Sets the CID for this [Source].
    pub fn set_cid(&mut self, cid: ComponentIdentifier) {
        self.inner.config.lock().unwrap().cid = cid;
    }

    /// Sets the name of this [Source].
    pub fn set_name(&mut self, name: String) {
        self.inner.config.lock().unwrap().name = name;
    }

    /// Sets the priority of this [Source].
    pub fn set_priority(&mut self, priority: u8) {
        self.inner.config.lock().unwrap().priority = priority;
    }

    /// Sets the preview data flag for this [Source].
    pub fn set_preview_data(&mut self, enabled: bool) {
        self.inner.config.lock().unwrap().preview_data = enabled;
    }

    /// Sets the synchronization address for this [Source].
    pub fn set_synchronization(&mut self, address: u16) {
        self.inner.config.lock().unwrap().synchronization_address = address;
    }

    /// Sets the force synchronization flag for this [Source].
    pub fn set_forced_synchronization(&mut self, enabled: bool) {
        self.inner.config.lock().unwrap().force_synchronization = enabled;
    }

    /// Sets the output data for this [Source].
    pub fn set_output(&mut self, data: Multiverse) {
        *self.inner.data.lock().unwrap() = Some(data);
    }

    /// Starts the [Source].
    ///
    /// Calling this method will start the source in a new thread.
    pub fn start(&self) {
        thread::spawn({
            let inner = Arc::clone(&self.inner);
            move || -> Result<(), Error> {
                inner.start_send_loop()?;
                Ok(())
            }
        });
    }

    /// Stops the [Source].
    pub fn stop(&self) -> Result<(), Error> {
        self.inner.socket.shutdown(Shutdown::Both)?;
        Ok(())
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

struct Inner {
    config: Mutex<SourceConfig>,

    socket: Socket,
    addr: SockAddr,
    sequence_numbers: Mutex<HashMap<UniverseId, u8>>,
    last_universe_discovery_time: Mutex<Option<Instant>>,

    data: Mutex<Option<Multiverse>>,
}

impl Inner {
    pub fn new(config: SourceConfig, socket: Socket, data: Mutex<Option<Multiverse>>) -> Self {
        let addr: SockAddr = SocketAddr::new(config.ip, config.port).into();
        Self {
            config: Mutex::new(config),
            socket,
            addr,
            sequence_numbers: Mutex::new(HashMap::new()),
            last_universe_discovery_time: Mutex::new(None),
            data,
        }
    }

    pub fn start_send_loop(&self) -> Result<(), Error> {
        self.send_discovery_packet()?;

        loop {
            thread::sleep(DMX_SEND_INTERVAL);
            let Some(multiverse) = self.data.lock().unwrap().clone() else { continue };

            for (id, universe) in multiverse.universes() {
                self.send_universe_data_packet(*id, universe)?;
            }

            if self.should_send_discovery_packet() {
                self.send_discovery_packet()?;
            }
        }
    }

    fn send_universe_data_packet(&self, id: UniverseId, universe: &Universe) -> Result<(), Error> {
        let sequence_number = self.next_sequence_number_for_universe(id);

        let packet = {
            let config = self.config.lock().unwrap();
            DataPacket::from_source_config(
                &config,
                sequence_number,
                false,
                id.into(),
                universe.clone().into(),
            )?
        };

        let bytes = packet.to_bytes();
        self.socket.send_to(&bytes, &self.addr)?;

        Ok(())
    }

    fn next_sequence_number_for_universe(&self, universe_id: UniverseId) -> u8 {
        let mut seq_nums = self.sequence_numbers.lock().unwrap();
        let current = seq_nums.get(&universe_id).copied().unwrap_or_default();
        let next = current.wrapping_add(1);
        seq_nums.insert(universe_id, next);
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

    fn send_discovery_packet(&self) -> Result<(), Error> {
        let create_and_send_packet = |page, last, list_of_universes| -> Result<(), Error> {
            let packet = {
                let config = self.config.lock().unwrap();
                UniverseDiscoveryPacket::from_source_config(&config, page, last, list_of_universes)?
            };

            let bytes = packet.to_bytes();
            self.socket.send_to(&bytes, &self.addr)?;

            Ok(())
        };

        let universe_ids = {
            let data = self.data.lock().unwrap();
            data.as_ref()
                .map(|multiverse| multiverse.universes().map(|(id, _)| *id).collect::<Vec<_>>())
                .unwrap_or_default()
        };

        let pages = universe_ids.chunks(512).take(u8::MAX as usize);

        let last_page = (pages.len() - 1) as u8;
        for (ix, page) in pages.enumerate() {
            let list_of_universes = page.iter().map(|id| (*id).into()).collect();
            create_and_send_packet(ix as u8, last_page, list_of_universes)?;
        }

        let mut last_time = self.last_universe_discovery_time.lock().unwrap();
        *last_time = Some(Instant::now());

        Ok(())
    }
}
