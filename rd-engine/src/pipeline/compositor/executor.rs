use std::time::Instant;

use crate::{
    object::{
        Cue, Executor, ExecutorContent, MergeMode, Objects, RecipeContent, SequenceExecutorContent,
    },
    patch::Patch,
    pipeline::cache::PipelineCache,
    value::{AttributeValues, ClampedValue},
};

pub fn compose(
    objects: &Objects,
    patch: &Patch,
    cache: &PipelineCache,
    output: &mut AttributeValues,
) -> anyhow::Result<()> {
    compose_sequence_executors(objects, patch, cache, output);

    Ok(())
}

fn compose_sequence_executors(
    objects: &Objects,
    patch: &Patch,
    cache: &PipelineCache,
    output: &mut AttributeValues,
) {
    let mut sequence_executors = objects
        .executors()
        .filter(|(_, exec)| exec.enabled() && exec.is_sequence_executor())
        .collect::<Vec<_>>();

    // Sort by priority first, then by activation time.
    // Oldest/lowest priority executors are compsed first.
    // Newer/higher priority executors are composed later, overwriting LTP channels.
    sequence_executors.sort_by_key(|(_id, exec)| match exec.content() {
        Some(ExecutorContent::Sequence(sc)) => (sc.priority(), sc.last_activation_time()),
        None => (u32::MIN, Instant::now()),
    });

    for (id, executor) in sequence_executors {
        if let Err(err) = compose_sequence_executor(executor, objects, patch, cache, output) {
            log::error!("Failed to compose Sequence Executor {}: {}", id, err);
        };
    }
}

fn compose_sequence_executor(
    executor: &Executor,
    objects: &Objects,
    patch: &Patch,
    cache: &PipelineCache,
    output: &mut AttributeValues,
) -> anyhow::Result<()> {
    match executor.content() {
        Some(ExecutorContent::Sequence(sc)) => {
            let sequence = objects.sequences.get_by_object_id(&sc.sequence())?;
            let current_cue = sequence.cue(sc.cue_index())?;
            compose_cue(current_cue, executor, sc, objects, patch, cache, output)?;
        }
        None => {}
    }

    Ok(())
}

fn compose_cue(
    current_cue: &Cue,
    executor: &Executor,
    sequence_content: &SequenceExecutorContent,
    objects: &Objects,
    patch: &Patch,
    cache: &PipelineCache,
    output: &mut AttributeValues,
) -> anyhow::Result<()> {
    let cue_values = values_from_cue(current_cue, objects, patch)?;
    for (fixture_id, attribute, target_value) in cue_values.values() {
        let Some(info) = cache.get(fixture_id, attribute) else {
            log::trace!(
                "Could not find cache for attribute '{}' on fixture with id '{}'",
                attribute,
                fixture_id
            );
            continue;
        };

        let new_value = ClampedValue::new(target_value.as_f32() * executor.master());

        match sequence_content.merge_mode() {
            MergeMode::Ltp => {
                output.set(*fixture_id, attribute.clone(), new_value);
            }
            MergeMode::Htp => {
                let existing_value = output
                    .get(fixture_id, attribute)
                    .map(|v| v.to_clamped_value(info.min, info.max))
                    .unwrap_or_else(|| ClampedValue::new(0.0));

                let merged = if new_value > existing_value { new_value } else { existing_value };
                dbg!((new_value, existing_value, attribute, merged));

                output.set(*fixture_id, attribute.clone(), merged);
            }
        }
    }

    Ok(())
}

fn values_from_cue(cue: &Cue, objects: &Objects, patch: &Patch) -> anyhow::Result<AttributeValues> {
    let mut values = AttributeValues::new();
    for recipe in cue.recipes() {
        for fixture_id in recipe.fixtures().fixture_ids(objects, patch)? {
            match recipe.content() {
                RecipeContent::Static(recipe_values) => {
                    for (attribute, value) in recipe_values {
                        values.set(*fixture_id, attribute.clone(), *value);
                    }
                }
                RecipeContent::Preset(preset_id) => {
                    let preset = match objects.preset_by_object_id(preset_id) {
                        Ok(preset) => preset,
                        Err(err) => {
                            log::error!("{err}");
                            continue;
                        }
                    };

                    // Universal values.
                    for (attribute, value) in preset.universal() {
                        values.set(*fixture_id, attribute.clone(), *value);
                    }

                    // Global values.
                    if let Some(fixture_type_id) =
                        patch.fixture(fixture_id).map(|f| f.gdtf().fixture_type_id())
                        && !preset.global().is_empty()
                    {
                        if let Some(global_values) = preset.global().get(&fixture_type_id) {
                            for (attribute, value) in global_values {
                                values.set(*fixture_id, attribute.clone(), *value);
                            }
                        }
                    }

                    // Selective values.
                    if let Some(selective_values) = preset.selective().get(fixture_id) {
                        for (attribute, value) in selective_values {
                            values.set(*fixture_id, attribute.clone(), *value);
                        }
                    }
                }
            }
        }
    }
    Ok(values)
}
