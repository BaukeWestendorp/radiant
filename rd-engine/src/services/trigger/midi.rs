use rd_midi::MidiPacket;

use crate::{ExecutorId, project, services::trigger::Trigger};

pub struct MidiTriggerService {
    mappings: Vec<project::midi::MidiMapping>,
    trigger_tx: flume::Sender<Trigger>,
}

impl MidiTriggerService {
    pub fn new(
        mappings: Vec<project::midi::MidiMapping>,
        trigger_tx: flume::Sender<Trigger>,
    ) -> Self {
        Self { mappings, trigger_tx }
    }

    fn parse_trigger(&self, packet: MidiPacket) -> Option<Trigger> {
        let msg_channel = match &packet.message {
            rd_midi::MidiMessage::ControlChange { channel, .. } => *channel,
            rd_midi::MidiMessage::NoteOn { channel, .. } => *channel,
            rd_midi::MidiMessage::NoteOff { channel, .. } => *channel,
            rd_midi::MidiMessage::PitchBend { channel, .. } => *channel,
            _ => return None,
        };

        for mapping in &self.mappings {
            if mapping.device_name != packet.port_name {
                continue;
            }

            if let Some(dev_channel) = mapping.device_channel {
                if dev_channel != msg_channel {
                    continue;
                }
            }

            let (mut normalized_val, mut is_pressed) = match (&mapping.filter_type, &packet.message)
            {
                (
                    project::midi::FilterType::ControlChange,
                    rd_midi::MidiMessage::ControlChange { controller: pkt_ctrl, value, .. },
                ) if mapping.filter_controller.map_or(true, |c| c == *pkt_ctrl) => {
                    (*value as f32 / 127.0, *value > 0)
                }

                (
                    project::midi::FilterType::NoteOn,
                    rd_midi::MidiMessage::NoteOn { note: pkt_note, velocity, .. },
                ) if mapping.filter_note.map_or(true, |n| n == *pkt_note) => {
                    (*velocity as f32 / 127.0, *velocity > 0)
                }

                (
                    project::midi::FilterType::NoteOff,
                    rd_midi::MidiMessage::NoteOff { note: pkt_note, .. },
                ) if mapping.filter_note.map_or(true, |n| n == *pkt_note) => (0.0, false),

                (
                    project::midi::FilterType::NoteOff,
                    rd_midi::MidiMessage::NoteOn { note: pkt_note, velocity: 0, .. },
                ) if mapping.filter_note.map_or(true, |n| n == *pkt_note) => (0.0, false),

                (
                    project::midi::FilterType::PitchBend,
                    rd_midi::MidiMessage::PitchBend { value, .. },
                ) => ((*value as f32 + 8192.0) / 16383.0, *value > 0),

                _ => continue,
            };

            if mapping.transform_invert {
                normalized_val = 1.0 - normalized_val;
                is_pressed = !is_pressed;
            }

            let final_value = mapping.transform_min_output
                + (normalized_val * (mapping.transform_max_output - mapping.transform_min_output));

            return match &mapping.target {
                project::TriggerTarget::HighlightToggle => {
                    if is_pressed {
                        Some(Trigger::ToggleHighlight)
                    } else {
                        None
                    }
                }
                project::TriggerTarget::ExecutorMaster { page_id, slot } => {
                    Some(Trigger::ExecutorMaster {
                        executor_id: ExecutorId::new(*page_id, *slot),
                        value: final_value,
                    })
                }
                project::TriggerTarget::ExecutorButton { page_id, slot, button } => {
                    Some(Trigger::ExecutorButton {
                        executor_id: ExecutorId::new(*page_id, *slot),
                        button: *button,
                        pressed: is_pressed,
                    })
                }
                project::TriggerTarget::Encoder { encoder_ix } => {
                    Some(Trigger::EncoderSetValue { encoder_ix: *encoder_ix, value: final_value })
                }
            };
        }

        None
    }
}

impl rd_service::Delegate for MidiTriggerService {
    type Error = anyhow::Error;
    type Data = MidiPacket;

    fn on_start(&self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn on_frame(&self, packet: MidiPacket) -> Result<(), Self::Error> {
        if let Some(trigger) = self.parse_trigger(packet) {
            let _ = self.trigger_tx.send(trigger);
        }
        Ok(())
    }

    fn on_stop(&self) -> Result<(), Self::Error> {
        Ok(())
    }
}
