use std::sync::RwLock;
use std::thread::{self, JoinHandle};
use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};

use thread_priority::{ThreadBuilderExt as _, ThreadPriority};

use crate::dmx::Multiverse;
use crate::output::OutputDefinition;
use crate::output::instance::enttec::EnttecInstance;
use crate::output::instance::sacn::SacnInstance;
use crate::service::ServiceDelegate;

pub struct OutputService {
    definition: OutputDefinition,

    multiverse: Arc<RwLock<Multiverse>>,

    sacn_instances: Vec<SacnInstance>,
    enttec_instances: Vec<EnttecInstance>,
}

impl ServiceDelegate for OutputService {
    fn on_start(&self) -> anyhow::Result<()> {
        // for instance in &mut self.sacn_instances {
        //     if let Err(err) = instance.start(self.notify_rx.clone(), self.multiverse.clone()) {
        //         log::error!("Failed to start sACN output instance: {err}");
        //     }
        // }

        // for instance in &mut self.enttec_instances {
        //     if let Err(err) = instance.start(self.notify_rx.clone(), self.multiverse.clone()) {
        //         log::error!("Failed to start Enttec output instance: {err}");
        //     }
        // }

        dbg!("start output service");
        Ok(())
    }

    fn on_tick(&self) -> anyhow::Result<()> {
        dbg!("tick output service");
        Ok(())
    }

    fn on_stop(&self) -> anyhow::Result<()> {
        // for instance in &mut self.sacn_instances {
        //     instance.stop();
        // }

        // for instance in &mut self.enttec_instances {
        //     instance.stop();
        // }

        dbg!("stop output service");
        Ok(())
    }

    fn name(&self) -> &'static str {
        "Output"
    }
}

impl OutputService {
    pub fn new(definition: OutputDefinition) -> anyhow::Result<Self> {
        let multiverse = Arc::new(RwLock::new(Multiverse::new()));

        let sacn_instances = definition
            .sacn
            .instances()
            .iter()
            .map(|instance| SacnInstance::new(instance.clone()))
            .collect::<anyhow::Result<Vec<_>>>()?;

        let enttec_instances = definition
            .enttec
            .instances()
            .iter()
            .map(|instance| EnttecInstance::new(instance.clone()))
            .collect::<anyhow::Result<Vec<_>>>()?;

        Ok(Self { definition, multiverse, sacn_instances, enttec_instances })
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
