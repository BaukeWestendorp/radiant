use artnet_protocol::{ArtCommand, Output};
use std::io;
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};
use std::sync::{Arc, Mutex};

pub const ARTNET_PORT: u16 = 6454;

#[derive(Debug, Clone)]
pub struct ArtnetSocket {
    socket: Arc<Mutex<UdpSocket>>,
    target_address: SocketAddr,
}

impl ArtnetSocket {
    pub fn new(target_ip: &str) -> io::Result<Self> {
        let target_address = (target_ip, ARTNET_PORT)
            .to_socket_addrs()
            .expect("Could not resolve address")
            .next()
            .unwrap();

        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.set_broadcast(true)?;
        let socket = Arc::new(Mutex::new(socket));

        Ok(ArtnetSocket {
            socket,
            target_address,
        })
    }

    pub fn send_dmx(&self, port_address: u16, data: Vec<u8>) {
        let command = ArtCommand::Output(Output {
            data: data.into(),
            port_address: port_address.try_into().unwrap(),
            ..Output::default()
        });
        let bytes = command.write_to_buffer().unwrap();

        self.socket
            .lock()
            .unwrap()
            .send_to(&bytes, self.target_address)
            .unwrap();
    }
}
