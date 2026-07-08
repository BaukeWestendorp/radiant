use std::{sync::Mutex, time::Instant};

use rd_service::{Notified, Service};

use crate::project;

mod sacn;

pub struct OutputService {
    notify_tx: flume::Sender<()>,

    sacn_output_service: Mutex<Service<sacn::SacnOutputService, Notified>>,
}

impl OutputService {
    pub fn new(config: &project::OutputConfig) -> Self {
        let (notify_tx, notify_rx) = flume::bounded(1);

        Self {
            notify_tx,

            sacn_output_service: Mutex::new(Service::new(
                sacn::SacnOutputService::new(&config.sacn),
                Notified::new(notify_rx),
            )),
        }
    }
}

impl Default for OutputService {
    fn default() -> Self {
        let (notify_tx, notify_rx) = flume::bounded(1);

        Self {
            notify_tx,
            sacn_output_service: Mutex::new(Service::new(
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
        self.sacn_output_service.lock().expect("Service should not be locked").start()?;
        Ok(())
    }

    fn on_frame(&self, instant: Instant) -> Result<(), Self::Error> {
        let _ = self.notify_tx.send(());
        Ok(())
    }

    fn on_stop(&self) -> Result<(), Self::Error> {
        self.sacn_output_service.lock().expect("Service should not be locked").stop()?;
        Ok(())
    }
}
