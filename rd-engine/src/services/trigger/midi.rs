use midir::MidiInput;

use crate::{ExecutorId, project, services::trigger::Trigger};

const MIDI_CHANNEL_MASK: u8 = 0x0F;
const MIDI_STATUS_MASK: u8 = 0xF0;
const MIDI_CC_STATUS: u8 = 0xB0;
const MIDI_NOTE_ON_STATUS: u8 = 0x90;

pub fn start_listener(
    stop_rx: flume::Receiver<()>,
    notify_tx: flume::Sender<Trigger>,
    config: &project::midi::MidiTriggerConfig,
) -> anyhow::Result<()> {
    let scanner = MidiInput::new("rd_trigger_service")?;
    let mut connections = Vec::new();

    for port in scanner.ports() {
        let port_name = scanner.port_name(&port).unwrap_or_default();

        if let Some(device_config) = config.devices.iter().find(|d| port_name.contains(&d.name)) {
            let config_clone = device_config.clone();

            let midi_in = MidiInput::new(&port_name)?;

            let conn = midi_in.connect(
                &port,
                &port_name,
                {
                    let notify_tx = notify_tx.clone();
                    move |_timestamp, message, _| {
                        if let Some(trigger) = parse_midi(message, &config_clone) {
                            let _ = notify_tx.send(trigger);
                        }
                    }
                },
                (),
            )?;

            connections.push(conn);
        }
    }

    let _ = stop_rx.recv();

    Ok(())
}

fn parse_midi(message: &[u8], device_config: &project::midi::MidiDeviceConfig) -> Option<Trigger> {
    let &[status, data1, data2, ..] = message else {
        return None;
    };

    let channel = status & MIDI_CHANNEL_MASK;
    let msg_type = status & MIDI_STATUS_MASK;

    if channel != device_config.channel {
        return None;
    }

    device_config.mappings.iter().find_map(|mapping| {
        let is_match = match &mapping.msg {
            project::midi::MidiMessage::ControlChange { controller, .. } => {
                msg_type == MIDI_CC_STATUS && data1 == *controller
            }
            project::midi::MidiMessage::NoteOn { note } => {
                msg_type == MIDI_NOTE_ON_STATUS && data1 == *note
            }
        };

        if !is_match {
            return None;
        }

        let normalized_value = data2 as f32 / 127.0;

        match &mapping.target {
            project::TriggerTarget::HighlightToggle => Some(Trigger::ToggleHighlight),
            project::TriggerTarget::ExecutorMaster { page_id, slot } => {
                Some(Trigger::ExecutorMaster {
                    executor_id: ExecutorId::new(*page_id, *slot),
                    value: normalized_value,
                })
            }
            project::TriggerTarget::ExecutorButton { page_id, slot, button } => {
                Some(Trigger::ExecutorButton {
                    executor_id: ExecutorId::new(*page_id, *slot),
                    button: *button,
                    pressed: data2 > 0,
                })
            }
            project::TriggerTarget::Encoder { encoder_ix } => {
                Some(Trigger::EncoderSetValue { encoder_ix: *encoder_ix, value: normalized_value })
            }
        }
    })
}
