use dmx::DmxOutput;

use crate::show::{Cue, Executor, Show};
use crate::update_dmx_output_with_attribute_values;

#[derive(Debug, Clone, PartialEq)]
pub struct PlaybackEngine {}

impl Default for PlaybackEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl PlaybackEngine {
    pub fn new() -> Self {
        Self {}
    }

    pub fn determine_dmx_output(&self, show: &Show) -> DmxOutput {
        let mut output = DmxOutput::new();
        for executor in show.executors().iter() {
            if !executor.is_running() {
                continue;
            }

            if let Some(current_cue) = self.get_current_cue_for_executor(executor, show) {
                log::trace!(
                    "Using values for Cue '{}' to determine dmx output",
                    current_cue.label
                );

                for (fixture_id, attribute_values) in current_cue.changes.iter() {
                    let Some(fixture) = show.fixture(*fixture_id) else {
                        log::warn!("Failed to get fixture with id {fixture_id} when determining dmx output");
                        continue;
                    };

                    // FIXME: We should update the DmxValue's values themselves, not the dmx output.
                    // FIXME: We currently assume the executor fader controls the master.
                    // let master = match executor.flash {
                    //     true => 1.0,
                    //     false => executor.fader_value,
                    // };
                    // raw_dmx_values.iter_mut().for_each(|value| {
                    //     *value = (*value as f32 * master) as u8;
                    // });

                    update_dmx_output_with_attribute_values(fixture, attribute_values, &mut output);
                }
            }
        }

        output
    }

    pub fn get_current_cue_for_executor<'a>(
        &'a self,
        executor: &Executor,
        show: &'a Show,
    ) -> Option<&Cue> {
        if let Some(sequence) = executor.sequence.and_then(|id| show.sequence(id)) {
            if let Some(current_index) = executor.current_index.get() {
                if let Some(current_cue) = sequence.cues.get(current_index) {
                    return Some(current_cue);
                } else {
                    log::error!(
                        "Tried to get Executor {}'s current cue, but the index {} is out of bounds",
                        executor.id,
                        current_index
                    )
                }
            }
        }

        None
    }
}
