use std::{collections::HashSet, sync::mpsc};

use midir::MidiInputConnection;

use crate::{
    Command, ExecutorButton, ExecutorId, MidiTriggerMapping, TriggerConfig, TriggerTarget,
};

pub enum Trigger {
    ExecutorMaster { executor_id: ExecutorId, value: f32 },
    ExecutorButton { executor_id: ExecutorId, button: ExecutorButton, pressed: bool },
}

pub struct TriggerResolver {
    trigger_rx: mpsc::Receiver<Trigger>,

    // NOTE: These are stored here to keep them alive for as long as the resolver lives.
    _midi_connections:
        Vec<MidiInputConnection<(Vec<MidiTriggerMapping>, mpsc::SyncSender<Trigger>)>>,
}

impl TriggerResolver {
    pub fn new(config: &TriggerConfig) -> anyhow::Result<Self> {
        let (trigger_tx, trigger_rx) = mpsc::sync_channel(512);

        let unique_midi_device_names: HashSet<_> =
            config.midi_mappings().iter().map(|child| child.device_name()).collect();

        let mut midi_connections = Vec::new();
        for device_name in unique_midi_device_names {
            let midi_mappings = config
                .midi_mappings()
                .iter()
                .filter(|mapping| mapping.device_name() == device_name)
                .cloned()
                .collect::<Vec<_>>();

            let midi_in = midir::MidiInput::new("Radiant")?;
            let port = midi_in
                .ports()
                .into_iter()
                .find(|port| midi_in.port_name(port).as_deref().ok() == Some(device_name))
                .ok_or_else(|| {
                    let available_ports = midi_in
                        .ports()
                        .into_iter()
                        .filter_map(|port| midi_in.port_name(&port).ok())
                        .collect::<Vec<_>>();

                    anyhow::anyhow!(
                        "midi port not found: {}. available ports: {:?}",
                        device_name,
                        available_ports
                    )
                })?;
            let midi_connection = midi_in.connect(
                &port,
                "Radiant",
                Self::handle_midi_event,
                (midi_mappings, trigger_tx.clone()),
            )?;
            midi_connections.push(midi_connection);
        }

        Ok(Self { _midi_connections: midi_connections, trigger_rx })
    }

    pub fn resolve(&mut self) -> anyhow::Result<Vec<Command>> {
        let mut commands = Vec::new();
        while let Ok(trigger) = self.trigger_rx.try_recv() {
            match trigger {
                Trigger::ExecutorMaster { executor_id, value } => {
                    commands.push(Command::ExecutorSetMaster { executor_id, value });
                }
                Trigger::ExecutorButton { executor_id, button, pressed } => {
                    commands.push(Command::ExecutorButton { executor_id, button, pressed });
                }
            }
        }
        Ok(commands)
    }

    fn handle_midi_event(
        _timestamp: u64,
        event_bytes: &[u8],
        (midi_mappings, trigger_tx): &mut (Vec<MidiTriggerMapping>, mpsc::SyncSender<Trigger>),
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

        let mut any_mapping_matched = false;
        let triggers = midi_mappings
            .into_iter()
            .filter(|mapping| {
                let channels_match = mapping.channel().contains(&channel.into());
                let messages_match = match (mapping.trigger(), message) {
                    (
                        crate::MidiMessage::NoteOff { note, velocity },
                        midly::MidiMessage::NoteOff { key, vel },
                    ) => note.contains(&key.into()) && velocity.contains(&vel.into()),
                    (
                        crate::MidiMessage::NoteOn { note, velocity },
                        midly::MidiMessage::NoteOn { key, vel },
                    ) => note.contains(&key.into()) && velocity.contains(&vel.into()),
                    (
                        crate::MidiMessage::PolyphonicAftertouch { note, pressure },
                        midly::MidiMessage::Aftertouch { key, vel },
                    ) => note.contains(&key.into()) && pressure.contains(&vel.into()),
                    (
                        crate::MidiMessage::ControlChange { controller: ctrl1, value: val1 },
                        midly::MidiMessage::Controller { controller: ctrl2, value: val2 },
                    ) => ctrl1.contains(&ctrl2.into()) && val1.contains(&val2.into()),
                    (
                        crate::MidiMessage::ProgramChange { program: prog1 },
                        midly::MidiMessage::ProgramChange { program: prog2 },
                    ) => prog1.contains(&prog2.into()),
                    (
                        crate::MidiMessage::ChannelAftertouch { pressure },
                        midly::MidiMessage::ChannelAftertouch { vel },
                    ) => pressure.contains(&vel.into()),
                    (
                        crate::MidiMessage::PitchBend { value },
                        midly::MidiMessage::PitchBend { bend },
                    ) => value.contains(&bend.as_int()),
                    _ => false,
                };

                let matched = channels_match && messages_match;
                any_mapping_matched |= matched;
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

        if !any_mapping_matched {
            log::warn!("received unmapped midi message on channel {}: {:?}", channel, message);
        }

        for trigger in triggers {
            log::debug!("sending trigger: {:?}", event);

            if let Err(err) = trigger_tx.send(trigger) {
                log::error!("failed to send trigger: {}", err);
            }
        }
    }
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
