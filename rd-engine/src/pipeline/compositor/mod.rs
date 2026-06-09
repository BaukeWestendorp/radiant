use crate::{
    object::Objects, patch::Patch, pipeline::cache::PipelineCache, programmer::Programmer,
    value::AttributeValues,
};

mod executor;

#[derive(Default)]
pub struct Compositor {}

impl Compositor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn compose(
        &mut self,
        objects: &Objects,
        patch: &Patch,
        programmer: &Programmer,
        cache: &PipelineCache,
    ) -> anyhow::Result<AttributeValues> {
        let defaults = cache.initial_defaults().clone();
        let executor_values = executor::compose(objects, patch, cache)?;
        let programmer_values = programmer.values().clone();

        let mut output = defaults;
        output.extend(executor_values);
        output.extend(programmer_values);

        Ok(output)
    }
}
