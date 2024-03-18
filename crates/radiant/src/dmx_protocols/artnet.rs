use std::sync::{Arc, Mutex};
use std::thread;

use artnet::ArtnetSocket;

use crate::dmx::DmxOutput;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Artnet {
    pub name: String,
    pub target_ip: String,
    pub target_universe: u16,
    pub local_universe: u16,

    #[serde(skip)]
    socket: Option<Arc<Mutex<ArtnetSocket>>>,
}

impl Artnet {
    pub fn open(&mut self) {
        log::info!(
            "Opening Artnet connection to {} ({})",
            self.name,
            self.target_ip
        );

        let socket = ArtnetSocket::new(&self.target_ip).unwrap();

        self.socket = Some(Arc::new(Mutex::new(socket)));

        log::info!(
            "Opened Artnet connection to {} ({})",
            self.name,
            self.target_ip
        );
    }

    pub fn send_dmx_universe(&self, dmx_output: &DmxOutput) {
        let socket = self.socket.clone().unwrap();
        let data = dmx_output
            .get_universe(self.local_universe)
            .unwrap()
            .get_channels()
            .to_vec();
        let port_address = self.target_universe - 1;
        thread::spawn(move || socket.lock().unwrap().send_dmx(port_address, data))
            .join()
            .unwrap();
    }
}
