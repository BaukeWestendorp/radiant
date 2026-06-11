use crate::{
    object::Objects,
    patch::{FixtureId, Patch},
    pipeline::cache::PipelineCache,
    programmer::Programmer,
    value::AttributeValues,
};

mod executor;

pub fn compose(
    objects: &Objects,
    patch: &Patch,
    programmer: &Programmer,
    highlighted_fixtures: &[FixtureId],
    cache: &PipelineCache,
) -> anyhow::Result<AttributeValues> {
    let defaults = cache.initial_defaults().clone();
    let executor_values = executor::compose(objects, patch, cache)?;
    let programmer_values = programmer.values().clone();

    let mut output = defaults;
    output.extend(executor_values);
    output.extend(programmer_values);

    for fixture_id in highlighted_fixtures {
        for (attribute_name, value) in cache.highlight_values(fixture_id) {
            output.set(*fixture_id, attribute_name.clone(), *value);
        }
    }

    Ok(output)
}
