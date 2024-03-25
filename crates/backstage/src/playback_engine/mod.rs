use dmx::DmxOutput;

use crate::show::{Cue, Executor, Show};

#[derive(Debug, Clone, PartialEq)]
pub struct PlaybackEngine {}

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
                let fixtures = show.get_fixtures_in_groups(current_cue.groups.clone());

                for fixture in fixtures.iter() {
                    for (attribute_name, attribute_value) in current_cue.attribute_values.iter() {
                        let Some(attribute_offset) = fixture.attribute_offset(attribute_name)
                        else {
                            continue;
                        };

                        let Some(channel_resolution) =
                            fixture.channel_resolution_for_attribute(attribute_name)
                        else {
                            continue;
                        };

                        let raw_dmx_values =
                            attribute_value.raw_values_for_channel_resolution(channel_resolution);

                        for (offset, value) in attribute_offset.iter().zip(raw_dmx_values) {
                            // Because the offset in the GDTF files starts at 1, we need to
                            // compensate for our zero-based array.
                            let offset = offset.saturating_sub(1);

                            let mut channel = fixture.channel;
                            channel.address += offset as u16;
                            if let Err(err) = output.set_channel(&channel, value) {
                                log::error!("Failed to set channel output: {}", err.to_string())
                            }
                        }
                    }
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
        if let Some(sequence) = executor.sequence.and_then(|id| show.get_sequence(id)) {
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
