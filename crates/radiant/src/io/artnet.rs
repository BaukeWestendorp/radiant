use artnet_protocol::{ArtCommand, Output};
use socket2::{Domain, Protocol, Type};
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};

pub struct ArtnetNode {
    socket: UdpSocket,
}

impl ArtnetNode {
    pub fn bind() -> Self {
        let socket = socket2::Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP)).unwrap();
        socket.set_broadcast(true).unwrap();
        socket.set_reuse_port(true).unwrap();
        let addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 6454);
        socket.bind(&addr.into()).unwrap();
        let socket: UdpSocket = socket.into();

        ArtnetNode { socket }
    }

    pub fn send_dmx(&self, data: Vec<u8>) {
        let command = ArtCommand::Output(Output {
            data: data.into(),
            ..Default::default()
        });

        let bytes = command.write_to_buffer().unwrap();
        let addr = "127.0.0.1:6454";
        self.socket.send_to(&bytes, addr).unwrap();
    }
}
