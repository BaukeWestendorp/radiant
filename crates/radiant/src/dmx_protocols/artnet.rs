use std::net::{ToSocketAddrs, UdpSocket};
use std::sync::{Arc, Mutex};
use std::thread;

use artnet_protocol::*;

use crate::dmx::DmxOutput;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Artnet {
    pub name: String,
    pub destination_ip: String,
    pub universe: u32,

    #[serde(skip)]
    socket: Option<Arc<Mutex<UdpSocket>>>,
    #[serde(skip)]
    broadcast_addr: Option<std::net::SocketAddr>,
}

impl Artnet {
    pub fn open(&mut self) {
        self.socket = Some(Arc::new(Mutex::new(
            UdpSocket::bind(("0.0.0.0", 6454)).unwrap(),
        )));

        self.broadcast_addr = Some(
            (self.destination_ip.clone(), 6454)
                .to_socket_addrs()
                .unwrap()
                .next()
                .unwrap(),
        );

        self.socket
            .as_ref()
            .unwrap()
            .lock()
            .unwrap()
            .set_broadcast(true)
            .unwrap();
    }

    pub fn poll(&self) {
        let buff = ArtCommand::Poll(Poll::default()).write_to_buffer().unwrap();
        self.socket
            .as_ref()
            .unwrap()
            .lock()
            .unwrap()
            .send_to(&buff, &self.broadcast_addr.unwrap())
            .unwrap();
    }

    pub fn send_dmx(&self, dmx_output: &DmxOutput) {
        let socket = self.socket.clone();
        let data = dmx_output
            .get_universe(self.universe)
            .unwrap()
            .get_channels()
            .to_vec();
        let destination_ip = self.destination_ip.clone();

        thread::spawn(move || {
            let command = ArtCommand::Output(Output {
                port_address: 0.into(),
                data: PaddedData::from(data),

                ..Output::default()
            });
            let bytes = command.write_to_buffer().unwrap();
            let addr = (destination_ip, 56537)
                .to_socket_addrs()
                .unwrap()
                .next()
                .unwrap();
            socket
                .unwrap()
                .lock()
                .unwrap()
                .send_to(&bytes, &addr)
                .unwrap();
        })
        .join()
        .unwrap();
    }
}
