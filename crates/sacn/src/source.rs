use crate::{
    ComponentIdentifier, DEFAULT_PORT, Error,
    packet::{DataPacket, Pdu},
};
use dmx::{Multiverse, Universe, UniverseId};
use socket2::{Domain, SockAddr, Socket, Type};
use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr, Shutdown, SocketAddr},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

const DMX_SEND_INTERVAL: Duration = Duration::from_millis(44);

pub struct Source {
    inner: Arc<Inner>,
}

impl Source {
    pub fn new(config: SourceConfig) -> Result<Self, Error> {
        let domain = if config.ip.is_ipv4() { Domain::IPV4 } else { Domain::IPV6 };
        let socket = Socket::new(domain, Type::DGRAM, None)?;

        Ok(Source { inner: Arc::new(Inner::new(config, socket, Mutex::new(None))) })
    }

    pub fn config(&self) -> &SourceConfig {
        &self.inner.config
    }

    pub fn set_output(&mut self, data: Multiverse) {
        *self.inner.data.lock().unwrap() = Some(data);
    }

    pub fn start(&self) {
        thread::spawn({
            let inner = Arc::clone(&self.inner);
            move || -> Result<(), Error> {
                inner.start_send_loop()?;
                Ok(())
            }
        });
    }

    pub fn stop(&self) -> Result<(), Error> {
        self.inner.socket.shutdown(Shutdown::Both)?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceConfig {
    pub cid: ComponentIdentifier,
    pub name: String,

    pub ip: IpAddr,
    pub port: u16,

    pub priority: u8,
    pub preview_data: bool,
    pub synchronization_address: u16,
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
    config: SourceConfig,

    socket: Socket,
    addr: SockAddr,
    sequence_numbers: Mutex<HashMap<UniverseId, u8>>,

    data: Mutex<Option<Multiverse>>,
}

impl Inner {
    pub fn new(config: SourceConfig, socket: Socket, data: Mutex<Option<Multiverse>>) -> Self {
        let addr: SockAddr = SocketAddr::new(config.ip, config.port).into();
        Self { config, socket, addr, sequence_numbers: Mutex::new(HashMap::new()), data }
    }

    pub fn start_send_loop(&self) -> Result<(), Error> {
        loop {
            let Some(multiverse) = self.data.lock().unwrap().clone() else { continue };

            for (id, universe) in multiverse.universes() {
                self.send_universe_packet(*id, universe)?;
            }

            thread::sleep(DMX_SEND_INTERVAL);
        }
    }

    fn send_universe_packet(&self, id: UniverseId, universe: &Universe) -> Result<(), Error> {
        let sequence_number = self.next_sequence_number_for_universe(id);

        let packet = DataPacket::from_source_config(
            &self.config,
            sequence_number,
            false,
            id.into(),
            universe.clone().into(),
        )?;

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
}
