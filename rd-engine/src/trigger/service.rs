use std::{
    collections::HashSet,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use midir::MidiInput;

use crate::trigger::{
    MidiMessage, MidiTriggerDefinition, Trigger, TriggerTarget, TriggersDefinition,
};

pub struct TriggerService {
    definition: TriggersDefinition,
    trigger_rx: flume::Receiver<Trigger>,
}

impl TriggerService {
    pub fn new(definition: TriggersDefinition) -> (Self, TriggerServiceRunner) {
        let (trigger_tx, trigger_rx) = flume::bounded(512);

        let delegate = Self { definition: definition.clone(), trigger_rx };

        let runner = TriggerServiceRunner::new(definition, trigger_tx);

        (delegate, runner)
    }

    pub fn definition(&self) -> &TriggersDefinition {
        &self.definition
    }

    pub fn drain(&self) -> Vec<Trigger> {
        self.trigger_rx.try_iter().collect()
    }
}

impl rd_service::Delegate for TriggerService {
    type Error = anyhow::Error;

    fn on_start(&self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn on_frame(&self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn on_stop(&self) -> Result<(), Self::Error> {
        Ok(())
    }
}

pub struct TriggerServiceRunner {
    definition: TriggersDefinition,
    trigger_tx: flume::Sender<Trigger>,
}

impl TriggerServiceRunner {
    pub fn new(definition: TriggersDefinition, trigger_tx: flume::Sender<Trigger>) -> Self {
        Self { definition, trigger_tx }
    }
}

impl rd_service::Runner for TriggerServiceRunner {
    fn start(
        &mut self,
        running: Arc<AtomicBool>,
        notify_tx: flume::Sender<()>,
    ) -> rd_service::Result<()> {
        log::info!("Initializing MIDI...");

        let unique_midi_device_names: HashSet<_> =
            self.definition.midi().iter().map(|child| child.device_name()).collect();

        // NOTE: These are stored here to keep them alive for as long as the service runs.
        let mut connections = Vec::new();

        for device_name in unique_midi_device_names {
            let midi_mappings = self
                .definition
                .midi()
                .iter()
                .filter(|mapping| mapping.device_name() == device_name)
                .cloned()
                .collect::<Vec<_>>();

            let midi_in = match MidiInput::new("Radiant") {
                Ok(midi_in) => midi_in,
                Err(err) => {
                    log::error!("Failed to create a MIDI input: {err}");
                    continue;
                }
            };

            let ports = midi_in.ports();
            let port = ports
                .iter()
                .find(|port| midi_in.port_name(port).as_deref().ok() == Some(device_name));

            let Some(port) = port else {
                let available_ports = ports
                    .iter()
                    .filter_map(|port| midi_in.port_name(port).ok())
                    .collect::<Vec<_>>();

                log::error!(
                    "MIDI port not found: {}. available ports: {:?}",
                    device_name,
                    available_ports
                );
                continue;
            };

            let trigger_tx = self.trigger_tx.clone();
            let notify_tx = notify_tx.clone();

            let connection = midi_in.connect(
                port,
                "Radiant",
                move |_timestamp, event_bytes, _| {
                    handle_midi_event(event_bytes, &midi_mappings, &trigger_tx, &notify_tx);
                },
                (),
            );

            match connection {
                Ok(conn) => connections.push(conn),
                Err(err) => log::error!("Failed to connect to MIDI port: {err}"),
            }
        }

        log::info!("MIDI Initialized");

        // FIXME: Do we really need this?
        while running.load(Ordering::SeqCst) {
            std::thread::sleep(Duration::from_millis(50));
        }

        log::info!("MIDI connections closed.");

        Ok(())
    }
}

fn handle_midi_event(
    event_bytes: &[u8],
    mappings: &[MidiTriggerDefinition],
    trigger_tx: &flume::Sender<Trigger>,
    notify_tx: &flume::Sender<()>,
) {
    let event = match midly::live::LiveEvent::parse(event_bytes) {
        Ok(event) => event,
        Err(err) => {
            log::warn!("Received invalid MIDI bytes {:?}: {}", event_bytes, err);
            return;
        }
    };

    let midly::live::LiveEvent::Midi { channel, message } = event else {
        return;
    };

    let mut triggered = false;

    let triggers = mappings
        .iter()
        .filter(|mapping| match_midi_message(mapping, channel, &message))
        .filter_map(|mapping| build_trigger(mapping.target(), &message));

    for trigger in triggers {
        if let Err(err) = trigger_tx.try_send(trigger) {
            log::error!("Failed to send trigger: {}", err);
        } else {
            triggered = true;
        }
    }

    if triggered {
        let _ = notify_tx.try_send(());
    }
}

fn match_midi_message(
    mapping: &MidiTriggerDefinition,
    channel: midly::num::u4,
    message: &midly::MidiMessage,
) -> bool {
    if !mapping.channel().contains(&channel.into()) {
        return false;
    }

    match (mapping.trigger(), message) {
        (MidiMessage::NoteOff { note, velocity }, midly::MidiMessage::NoteOff { key, vel })
        | (MidiMessage::NoteOn { note, velocity }, midly::MidiMessage::NoteOn { key, vel }) => {
            note.contains(&(*key).into()) && velocity.contains(&(*vel).into())
        }
        (
            MidiMessage::PolyphonicAftertouch { note, pressure },
            midly::MidiMessage::Aftertouch { key, vel },
        ) => note.contains(&(*key).into()) && pressure.contains(&(*vel).into()),
        (
            MidiMessage::ControlChange { controller, value },
            midly::MidiMessage::Controller { controller: ctrl, value: val },
        ) => controller.contains(&(*ctrl).into()) && value.contains(&(*val).into()),
        (
            MidiMessage::ProgramChange { program },
            midly::MidiMessage::ProgramChange { program: prog },
        ) => program.contains(&(*prog).into()),
        (
            MidiMessage::ChannelAftertouch { pressure },
            midly::MidiMessage::ChannelAftertouch { vel },
        ) => pressure.contains(&(*vel).into()),
        (MidiMessage::PitchBend { value }, midly::MidiMessage::PitchBend { bend }) => {
            value.contains(&(*bend).as_int())
        }
        _ => false,
    }
}

fn build_trigger(target: &TriggerTarget, message: &midly::MidiMessage) -> Option<Trigger> {
    match target {
        TriggerTarget::ExecutorMaster { executor_id } => midi_value_as_f32(message)
            .map(|value| Trigger::ExecutorMaster { executor_id: *executor_id, value }),
        TriggerTarget::ExecutorButton { executor_id, button } => Some(Trigger::ExecutorButton {
            executor_id: *executor_id,
            button: *button,
            pressed: midi_pressed(message),
        }),
        TriggerTarget::Encoder { encoder_ix } => midi_value_as_f32(message)
            .map(|value| Trigger::EncoderSetValue { encoder_ix: *encoder_ix, value }),
    }
}

fn midi_pressed(message: &midly::MidiMessage) -> bool {
    match *message {
        midly::MidiMessage::NoteOn { .. }
        | midly::MidiMessage::ProgramChange { .. }
        | midly::MidiMessage::PitchBend { .. } => true,
        midly::MidiMessage::NoteOff { .. } => false,
        midly::MidiMessage::Controller { value, .. }
        | midly::MidiMessage::Aftertouch { vel: value, .. }
        | midly::MidiMessage::ChannelAftertouch { vel: value } => value.as_int() > 0,
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
