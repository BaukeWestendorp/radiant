use rd_midi::MidiPacket;

use crate::{ExecutorId, project, services::trigger::Trigger};

pub struct MidiTriggerService {
    device_config: project::midi::MidiTriggerConfig,
    trigger_tx: flume::Sender<Trigger>,
}

impl MidiTriggerService {
    pub fn new(
        device_config: project::midi::MidiTriggerConfig,
        trigger_tx: flume::Sender<Trigger>,
    ) -> Self {
        Self { device_config, trigger_tx }
    }

    fn parse_trigger(&self, packet: MidiPacket) -> Option<Trigger> {
        let device_config =
            self.device_config.devices.iter().find(|d| d.name == packet.port_name)?;

        let msg_channel = match &packet.message {
            rd_midi::MidiMessage::ControlChange { channel, .. } => *channel,
            rd_midi::MidiMessage::NoteOn { channel, .. } => *channel,
            rd_midi::MidiMessage::NoteOff { channel, .. } => *channel,
            rd_midi::MidiMessage::PitchBend { channel, .. } => *channel,
            _ => return None,
        };

        if let Some(dev_channel) = device_config.channel {
            if dev_channel != msg_channel {
                return None;
            }
        }

        for mapping in &device_config.mappings {
            let (mut normalized_val, mut is_pressed) = match (&mapping.filter, &packet.message) {
                (
                    project::midi::MidiFilter::ControlChange { controller },
                    rd_midi::MidiMessage::ControlChange { controller: pkt_ctrl, value, .. },
                ) if controller.map_or(true, |c| c == *pkt_ctrl) => {
                    (*value as f32 / 127.0, *value > 0)
                }

                (
                    project::midi::MidiFilter::NoteOn { note },
                    rd_midi::MidiMessage::NoteOn { note: pkt_note, velocity, .. },
                ) if note.map_or(true, |n| n == *pkt_note) => {
                    (*velocity as f32 / 127.0, *velocity > 0)
                }

                (
                    project::midi::MidiFilter::NoteOff { note },
                    rd_midi::MidiMessage::NoteOff { note: pkt_note, .. },
                ) if note.map_or(true, |n| n == *pkt_note) => (0.0, false),

                (
                    project::midi::MidiFilter::NoteOff { note },
                    rd_midi::MidiMessage::NoteOn { note: pkt_note, velocity: 0, .. },
                ) if note.map_or(true, |n| n == *pkt_note) => (0.0, false),

                (
                    project::midi::MidiFilter::PitchBend,
                    rd_midi::MidiMessage::PitchBend { value, .. },
                ) => ((*value as f32 + 8192.0) / 16383.0, *value > 0),

                _ => continue,
            };

            let transform = &mapping.transform;
            if transform.invert {
                normalized_val = 1.0 - normalized_val;
                is_pressed = !is_pressed;
            }

            let final_value = transform.min_output
                + (normalized_val * (transform.max_output - transform.min_output));

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
