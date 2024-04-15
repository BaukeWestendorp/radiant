use crate::show::{Cue, Executor, Show};
use crate::Output;

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

    pub fn determine_output(&self, show: &Show) -> Output {
        let mut output = Output::new();
        for executor in show.executors().iter() {
            if !executor.is_running() {
                continue;
            }

            if let Some(current_cue) = self.get_current_cue_for_executor(executor, show) {
                log::trace!(
                    "Using values for Cue '{}' to determine dmx output",
                    current_cue.label
                );

                for (fixture_id, attribute_values) in current_cue.changes.values.iter() {
                    let values = attribute_values.clone();

                    // FIXME: We currently assume the executor fader controls the master.
                    // FIXME: Implement master control
                    // let master = match executor.flash {
                    //     true => 1.0,
                    //     false => executor.fader_value,
                    // };
                    // attribute_values.iter_mut().for_each(|(_, value)| {
                    //     *value = (*value as f32 * master) as u8;
                    // });

                    output.values.insert(*fixture_id, values);
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
