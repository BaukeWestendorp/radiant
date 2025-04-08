use std::{
    net::{IpAddr, Ipv4Addr, Shutdown, SocketAddr},
    time::Duration,
};

use socket2::{Domain, SockAddr, Socket, Type};

use crate::{DEFAULT_PORT, Error, packet::Packet};

const _NETWORK_DATA_LOSS_TIMEOUT: Duration = Duration::from_millis(2500);
const _UNIVERSE_DISCOVERY_INTERVAL: Duration = Duration::from_secs(10);

pub struct Receiver {
    config: ReceiverConfig,

    socket: Socket,
}

impl Receiver {
    pub fn new(config: ReceiverConfig) -> Self {
        let domain = if config.ip.is_ipv4() { Domain::IPV4 } else { Domain::IPV6 };
        let socket = Socket::new(domain, Type::DGRAM, None).unwrap();

        Self { config, socket }
    }

    pub fn config(&self) -> &ReceiverConfig {
        &self.config
    }

    pub fn start(&mut self) -> Result<(), Error> {
        let addr = SocketAddr::new(self.config.ip, self.config.port);
        self.socket.bind(&addr.into())?;
        Ok(())
    }

    pub fn stop(&self) -> Result<(), Error> {
        self.socket.shutdown(Shutdown::Both)?;
        Ok(())
    }

    pub fn recv_packet_from(&self) -> Result<(Packet, SockAddr), Error> {
        const MAX_PACKET_SIZE: usize = 1144;

        let mut buffer = Vec::with_capacity(MAX_PACKET_SIZE);
        let (received, addr) = self.socket.recv_from(buffer.spare_capacity_mut())?;
        // SAFETY: just received into the `buffer`.
        unsafe {
            buffer.set_len(received);
        }

        let packet = Packet::from_bytes(&buffer)?;

        Ok((packet, addr))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceiverConfig {
    pub ip: IpAddr,
    pub port: u16,
}

impl Default for ReceiverConfig {
    fn default() -> Self {
        Self { ip: Ipv4Addr::UNSPECIFIED.into(), port: DEFAULT_PORT }
    }
}
