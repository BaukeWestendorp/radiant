use midir::MidiInput;

use crate::{ExecutorId, project, services::trigger::Trigger};

pub fn start_listener(
    stop_rx: flume::Receiver<()>,
    notify_tx: flume::Sender<Trigger>,
    config: &project::midi::MidiTriggerConfig,
) -> anyhow::Result<()> {
    // FIXME: Use stop_rx, and maybe create rd-midi?
    let scanner = MidiInput::new("rd_trigger_service").map_err(|e| anyhow::anyhow!(e))?;
    let mut connections = Vec::new();

    for port in scanner.ports() {
        let port_name = scanner.port_name(&port).unwrap_or_default();

        if let Some(device_config) = config.devices.iter().find(|d| port_name.contains(&d.name)) {
            let config_clone = device_config.clone();

            let midi_in = MidiInput::new(&port_name).map_err(|e| anyhow::anyhow!(e))?;

            let conn = midi_in
                .connect(
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
                )
                .map_err(|e| anyhow::anyhow!(e))?;

            connections.push(conn);
        }
    }

    Ok(())
}

fn parse_midi(message: &[u8], device_config: &project::midi::MidiDeviceConfig) -> Option<Trigger> {
    if message.len() < 3 {
        return None;
    }

    let status = message[0];
    let data1 = message[1];
    let data2 = message[2];

    let channel = status & 0x0F;
    let msg_type = status & 0xF0;

    if channel != device_config.channel {
        return None;
    }

    for mapping in &device_config.mappings {
        let is_match = match &mapping.msg {
            project::midi::MidiMessage::ControlChange { controller, .. } => {
                msg_type == 0xB0 && data1 == *controller
            }
            project::midi::MidiMessage::NoteOn { note } => msg_type == 0x90 && data1 == *note,
        };

        if is_match {
            let normalized_value = data2 as f32 / 127.0;

            return match &mapping.target {
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
                project::TriggerTarget::Encoder { encoder_ix } => Some(Trigger::EncoderSetValue {
                    encoder_ix: *encoder_ix,
                    value: normalized_value,
                }),
            };
        }
    }

    None
}
