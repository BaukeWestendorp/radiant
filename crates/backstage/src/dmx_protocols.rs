use anyhow::Result;
use artnet_protocol::{ArtCommand, Output};
use dmx::DmxOutput;
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};

const ARTNET_PORT: u16 = 6454;

#[derive(Debug)]
pub struct ArtnetDmxProtocol {
    socket: ArtnetSocket,
}

impl ArtnetDmxProtocol {
    pub fn new() -> Result<Self> {
        Ok(Self {
            socket: ArtnetSocket::new("0.0.0.0")?,
        })
    }

    pub fn send_dmx_output(&self, dmx_output: &DmxOutput) {
        for universe in dmx_output.universes() {
            self.socket
                .send_dmx(universe.id(), universe.get_channels().to_vec());
        }
    }
}

#[derive(Debug)]
pub struct ArtnetSocket {
    socket: UdpSocket,
    target_address: SocketAddr,
}

impl ArtnetSocket {
    pub fn new(target_ip: &str) -> Result<Self> {
        let target_address = (target_ip, ARTNET_PORT)
            .to_socket_addrs()
            .expect("Could not resolve address")
            .next()
            .unwrap();

        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.set_broadcast(true)?;

        Ok(ArtnetSocket {
            socket,
            target_address,
        })
    }

    fn send_dmx(&self, port_address: u16, data: Vec<u8>) {
        let command = ArtCommand::Output(Output {
            data: data.into(),
            port_address: port_address.try_into().unwrap(),
            ..Output::default()
        });
        let bytes = command.write_to_buffer().unwrap();

        self.socket.send_to(&bytes, self.target_address).unwrap();
    }
}
