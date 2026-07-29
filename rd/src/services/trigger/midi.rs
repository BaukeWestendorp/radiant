use rd_midi::{MidiMessage, MidiPacket, u4, u7};

use crate::{
    ExecutorId,
    project::{
        self,
        midi::{MidiChannel, MidiController, MidiFilter, MidiNote},
    },
    services::trigger::Trigger,
};

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

    fn matches_channel(filter_channel: &MidiChannel, packet_channel: u4) -> bool {
        match filter_channel {
            MidiChannel::All => true,
            MidiChannel::Single(channel) => channel.get() == packet_channel.get(),
        }
    }

    fn matches_note(filter_note: &MidiNote, packet_note: u7) -> bool {
        match filter_note {
            MidiNote::All => true,
            MidiNote::Single(note) => note.get() == packet_note.get(),
        }
    }

    fn matches_controller(filter_controller: &MidiController, packet_controller: u7) -> bool {
        match filter_controller {
            MidiController::All => true,
            MidiController::Single(controller) => controller.get() == packet_controller.get(),
        }
    }

    fn parse_trigger(&self, packet: MidiPacket) -> Option<Trigger> {
        let msg_channel = match &packet.message {
            MidiMessage::ControlChange { channel, .. }
            | MidiMessage::NoteOn { channel, .. }
            | MidiMessage::NoteOff { channel, .. }
            | MidiMessage::PitchBend { channel, .. } => *channel,
            _ => return None,
        };

        self.mappings.iter().find_map(|mapping| {
            if mapping.device_name != packet.port_name {
                return None;
            }

            if !Self::matches_channel(&mapping.device_channel, msg_channel) {
                return None;
            }

            let (normalized_val, is_pressed) = match (&mapping.filter, &packet.message) {
                (
                    MidiFilter::ControlChange { controller },
                    MidiMessage::ControlChange { controller: pkt_ctrl, value, .. },
                ) if Self::matches_controller(controller, *pkt_ctrl) => {
                    (value.get() as f32 / 127.0, value.get() > 0)
                }

                (
                    MidiFilter::NoteOn { note },
                    MidiMessage::NoteOn { note: pkt_note, velocity, .. },
                ) if Self::matches_note(note, *pkt_note) => {
                    (velocity.get() as f32 / 127.0, velocity.get() > 0)
                }

                (MidiFilter::NoteOff { note }, MidiMessage::NoteOff { note: pkt_note, .. })
                    if Self::matches_note(note, *pkt_note) =>
                {
                    (0.0, false)
                }

                (
                    MidiFilter::NoteOff { note },
                    MidiMessage::NoteOn { note: pkt_note, velocity, .. },
                ) if Self::matches_note(note, *pkt_note) && velocity.get() == 0 => (0.0, false),

                (MidiFilter::PitchBend, MidiMessage::PitchBend { value, .. }) => {
                    ((*value as f32 + 8192.0) / 16383.0, *value > 0)
                }

                _ => return None,
            };

            match &mapping.target {
                project::TriggerTarget::HighlightToggle => {
                    is_pressed.then_some(Trigger::ToggleHighlight)
                }
                project::TriggerTarget::ExecutorMaster { page_id, slot } => {
                    Some(Trigger::ExecutorMaster {
                        executor_id: ExecutorId::new(*page_id, *slot),
                        value: normalized_val,
                    })
                }
                project::TriggerTarget::ExecutorButton { page_id, slot, button } => {
                    Some(Trigger::ExecutorButton {
                        executor_id: ExecutorId::new(*page_id, *slot),
                        button: *button,
                        pressed: is_pressed,
                    })
                }
                project::TriggerTarget::Encoder { encoder_ix } => Some(Trigger::EncoderSetValue {
                    encoder_ix: *encoder_ix,
                    value: normalized_val,
                }),
            }
        })
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
