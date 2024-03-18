use std::sync::{Arc, Mutex};
use std::thread;

use artnet::ArtnetSocket;

use crate::dmx::DmxOutput;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Artnet {
    pub name: String,
    pub destination_ip: String,
    pub local_universe: u32,

    #[serde(skip)]
    socket: Option<Arc<Mutex<ArtnetSocket>>>,
}

impl Artnet {
    pub fn open(&mut self) {
        log::info!(
            "Opening Artnet connection to {} ({})",
            self.name,
            self.destination_ip
        );

        let socket = ArtnetSocket::new(&self.destination_ip).unwrap();

        self.socket = Some(Arc::new(Mutex::new(socket)));

        log::info!(
            "Opened Artnet connection to {} ({})",
            self.name,
            self.destination_ip
        );
    }

    pub fn send_dmx(&self, dmx_output: &DmxOutput) {
        let socket = self.socket.clone().unwrap();
        let data = dmx_output
            .get_universe(self.local_universe)
            .unwrap()
            .get_channels()
            .to_vec();

        thread::spawn(move || socket.lock().unwrap().send_dmx(data))
            .join()
            .unwrap();
    }
}
