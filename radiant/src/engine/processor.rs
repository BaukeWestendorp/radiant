use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::engine::ShowHandle;
use crate::engine::event::{EngineEvent, EventHandler};
use crate::pipeline::Pipeline;
use crate::show::{Cue, PresetContent, Show};

const PROCESSOR_FRAME_TIME: Duration = Duration::from_millis(30);

pub fn start(pipeline: Arc<Mutex<Pipeline>>, show: ShowHandle, event_handler: Arc<EventHandler>) {
    thread::Builder::new()
        .name("processor".to_string())
        .spawn(move || {
            loop {
                process_frame(&pipeline, &show, &event_handler);
                spin_sleep::sleep(PROCESSOR_FRAME_TIME);
            }
        })
        .unwrap();
}

fn process_frame(
    pipeline: &Arc<Mutex<Pipeline>>,
    show: &ShowHandle,
    event_handler: &Arc<EventHandler>,
) {
    show.update(|show| {
        process_default_values(pipeline, show);
        process_executors(pipeline, show, event_handler);

        // Merge programmer values into output pipeline.
        for (fid, attr, value) in show.programmer.values() {
            pipeline.lock().unwrap().set_value(*fid, attr.clone(), *value);
        }

        // Resolve output pipeline.
        pipeline.lock().unwrap().resolve(&show.patch());
    });
}

fn process_default_values(pipeline: &Arc<Mutex<Pipeline>>, show: &Show) {
    // Put each fixture's default values into the output pipeline before resolving
    // other values.
    for fixture in show.patch().fixtures().to_vec() {
        for (attribute, value) in fixture.get_default_attribute_values(show.patch()) {
            pipeline.lock().unwrap().set_value(fixture.fid(), attribute.clone(), value);
        }
    }
}

fn process_executors(
    pipeline: &Arc<Mutex<Pipeline>>,
    show: &mut Show,
    event_handler: &Arc<EventHandler>,
) {
    for executor in show.executors.objects() {
        let Some(sequence_id) = executor.sequence_id() else { continue };
        let Some(sequence) = show.sequences.get_mut(sequence_id) else { continue };
        sequence.update_fade_times();
    }

    let mut fade_in_progress = false;

    for executor in show.executors.objects() {
        if !executor.is_on() {
            continue;
        }

        let Some(sequence) = executor.sequence(show) else { continue };

        for cue in sequence.active_cues() {
            let is_current =
                sequence.current_cue().as_ref().is_some_and(|current| current.id() == cue.id());

            if let Some(fade_in_start) = sequence.cue_fade_in_starts.get(cue.id()) {
                let prev_cue = sequence.cue_before(cue.id());
                let elapsed = fade_in_start.elapsed().as_secs_f32();
                let duration = cue.fade_in_time().as_secs_f32();
                let t = if duration > 0.0 { (elapsed / duration).clamp(0.0, 1.0) } else { 1.0 };
                fade_cue(prev_cue, Some(cue), t, pipeline, show);
                fade_in_progress = true;
            } else if let Some(fade_out_start) = sequence.cue_fade_out_starts.get(cue.id()) {
                let next_cue = sequence.cue_after(cue.id());
                let elapsed = fade_out_start.elapsed().as_secs_f32();
                let duration = cue.fade_out_time().as_secs_f32();
                let t = if duration > 0.0 { (elapsed / duration).clamp(0.0, 1.0) } else { 1.0 };
                fade_cue(Some(cue), next_cue, t, pipeline, show);
                fade_in_progress = true;
            } else if is_current {
                process_cue(cue, &mut pipeline.lock().unwrap(), show);
            }
        }
    }

    if fade_in_progress {
        event_handler.emit_event(EngineEvent::CueFadeInProgress);
    }
}

fn fade_cue(
    from_cue: Option<&Cue>,
    to_cue: Option<&Cue>,
    t: f32,
    pipeline: &Arc<Mutex<Pipeline>>,
    show: &Show,
) {
    let mut from_pipeline = Pipeline::new();
    if let Some(from_cue) = from_cue {
        process_cue(from_cue, &mut from_pipeline, show);
    }
    let mut to_pipeline = Pipeline::new();
    if let Some(to_cue) = to_cue {
        process_cue(to_cue, &mut to_pipeline, show);
    }

    let blended = Pipeline::lerp(&from_pipeline, &to_pipeline, t, show.patch());
    pipeline.lock().unwrap().merge(&blended)
}

fn process_cue(cue: &Cue, pipeline: &mut Pipeline, show: &Show) {
    for recipe in cue.recipes() {
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
                        pipeline.set_value(*fid, attribute.clone(), *value);
                    }
                }
            }
            PresetContent::Global(global_preset) => {
                for ((fixture_type_id, attribute), value) in global_preset.values() {
                    for fid in group.fids() {
                        let Some(fixture) = show.patch().fixture(*fid) else {
                            continue;
                        };
                        if fixture.fixture_type_id() == fixture_type_id {
                            pipeline.set_value(*fid, attribute.clone(), *value);
                        }
                    }
                }
            }
            PresetContent::Selective(selective_preset) => {
                for ((preset_fid, attribute), value) in selective_preset.values() {
                    for fid in group.fids() {
                        if fid == preset_fid {
                            pipeline.set_value(*fid, attribute.clone(), *value);
                        }
                    }
                }
            }
        }
    }
}
