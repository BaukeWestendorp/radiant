use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::backend::object::{AnyPreset, CueContent, PresetContent, RecipeCombination};
use crate::backend::{pipeline::Pipeline, show::Show};

/// The amount of milliseconds between updating DMX output.
const DMX_UPDATE_INTERVAL: Duration = Duration::from_millis(40);

pub fn start(output_pipeline: Arc<Mutex<Pipeline>>, show: Arc<Mutex<Show>>) {
    loop {
        {
            let show = &mut show.lock().unwrap();

            // Resolve and merge executor outputs.
            resolve_executors(&output_pipeline, show);

            // Resolve and merge programmer pipeline with output pipeline.
            resolve_programmer(&output_pipeline, show);

            // Resolve output pipeline.
            resolve_output_pipeline(&output_pipeline, show);

            let output_pipeline = output_pipeline.lock().unwrap();
            let multiverse = output_pipeline.output_multiverse();
            eprintln!("{multiverse:?}");
        }

        thread::sleep(DMX_UPDATE_INTERVAL);
    }
}

fn resolve_executors(output_pipeline: &Arc<Mutex<Pipeline>>, show: &mut Show) {
    for executor in show.executors.values() {
        let Some(active_cue) = executor.get_active_cue(show) else { continue };
        match &active_cue.content {
            CueContent::Recipe(recipe) => {
                for RecipeCombination { fixture_group_id, preset_id } in &recipe.combinations {
                    let Some(preset) = show.get_preset(preset_id) else {
                        log::warn!("Preset with id {preset_id} in RecipeCombination not found");
                        continue;
                    };

                    let Some(fixture_group) = show.fixture_groups.get(&fixture_group_id) else {
                        log::warn!(
                            "FixtureGroup with id {fixture_group_id} in RecipeCombination not found"
                        );
                        continue;
                    };

                    let content = match preset {
                        AnyPreset::Dimmer(preset) => &preset.content,
                    };

                    match &content {
                        PresetContent::Selective(selective_preset) => {
                            for ((fixture_id, attribute), value) in
                                selective_preset.get_attribute_values()
                            {
                                // NOTE: We should only resolve attributes for
                                //       fixtures that are both in the Fixture Group
                                //       and in the Preset.
                                if fixture_group.contains(fixture_id) {
                                    output_pipeline.lock().unwrap().set_attribute_value(
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
    }
}

fn resolve_programmer(output_pipeline: &Arc<Mutex<Pipeline>>, show: &mut Show) {
    // FIXME: It would be nice if we would not have to clone the entire patch.
    let patch = show.patch.clone();
    show.programmer.resolve(&patch);
    show.programmer.merge_into(&mut output_pipeline.lock().unwrap());
}

fn resolve_output_pipeline(output_pipeline: &Arc<Mutex<Pipeline>>, show: &mut Show) {
    let mut output_pipeline = output_pipeline.lock().unwrap();
    output_pipeline.resolve(&show.patch);
}
