use std::sync::Arc;
use std::sync::RwLock;

use rd_service::Service;

use crate::dmx::Multiverse;
use crate::output::OutputDefinition;
use crate::output::instance::enttec::EnttecInstanceService;
use crate::output::instance::sacn::SacnInstanceService;

pub struct OutputService {
    definition: OutputDefinition,

    multiverse: Arc<RwLock<Multiverse>>,
    notify_tx: flume::Sender<()>,

    sacn_instances: Vec<RwLock<Service<SacnInstanceService, rd_service::Notified>>>,
    enttec_instances: Vec<RwLock<Service<EnttecInstanceService, rd_service::Notified>>>,
}

impl rd_service::Delegate for OutputService {
    type Error = anyhow::Error;

    fn on_start(&self) -> Result<(), Self::Error> {
        for instance in &self.sacn_instances {
            instance
                .write()
                .map_err(|err| anyhow::anyhow!("Failed to acquire sACN instance lock: {err}"))?
                .start()?;
        }

        for instance in &self.enttec_instances {
            instance
                .write()
                .map_err(|err| anyhow::anyhow!("Failed to acquire Enttec instance lock: {err}"))?
                .start()?;
        }

        Ok(())
    }

    fn on_frame(&self) -> Result<(), Self::Error> {
        self.notify_tx.send(())?;
        Ok(())
    }

    fn on_stop(&self) -> Result<(), Self::Error> {
        for instance in &self.sacn_instances {
            instance
                .write()
                .map_err(|err| anyhow::anyhow!("Failed to acquire sACN instance lock: {err}"))?
                .stop()?;
        }

        for instance in &self.enttec_instances {
            instance
                .write()
                .map_err(|err| anyhow::anyhow!("Failed to acquire Enttec instance lock: {err}"))?
                .stop()?;
        }

        Ok(())
    }
}

impl OutputService {
    pub fn new(definition: OutputDefinition) -> anyhow::Result<Self> {
        let multiverse = Arc::new(RwLock::new(Multiverse::new()));
        let (notify_tx, notify_rx) = flume::bounded(1);

        let sacn_instances = definition
            .sacn
            .instances()
            .iter()
            .map(|instance| {
                SacnInstanceService::new(instance.clone(), Arc::clone(&multiverse)).map(
                    |instance| {
                        RwLock::new(Service::new(
                            instance,
                            rd_service::Notified::new(notify_rx.clone()),
                        ))
                    },
                )
            })
            .collect::<anyhow::Result<Vec<_>>>()?;

        let enttec_instances = definition
            .enttec
            .instances()
            .iter()
            .map(|instance| {
                EnttecInstanceService::new(instance.clone(), Arc::clone(&multiverse)).map(
                    |instance| {
                        RwLock::new(Service::new(
                            instance,
                            rd_service::Notified::new(notify_rx.clone()),
                        ))
                    },
                )
            })
            .collect::<anyhow::Result<Vec<_>>>()?;

        Ok(Self { definition, notify_tx, multiverse, sacn_instances, enttec_instances })
    }

    pub fn definition(&self) -> &OutputDefinition {
        &self.definition
    }

    pub(crate) fn update(&self, multiverse: Multiverse) {
        *self.multiverse.write().unwrap() = multiverse;
    }
}

impl Default for OutputService {
    fn default() -> Self {
        Self::new(OutputDefinition::default()).unwrap()
    }
}
