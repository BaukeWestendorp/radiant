use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::engine::ShowHandle;
use crate::pipeline::Pipeline;
use crate::show::PresetContent;

const PROCESSOR_FRAME_TIME: Duration = Duration::from_millis(30);

pub fn start(pipeline: Arc<Mutex<Pipeline>>, show: ShowHandle) {
    thread::Builder::new()
        .name("processor".to_string())
        .spawn(move || {
            loop {
                show.read(|show| {
                    // Put each fixture's default values into the output pipeline before resolving
                    // other values.
                    for fixture in show.patch().fixtures().to_vec() {
                        for (attribute, value) in fixture.get_default_attribute_values(show.patch())
                        {
                            pipeline.lock().unwrap().set_value(
                                fixture.fid(),
                                attribute.clone(),
                                value,
                            );
                        }
                    }

                    // TODO: Properly resolve and merge executor outputs.
                    let sequence = show.sequences.get(1).unwrap();
                    if let Some(active_cue) = sequence.active_cue() {
                        for recipe in &active_cue.recipes {
                            let Some(group_id) = recipe.group else { continue };
                            let Some(group) = show.groups.get(group_id) else {
                                log::warn!("recipe references missing group id: {group_id:?}");
                                continue;
                            };
                            let Some(preset_id) = recipe.preset else { continue };
                            let Some(preset) = show.any_preset(preset_id) else {
                                log::warn!("recipe references missing preset id: {preset_id:?}");
                                continue;
                            };

                            match preset.content() {
                                PresetContent::Universal(universal_preset) => {
                                    for (attribute, value) in universal_preset.values() {
                                        for fid in group.fids() {
                                            pipeline.lock().unwrap().set_value(
                                                *fid,
                                                attribute.clone(),
                                                *value,
                                            );
                                        }
                                    }
                                }
                                PresetContent::Global(global_preset) => {
                                    for ((fixture_type_id, attribute), value) in
                                        global_preset.values()
                                    {
                                        for fid in group.fids() {
                                            let Some(fixture) = show.patch().fixture(*fid) else {
                                                continue;
                                            };
                                            if fixture.fixture_type_id() == fixture_type_id {
                                                pipeline.lock().unwrap().set_value(
                                                    *fid,
                                                    attribute.clone(),
                                                    *value,
                                                );
                                            }
                                        }
                                    }
                                }
                                PresetContent::Selective(selective_preset) => {
                                    for ((preset_fid, attribute), value) in
                                        selective_preset.values()
                                    {
                                        for fid in group.fids() {
                                            if fid == preset_fid {
                                                pipeline.lock().unwrap().set_value(
                                                    *fid,
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

                    // Merge programmer values into output pipeline.
                    for (fid, attr, value) in show.programmer.values() {
                        pipeline.lock().unwrap().set_value(*fid, attr.clone(), *value);
                    }

                    // Resolve output pipeline.
                    pipeline.lock().unwrap().resolve(&show.patch());
                });

                spin_sleep::sleep(PROCESSOR_FRAME_TIME);
            }
        })
        .unwrap();
}
