use std::{
    sync::{Arc, Mutex, RwLock},
    time::Instant,
};

use rd_dmx::Multiverse;
use rd_service::{Notified, Service};

use crate::project;

mod artnet;

pub struct OutputService {
    artnet_output_service: Mutex<Service<artnet::ArtnetOutputService, Notified>>,

    notify_tx: flume::Sender<()>,
    multiverse: Arc<RwLock<Multiverse>>,
}

impl OutputService {
    pub fn new(config: &project::OutputConfig) -> Self {
        let (notify_tx, notify_rx) = flume::bounded(1);
        let multiverse = Arc::new(RwLock::new(Multiverse::new()));

        Self {
            artnet_output_service: Mutex::new(Service::new(
                artnet::ArtnetOutputService::new(config.artnet.clone(), multiverse.clone()),
                Notified::new(notify_rx),
            )),

            notify_tx,
            multiverse,
        }
    }

    pub fn update_multiverse(&self, multiverse: Multiverse) {
        *self.multiverse.write().unwrap() = multiverse;
    }
}

impl Default for OutputService {
    fn default() -> Self {
        let (notify_tx, notify_rx) = flume::bounded(1);

        Self {
            notify_tx,
            multiverse: Arc::new(RwLock::new(Multiverse::new())),

            artnet_output_service: Mutex::new(Service::new(
                Default::default(),
                Notified::new(notify_rx),
            )),
        }
    }
}

impl rd_service::Delegate for OutputService {
    type Error = anyhow::Error;
    type Data = Instant;

    fn on_start(&self) -> Result<(), Self::Error> {
        self.artnet_output_service.lock().expect("Service should not be locked").start()?;
        Ok(())
    }

    fn on_frame(&self, _instant: Instant) -> Result<(), Self::Error> {
        let _ = self.notify_tx.send(());
        Ok(())
    }

    fn on_stop(&self) -> Result<(), Self::Error> {
        self.artnet_output_service.lock().expect("Service should not be locked").stop()?;
        Ok(())
    }
}
