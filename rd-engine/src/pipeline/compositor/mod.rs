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
        let mut output = cache.initial_defaults().clone();
        executor::compose(objects, patch, cache, &mut output)?;
        Ok(output)
    }
}
