use crate::{
    ComponentIdentifier, DEFAULT_PORT, Error,
    packet::{DataPacket, Pdu},
};
use dmx::{Multiverse, UniverseId};
use socket2::{Domain, SockAddr, Socket, Type};
use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr, Shutdown, SocketAddr},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

const DMX_UPDATE_DELAY: Duration = Duration::from_millis(44);

pub struct Source {
    config: SourceConfig,

    socket: Arc<Socket>,
    data: Arc<Mutex<Option<Multiverse>>>,
}

impl Source {
    pub fn new(config: SourceConfig) -> Self {
        let domain = if config.ip.is_ipv4() { Domain::IPV4 } else { Domain::IPV6 };
        let socket = Arc::new(Socket::new(domain, Type::DGRAM, None).unwrap());

        Source { config, socket, data: Arc::new(Mutex::new(None)) }
    }

    pub fn set_output(&mut self, data: Multiverse) {
        *self.data.lock().unwrap() = Some(data);
    }

    pub fn start(&self) {
        thread::spawn({
            let config = self.config.clone();
            let socket = self.socket.clone();
            let data = self.data.clone();

            move || -> Result<(), Error> {
                let mut sequence_numbers = HashMap::<UniverseId, u8>::new();
                let addr: SockAddr = SocketAddr::new(config.ip, config.port).into();
                loop {
                    let Some(multiverse) = data.lock().unwrap().clone() else { continue };

                    for universe in multiverse.universes() {
                        let sequence_number = sequence_numbers
                            .insert(
                                universe.id(),
                                sequence_numbers
                                    .get(&universe.id())
                                    .copied()
                                    .unwrap_or_default()
                                    .wrapping_add(1),
                            )
                            .unwrap_or_default();

                        let packet = DataPacket::from_source_config(
                            &config,
                            sequence_number,
                            false,
                            universe.id().into(),
                            universe.clone().into(),
                        )?;

                        let bytes = packet.to_bytes();
                        socket.send_to(&bytes, &addr)?;
                    }

                    thread::sleep(DMX_UPDATE_DELAY);
                }
            }
        });
    }

    pub fn stop(&self) -> Result<(), Error> {
        self.socket.shutdown(Shutdown::Both)?;
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
