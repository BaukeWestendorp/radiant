use anyhow::Context as _;
use zeevonk::{project::Stage, value::AttributeValues};

use crate::{Command, Config, Objects};

pub mod compositor;
pub mod trigger;

pub(crate) struct Pipeline {
    trigger_resolver: trigger::TriggerResolver,
    compositor: compositor::Compositor,
}

impl Pipeline {
    pub fn new(config: &Config) -> anyhow::Result<Self> {
        Ok(Self {
            trigger_resolver: trigger::TriggerResolver::new(config.triggers())
                .context("failed to build trigger resolver")?,
            compositor: compositor::Compositor::new(),
        })
    }

    pub fn resolve_triggers(&mut self) -> anyhow::Result<Vec<Command>> {
        self.trigger_resolver.resolve()
    }

    pub fn compose(&mut self, objects: &Objects, stage: &Stage) -> anyhow::Result<AttributeValues> {
        self.compositor.compose(objects, stage)
    }
}
