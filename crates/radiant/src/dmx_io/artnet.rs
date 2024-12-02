use artnet_protocol::{ArtCommand, Output};
use show::ArtnetNodeSettings;
use socket2::{Domain, Protocol, Type};
use std::{
    io,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket},
};

pub const PORT: u16 = 6454;

pub struct ArtnetNode {
    pub settings: ArtnetNodeSettings,
    socket: UdpSocket,
}

impl ArtnetNode {
    pub fn bind(settings: ArtnetNodeSettings) -> io::Result<Self> {
        let socket = socket2::Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))?;
        socket.set_broadcast(true)?;
        socket.set_reuse_port(true)?;
        let addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, PORT);
        socket.bind(&addr.into())?;
        let socket: UdpSocket = socket.into();

        Ok(ArtnetNode { settings, socket })
    }

    pub fn send_dmx(&self, data: Vec<u8>) -> anyhow::Result<()> {
        let command = ArtCommand::Output(Output {
            data: data.into(),
            port_address: self.settings.universe.value().try_into()?,
            ..Default::default()
        });

        let bytes = command.write_to_buffer()?;
        let addr = SocketAddr::new(self.settings.destination_ip.into(), PORT);
        self.socket.send_to(&bytes, addr)?;

        Ok(())
    }
}
