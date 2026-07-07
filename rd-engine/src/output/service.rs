use std::sync::Arc;
use std::sync::RwLock;

use crate::dmx::Multiverse;
use crate::output::OutputDefinition;
use crate::output::instance::enttec::EnttecInstanceService;
use crate::output::instance::sacn::SacnInstanceService;
use crate::service::Service;
use crate::service::ServiceDelegate;

pub struct OutputService {
    definition: OutputDefinition,

    multiverse: Arc<RwLock<Multiverse>>,
    notify_tx: flume::Sender<()>,

    sacn_instances: Vec<RwLock<Service<SacnInstanceService>>>,
    enttec_instances: Vec<RwLock<Service<EnttecInstanceService>>>,
}

impl ServiceDelegate for OutputService {
    fn on_start(&self, _tick_tx: flume::Sender<()>) -> anyhow::Result<()> {
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

    fn on_tick(&self) -> anyhow::Result<()> {
        self.notify_tx.send(())?;
        Ok(())
    }

    fn on_stop(&self) -> anyhow::Result<()> {
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

    fn name(&self) -> &'static str {
        "Output"
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
                SacnInstanceService::new(instance.clone(), Arc::clone(&multiverse))
                    .map(|instance| RwLock::new(Service::new_driven(instance, notify_rx.clone())))
            })
            .collect::<anyhow::Result<Vec<_>>>()?;

        let enttec_instances = definition
            .enttec
            .instances()
            .iter()
            .map(|instance| {
                EnttecInstanceService::new(instance.clone(), Arc::clone(&multiverse))
                    .map(|instance| RwLock::new(Service::new_driven(instance, notify_rx.clone())))
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
