use crate::backend::object::{AnyPreset, Cue, PresetContent, Recipe, RecipeContent};
use crate::backend::{pipeline::Pipeline, show::Show};
use crate::dmx::UniverseId;

pub fn resolve(output_pipeline: &mut Pipeline, show: &mut Show) {
    // Resolve and merge executor outputs.
    resolve_executors(output_pipeline, show);

    // Resolve and merge programmer pipeline with output pipeline.
    resolve_programmer(output_pipeline, show);

    // Resolve output pipeline.
    resolve_output_pipeline(output_pipeline, show);

    let multiverse = output_pipeline.output_multiverse();
    eprintln!("{:?}", &multiverse.universe(&UniverseId::new(1).unwrap()).unwrap().values()[0..8]);
}

fn resolve_executors(output_pipeline: &mut Pipeline, show: &Show) {
    for executor in show.executors() {
        let Some(active_cue) = executor.get_active_cue(show) else { continue };
        resolve_cue(active_cue, output_pipeline, show);
    }
}

fn resolve_cue(cue: &Cue, output_pipeline: &mut Pipeline, show: &Show) {
    for recipe in &cue.recipes {
        resolve_recipe(recipe, output_pipeline, show);
    }
}

fn resolve_recipe(recipe: &Recipe, output_pipeline: &mut Pipeline, show: &Show) {
    match &recipe.content {
        RecipeContent::Preset(preset_id) => {
            let Some(preset) = show.preset(preset_id) else {
                log::warn!("Preset with id {preset_id} in RecipeCombination not found");
                return;
            };

            let Some(fixture_group) = show.fixture_group(&recipe.fixture_group_id) else {
                log::warn!(
                    "FixtureGroup with id {} in RecipeCombination not found",
                    recipe.fixture_group_id
                );
                return;
            };

            let content = match preset {
                AnyPreset::Dimmer(preset) => &preset.content,
            };

            match &content {
                PresetContent::Selective(selective_preset) => {
                    for ((fixture_id, attribute), value) in selective_preset.get_attribute_values()
                    {
                        // NOTE: We only resolve attributes for
                        //       fixtures that are both in the Fixture Group
                        //       and in the Preset.
                        if fixture_group.contains(fixture_id) {
                            output_pipeline.set_attribute_value(
                                *fixture_id,
                                attribute.clone(),
                                *value,
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
    show.programmer.merge_into(output_pipeline);
}

fn resolve_output_pipeline(output_pipeline: &mut Pipeline, show: &Show) {
    output_pipeline.resolve(&show.patch);
}
