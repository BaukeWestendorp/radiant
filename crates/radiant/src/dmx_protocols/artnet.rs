use std::sync::{Arc, Mutex};
use std::thread;

use artnet::ArtnetSocket;

use crate::dmx::DmxUniverse;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Artnet {
    pub name: String,
    pub target_ip: String,
    pub target_universe: u32,
    pub local_universe: u32,

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

    pub fn send_dmx_universe(&self, dmx_universe: &DmxUniverse) {
        let socket = self.socket.clone().unwrap();
        let data = dmx_universe.get_channels().to_vec();
        let port_address = dmx_universe.id() - 1;
        thread::spawn(move || socket.lock().unwrap().send_dmx(port_address, data))
            .join()
            .unwrap();
    }
}
