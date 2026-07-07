use std::collections::HashSet;
use std::sync::Mutex;

use midir::MidiInputConnection;

use crate::service::ServiceDelegate;
use crate::trigger::{
    MidiMessage, MidiTriggerDefinition, Trigger, TriggerTarget, TriggersDefinition,
};

pub struct TriggersService {
    definition: TriggersDefinition,
    trigger_rx: flume::Receiver<Trigger>,
    trigger_tx: flume::Sender<Trigger>,

    // NOTE: These are stored here to keep them alive for as long as the resolver lives.
    midi_connections: Mutex<
        Vec<
            MidiInputConnection<(
                Vec<MidiTriggerDefinition>,
                flume::Sender<Trigger>,
                flume::Sender<()>,
            )>,
        >,
    >,
}

impl TriggersService {
    pub fn new(definition: TriggersDefinition) -> anyhow::Result<Self> {
        let (trigger_tx, trigger_rx) = flume::bounded(512);

        Ok(Self { definition, trigger_rx, trigger_tx, midi_connections: Mutex::new(Vec::new()) })
    }

    pub fn definition(&self) -> &TriggersDefinition {
        &self.definition
    }

    pub fn drain(&self) -> Vec<Trigger> {
        self.trigger_rx.try_iter().collect()
    }

    fn handle_midi_event(
        _timestamp: u64,
        event_bytes: &[u8],
        (midi_mappings, trigger_tx, tick_tx): &mut (
            Vec<MidiTriggerDefinition>,
            flume::Sender<Trigger>,
            flume::Sender<()>,
        ),
    ) {
        let event = match midly::live::LiveEvent::parse(event_bytes) {
            Err(err) => {
                log::warn!("Received invalid MIDI bytes {:?}: {}", event_bytes, err);
                return;
            }
            Ok(event) => event,
        };

        log::debug!("Received MIDI event: {:?}", event);

        let midly::live::LiveEvent::Midi { channel, message } = event else {
            return;
        };

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
                                "Mapped MIDI message did not carry a value usable as f32: {:?}",
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
                    TriggerTarget::Encoder { encoder_ix } => {
                        let Some(value) = midi_value_as_f32(&message) else {
                            log::warn!(
                                "Mapped MIDI message did not contain a value usable as f32: {:?}",
                                message
                            );
                            return None;
                        };
                        Some(Trigger::EncoderSetValue { encoder_ix: *encoder_ix, value })
                    }
                }
            })
            .collect::<Vec<_>>();

        let mut triggered = false;
        for trigger in triggers {
            log::debug!("Sending trigger: {:?}", event);

            if let Err(err) = trigger_tx.send(trigger) {
                log::error!("Failed to send trigger: {}", err);
            } else {
                triggered = true;
            }
        }

        if triggered {
            let _ = tick_tx.try_send(());
        }
    }
}

impl ServiceDelegate for TriggersService {
    fn on_start(&self, tick_tx: flume::Sender<()>) -> anyhow::Result<()> {
        log::info!("Initializing MIDI...");

        let unique_midi_device_names: HashSet<_> =
            self.definition.midi().iter().map(|child| child.device_name()).collect();

        let mut local_connections = Vec::new();

        for device_name in unique_midi_device_names {
            let midi = self
                .definition
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
                        "MIDI port not found: {}. available ports: {:?}",
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
                (midi, self.trigger_tx.clone(), tick_tx.clone()),
            ) {
                Ok(midi_connection) => midi_connection,
                Err(err) => {
                    log::error!("Failed to connect to MIDI port: {err}");
                    continue;
                }
            };

            local_connections.push(midi_connection);
        }

        match self.midi_connections.lock() {
            Ok(mut guard) => *guard = local_connections,
            Err(err) => log::error!("Failed to lock MIDI connections guard: {err}"),
        }

        log::info!("MIDI Initialized");
        Ok(())
    }

    fn on_tick(&self) -> anyhow::Result<()> {
        Ok(())
    }

    fn on_stop(&self) -> anyhow::Result<()> {
        match self.midi_connections.lock() {
            Ok(mut guard) => guard.clear(),
            Err(err) => log::error!("Failed to lock MIDI connections guard on stop: {err}"),
        }
        Ok(())
    }

    fn name(&self) -> &'static str {
        "Triggers"
    }
}

impl Default for TriggersService {
    fn default() -> Self {
        Self::new(TriggersDefinition::default()).unwrap()
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
