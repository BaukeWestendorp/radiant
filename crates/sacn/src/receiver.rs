use std::{
    net::{IpAddr, Ipv4Addr},
    time::Duration,
};

use socket2::{Domain, Socket, Type};

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

    pub fn recv_packet(&self) -> Result<Packet, Error> {
        const MAX_PACKET_SIZE: usize = 1144;

        let mut buffer = Vec::with_capacity(MAX_PACKET_SIZE);
        let received = self.socket.recv(buffer.spare_capacity_mut())?;
        // SAFETY: just received into the `buffer`.
        unsafe {
            buffer.set_len(received);
        }

        let packet = Packet::from_bytes(&buffer)?;

        Ok(packet)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceiverConfig {
    ip: IpAddr,
    port: u16,
}

impl Default for ReceiverConfig {
    fn default() -> Self {
        Self { ip: Ipv4Addr::UNSPECIFIED.into(), port: DEFAULT_PORT }
    }
}
