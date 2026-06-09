mod cache;
mod compositor;
mod mapper;

use crate::{
    cmd::Command,
    dmx::Multiverse,
    object::Objects,
    patch::{FixtureId, Patch},
    programmer::Programmer,
    trigger::{Trigger, Triggers},
    value::AttributeValues,
};

#[derive(Default)]
pub struct Pipeline {
    compositor: compositor::Compositor,
    mapper: mapper::Mapper,

    cache: cache::PipelineCache,
}

impl Pipeline {
    pub fn new(patch: &Patch) -> Self {
        Self {
            compositor: compositor::Compositor::new(),
            mapper: mapper::Mapper::new(),
            cache: cache::PipelineCache::new(patch),
        }
    }

    pub fn resolve_triggers(&self, triggers: &Triggers) -> Vec<Command> {
        let mut commands = Vec::new();
        for trigger in triggers.drain() {
            match trigger {
                Trigger::ExecutorMaster { executor_id, value } => {
                    commands.push(Command::ExecutorSetMaster { executor_id, value });
                }
                Trigger::ExecutorButton { executor_id, button, pressed } => {
                    commands.push(Command::ExecutorButton { executor_id, button, pressed });
                }
            }
        }
        commands
    }

    pub fn resolve_attributes(
        &mut self,
        objects: &Objects,
        patch: &Patch,
        programmer: &Programmer,
    ) -> anyhow::Result<AttributeValues> {
        self.compositor.compose(objects, patch, programmer, &self.cache)
    }

    pub fn resolve_dmx(
        &self,
        attributes: &AttributeValues,
        highlighted_fixtures: Option<&[FixtureId]>,
    ) -> Multiverse {
        let mut multiverse = self.mapper.map(attributes, &self.cache);

        // Resolve highlighted fixture DMX values.
        if let Some(highlighted_fixtures) = highlighted_fixtures {
            for fixture_id in highlighted_fixtures {
                let Some(values) = self.cache.highlights().get(fixture_id) else {
                    log::error!(
                        "Could not get cached highlight values for fixture with id '{}'",
                        fixture_id
                    );
                    continue;
                };

                for (address, value) in values {
                    multiverse.set_value(address, *value);
                }
            }
        }

        multiverse
    }
}
