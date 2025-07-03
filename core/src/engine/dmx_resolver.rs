use crate::object::{AnyPreset, Cue, PresetContent, Recipe, RecipeContent};
use crate::patch::AttributeValue;
use crate::pipeline::Pipeline;
use crate::show::Show;

pub fn resolve(output_pipeline: &mut Pipeline, show: &mut Show) {
    // Resolve and merge executor outputs.
    resolve_executors(output_pipeline, show);

    // Resolve and merge programmer pipeline with output pipeline.
    resolve_programmer(output_pipeline, show);

    // Resolve output pipeline.
    resolve_output_pipeline(output_pipeline, show);
}

fn resolve_executors(output_pipeline: &mut Pipeline, show: &Show) {
    for executor in show.executors() {
        let Some(active_cue) = executor.active_cue(show) else { continue };
        resolve_cue(active_cue, executor.master_level(), output_pipeline, show);
    }
}

fn resolve_cue(cue: &Cue, level: f32, output_pipeline: &mut Pipeline, show: &Show) {
    for recipe in &cue.recipes {
        resolve_recipe(recipe, level, output_pipeline, show);
    }
}

fn resolve_recipe(recipe: &Recipe, level: f32, output_pipeline: &mut Pipeline, show: &Show) {
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
                PresetContent::Selective(selective_preset) => {
                    for ((fixture_id, attribute), value) in selective_preset.get_attribute_values()
                    {
                        // NOTE: We only resolve attributes for
                        //       fixtures that are both in the Fixture Group
                        //       and in the Preset.
                        if fixture_group.contains(fixture_id) {
                            // FIXME: We should implement different merging strategies like HTP
                            //        But for now let's just lerp with the existing value.
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
                        //       and in the Preset.
                        for fixture_id in fixture_group.fixtures() {
                            // FIXME: We should implement different merging strategies like HTP
                            //        But for now let's just lerp with the existing value.
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

fn resolve_programmer(output_pipeline: &mut Pipeline, show: &mut Show) {
    show.programmer.resolve(&show.patch);
    show.programmer.merge_unresolved_into(output_pipeline);
}

fn resolve_output_pipeline(output_pipeline: &mut Pipeline, show: &Show) {
    output_pipeline.resolve(&show.patch);
}
