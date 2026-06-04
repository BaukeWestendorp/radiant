use std::collections::HashSet;

use midir::MidiInputConnection;

use crate::object::{ExecutorButton, ExecutorId};

mod definition;

pub use definition::*;

pub struct Triggers {
    definition: TriggersDefinition,

    trigger_rx: crossbeam_channel::Receiver<Trigger>,

    // NOTE: These are stored here to keep them alive for as long as the resolver lives.
    _midi_connections:
        Vec<MidiInputConnection<(Vec<MidiTriggerDefinition>, crossbeam_channel::Sender<Trigger>)>>,
}

impl Triggers {
    pub fn new(definition: TriggersDefinition) -> anyhow::Result<Self> {
        let (trigger_tx, trigger_rx) = crossbeam_channel::bounded(512);

        let unique_midi_device_names: HashSet<_> =
            definition.midi().iter().map(|child| child.device_name()).collect();

        let mut midi_connections = Vec::new();
        for device_name in unique_midi_device_names {
            let midi = definition
                .midi()
                .iter()
                .filter(|midi_mapping| midi_mapping.device_name() == device_name)
                .cloned()
                .collect::<Vec<_>>();

            let midi_in = match midir::MidiInput::new("Radiant") {
                Ok(midi_in) => midi_in,
                Err(err) => {
                    log::error!("Failed to create a MIDI input: {err}");
                    continue;
                }
            };

            let port = match midi_in
                .ports()
                .into_iter()
                .find(|port| midi_in.port_name(port).as_deref().ok() == Some(device_name))
            {
                Some(port) => port,
                None => {
                    let available_ports = midi_in
                        .ports()
                        .into_iter()
                        .filter_map(|port| midi_in.port_name(&port).ok())
                        .collect::<Vec<_>>();

                    log::error!(
                        "midi port not found: {}. available ports: {:?}",
                        device_name,
                        available_ports
                    );

                    continue;
                }
            };

            let midi_connection = match midi_in.connect(
                &port,
                "Radiant",
                Self::handle_midi_event,
                (midi, trigger_tx.clone()),
            ) {
                Ok(midi_connection) => midi_connection,
                Err(err) => {
                    log::error!("failed to connect to midi port: {err}");
                    continue;
                }
            };

            midi_connections.push(midi_connection);
        }

        Ok(Self { definition, _midi_connections: midi_connections, trigger_rx })
    }

    pub fn definition(&self) -> &TriggersDefinition {
        &self.definition
    }

    /// Drains all pending triggers currently queued.
    ///
    /// This is the primary external API for retrieving triggers.
    pub fn drain(&self) -> Vec<Trigger> {
        self.trigger_rx.try_iter().collect()
    }

