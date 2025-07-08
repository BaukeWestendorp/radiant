use crate::engine::EngineUptime;
use crate::object::{AnyPreset, Cue, PresetContent, Recipe, RecipeContent};
use crate::patch::AttributeValue;
use crate::pipeline::Pipeline;
use crate::show::Show;

pub fn resolve(engine_uptime: EngineUptime, output_pipeline: &mut Pipeline, show: &mut Show) {
    // Resolve and merge executor outputs.
    resolve_executors(engine_uptime, output_pipeline, show);

    // Resolve and merge programmer pipeline with output pipeline.
    show.programmer.resolve(&show.patch);
    show.programmer.merge_unresolved_into(output_pipeline);

    // Resolve output pipeline.
    output_pipeline.resolve(&show.patch);
}

fn resolve_executors(engine_uptime: EngineUptime, output_pipeline: &mut Pipeline, show: &Show) {
    for executor in show.executors() {
        let Some(active_cue) = executor.active_cue(show) else { continue };
        resolve_cue(active_cue, executor.master_level(), engine_uptime, output_pipeline, show);
    }
}

fn resolve_cue(
    cue: &Cue,
    master_level: f32,
    engine_uptime: EngineUptime,
    output_pipeline: &mut Pipeline,
    show: &Show,
) {
    for recipe in &cue.recipes {
        resolve_recipe(recipe, master_level, engine_uptime, output_pipeline, show);
    }
}

fn resolve_recipe(
    recipe: &Recipe,
    master_level: f32,
    engine_uptime: EngineUptime,
    output_pipeline: &mut Pipeline,
    show: &Show,
) {
    match &recipe.content {
        RecipeContent::Preset(preset_id) => {
            let Some(preset) = show.preset(*preset_id) else {
                log::warn!("{preset_id} not found in recipe");
                return;
            };

            let Some(fixture_group) = show.fixture_group(recipe.fixture_group) else {
                log::warn!("fixture group '{}' not found in recipe", recipe.fixture_group);
                return;
            };

            let content = match preset {
                AnyPreset::Dimmer(preset) => preset.content,
                AnyPreset::Color(preset) => preset.content,
            };

            match &content {
                PresetContent::Selective(preset) => {
                    for ((fixture_id, attribute), value) in preset.get_attribute_values() {
                        // NOTE: We only resolve attributes for
                        //       fixtures that are both in the Fixture Group
                        //       and in the preset.
                        if fixture_group.contains(fixture_id) {
                            let effect_level = recipe
                                .level_effect
                                .as_ref()
                                .map_or(1.0, |level_effect| level_effect.compute(engine_uptime));
                            let level = effect_level * master_level;

                            // FIXME: We should implement different merging strategies like HTP
                            //        but for now let's just lerp with the existing value.
                            let old_value = output_pipeline
                                .get_attribute_value(*fixture_id, attribute)
                                .unwrap_or_default();
                            let lerped_value = AttributeValue::lerp(&old_value, value, level);

                            output_pipeline.set_attribute_value(
                                *fixture_id,
                                attribute.clone(),
                                lerped_value,
                            );
                        }
                    }
                }
                PresetContent::Universal(preset) => {
                    for (attribute, value) in preset.get_attribute_values() {
                        // NOTE: We only resolve attributes for
                        //       fixtures that are both in the Fixture Group
                        //       and in the preset.
                        for fixture_id in fixture_group.fixtures() {
                            let effect_level = recipe
                                .level_effect
                                .as_ref()
                                .map_or(1.0, |level_effect| level_effect.compute(engine_uptime));
                            let level = effect_level * master_level;

                            // FIXME: We should implement different merging strategies like HTP
                            //        but for now let's just lerp with the existing value.
                            let old_value = output_pipeline
                                .get_attribute_value(*fixture_id, attribute)
                                .unwrap_or_default();
                            let lerped_value = AttributeValue::lerp(&old_value, value, level);
                            output_pipeline.set_attribute_value(
                                *fixture_id,
                                attribute.clone(),
                                lerped_value,
                            );
                        }
                    }
                }
            }
        }
    }
}
