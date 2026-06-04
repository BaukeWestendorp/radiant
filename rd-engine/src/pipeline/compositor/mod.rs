use crate::{
    object::Objects, patch::Patch, pipeline::cache::PipelineCache, value::AttributeValues,
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
        cache: &PipelineCache,
    ) -> anyhow::Result<AttributeValues> {
        let defaults = cache.initial_defaults().clone();
        let executor_values = executor::compose(objects, patch, cache)?;

        let mut output = defaults;
        output.extend(executor_values);

        Ok(output)
    }
}