    fn handle_midi_event(
        _timestamp: u64,
        event_bytes: &[u8],
        (midi_mappings, trigger_tx): &mut (
            Vec<MidiTriggerDefinition>,
            crossbeam_channel::Sender<Trigger>,
        ),
    ) {
        let event = match midly::live::LiveEvent::parse(event_bytes) {
            Err(err) => {
                log::warn!("received invalid midi bytes {:?}: {}", event_bytes, err);
                return;
            }
            Ok(event) => event,
        };

        log::debug!("received midi event: {:?}", event);

        let midly::live::LiveEvent::Midi { channel, message } = event else { return };

        let triggers = midi_mappings
            .into_iter()
            .filter(|mapping| {
                let channels_match = mapping.channel().contains(&channel.into());
                let messages_match = match (mapping.trigger(), message) {
                    (
                        MidiMessage::NoteOff { note, velocity },
                        midly::MidiMessage::NoteOff { key, vel },
                    ) => note.contains(&key.into()) && velocity.contains(&vel.into()),
                    (
                        MidiMessage::NoteOn { note, velocity },
                        midly::MidiMessage::NoteOn { key, vel },
                    ) => note.contains(&key.into()) && velocity.contains(&vel.into()),
                    (
                        MidiMessage::PolyphonicAftertouch { note, pressure },
                        midly::MidiMessage::Aftertouch { key, vel },
                    ) => note.contains(&key.into()) && pressure.contains(&vel.into()),
                    (
                        MidiMessage::ControlChange { controller: ctrl1, value: val1 },
                        midly::MidiMessage::Controller { controller: ctrl2, value: val2 },
                    ) => ctrl1.contains(&ctrl2.into()) && val1.contains(&val2.into()),
                    (
                        MidiMessage::ProgramChange { program: prog1 },
                        midly::MidiMessage::ProgramChange { program: prog2 },
                    ) => prog1.contains(&prog2.into()),
                    (
                        MidiMessage::ChannelAftertouch { pressure },
                        midly::MidiMessage::ChannelAftertouch { vel },
                    ) => pressure.contains(&vel.into()),
                    (MidiMessage::PitchBend { value }, midly::MidiMessage::PitchBend { bend }) => {
                        value.contains(&bend.as_int())
                    }
                    _ => false,
                };

                let matched = channels_match && messages_match;
                matched
            })
            .filter_map(|mapping| {
                let target = mapping.target();
                match target {
                    TriggerTarget::ExecutorMaster { executor_id } => {
                        let Some(value) = midi_value_as_f32(&message) else {
                            log::warn!(
                                "mapped midi message did not carry a value usable as f32: {:?}",
                                message
                            );
                            return None;
                        };
                        Some(Trigger::ExecutorMaster { executor_id: *executor_id, value })
                    }
                    TriggerTarget::ExecutorButton { executor_id, button } => {
                        let pressed = midi_pressed(&message);
                        Some(Trigger::ExecutorButton {
                            executor_id: *executor_id,
                            button: *button,
                            pressed,
                        })
                    }
                }
            })
            .collect::<Vec<_>>();

        for trigger in triggers {
            log::debug!("sending trigger: {:?}", event);

            if let Err(err) = trigger_tx.send(trigger) {
                log::error!("failed to send trigger: {}", err);
            }
        }
    }
}

impl Default for Triggers {
    fn default() -> Self {
        let (_, trigger_rx) = crossbeam_channel::bounded(1);
        Self { definition: Default::default(), trigger_rx, _midi_connections: Default::default() }
    }
}

pub enum Trigger {
    ExecutorMaster { executor_id: ExecutorId, value: f32 },
    ExecutorButton { executor_id: ExecutorId, button: ExecutorButton, pressed: bool },
}

fn midi_pressed(message: &midly::MidiMessage) -> bool {
    match *message {
        midly::MidiMessage::NoteOn { .. } => true,
        midly::MidiMessage::NoteOff { .. } => false,
        midly::MidiMessage::Controller { value, .. }
        | midly::MidiMessage::Aftertouch { vel: value, .. }
        | midly::MidiMessage::ChannelAftertouch { vel: value } => value.as_int() > 0,
        midly::MidiMessage::ProgramChange { .. } => true,
        midly::MidiMessage::PitchBend { .. } => true,
    }
}

fn midi_value_as_f32(message: &midly::MidiMessage) -> Option<f32> {
    let v = match *message {
        midly::MidiMessage::NoteOff { vel, .. }
        | midly::MidiMessage::NoteOn { vel, .. }
        | midly::MidiMessage::Aftertouch { vel, .. }
        | midly::MidiMessage::ChannelAftertouch { vel }
        | midly::MidiMessage::Controller { value: vel, .. } => vel.as_int() as f32 / 127.0,
        midly::MidiMessage::ProgramChange { program } => program.as_int() as f32 / 127.0,
        midly::MidiMessage::PitchBend { bend } => {
            let v = bend.as_int() as f32;
            (v + 8192.0) / 16383.0
        }
    };

    Some(v.clamp(0.0, 1.0))
}
