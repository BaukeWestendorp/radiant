use crate::{
    ComponentIdentifier, Error,
    packet::{DataPacket, Pdu},
};
use dmx::{Universe, UniverseId};
use socket2::{Domain, SockAddr, Socket, Type};
use std::net::SocketAddr;

pub struct Source {
    config: SourceConfig,
    cid: ComponentIdentifier,

    socket: Socket,
    data: Option<Universe>,
}

impl Source {
    pub fn new(config: SourceConfig) -> Self {
        let cid = ComponentIdentifier::new_v4();

        let socket = if config.addr.ip().is_ipv4() {
            Socket::new(Domain::IPV4, Type::DGRAM, None).unwrap()
        } else {
            Socket::new(Domain::IPV6, Type::DGRAM, None).unwrap()
        };

        Source { config, cid, socket, data: None }
    }

    pub fn set_output(&mut self, universe: Universe) {
        self.data = Some(universe);
    }

    pub fn start(&mut self) -> Result<(), Error> {
        todo!();
    }

    fn send_data_packet(&mut self) -> Result<(), Error> {
        let Some(data) = self.data.clone() else { return Ok(()) };

        let packet = DataPacket::new(&self.config, self.cid, data.into())?;
        let bytes = packet.to_bytes();

        let addr = SockAddr::from(self.config.addr);
        self.socket.send_to(&bytes, &addr).unwrap();

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceConfig {
    pub name: String,
    pub addr: SocketAddr,
    pub priority: u8,
    pub sync_addr: u16,
    pub preview_data: bool,
    pub force_synchronization: bool,
    pub universe: UniverseId,
}
