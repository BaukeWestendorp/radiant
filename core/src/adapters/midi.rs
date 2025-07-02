use crate::{
    ExecutorId, Result,
    showfile::{MidiAction, MidiConfiguration},
};
use midir::{MidiInput, MidiInputConnection};
use std::sync::mpsc;

pub struct MidiAdapter {
    // Needs to be kept alive.
    _connections: Vec<MidiInputConnection<()>>,
}

impl MidiAdapter {
    pub fn new<'id>(config: &MidiConfiguration, tx: mpsc::Sender<MidiCommand>) -> Result<Self> {
        let midi_input = MidiInput::new("Radiant MIDI Input").unwrap();

        let in_ports = midi_input.ports();
        log::debug!(
            "available midi port ids: {:?}",
            midi_input.ports().iter().map(|port| port.id()).collect::<Vec<_>>()
        );
        let ids = config.active_devices().iter().map(Into::into).collect::<Vec<_>>();
        let ports = in_ports.iter().filter(|port| ids.contains(&&port.id()));

        let mut connections = Vec::new();
        for port in ports {
            let midi_input = MidiInput::new("Radiant MIDI Input").unwrap();

            let port_name = midi_input.port_name(port)?;
            log::info!("using MIDI port '{port_name}'");

            let config = config.clone();
            let tx = tx.clone();
            let connection = midi_input.connect(
                port,
                &format!("Radiant MIDI Input ({})", port_name),
                move |_stamp, message, _| {
                    let midi_cmds = get_midi_commands(message, &config);
                    for midi_cmd in midi_cmds {
                        tx.send(midi_cmd)
                            .map_err(|err| {
                                log::error!("failed to send MIDI command from MIDI adapter: {err}");
                            })
                            .ok();
                    }
                },
                (),
            )?;

            connections.push(connection);
        }

        Ok(Self { _connections: connections })
    }
}

fn get_midi_commands(message: &[u8], config: &MidiConfiguration) -> Vec<MidiCommand> {
    const NOTE_ON: u8 = 0x90;
    const NOTE_OFF: u8 = 0x80;
    const CC: u8 = 0xB0;

    if message.len() < 3 {
        return Vec::new();
    }

    let status = message[0] & 0xF0;
    let channel = message[0] & 0x0F;
    let data1 = message[1];
    let data2 = message[2];

    let mut commands = Vec::new();
    for (executor_id, action) in config.actions().executors() {
        let executor_id = *executor_id;

        if let Some(button) = action.button()
            && button.channel() == channel
        {
            match button.msg() {
                MidiAction::Note(msg) => {
                    if msg == data1 {
                        let cmd = match status {
                            NOTE_ON => Some(MidiCommand::ExecutorButtonPress { executor_id }),
                            NOTE_OFF => Some(MidiCommand::ExecutorButtonRelease { executor_id }),
                            _ => None,
                        };
                        commands.extend(cmd);
                    };
                }
                MidiAction::ControlChange(msg) => {
                    if msg == data1 {
                        let cmd = match status {
                            CC => Some(match data2 > 63 {
                                true => MidiCommand::ExecutorButtonPress { executor_id },
                                false => MidiCommand::ExecutorButtonRelease { executor_id },
                            }),
                            _ => None,
                        };
                        commands.extend(cmd);
                    };
                }
            }
        }

        if let Some(fader) = action.fader()
            && fader.channel() == channel
        {
            match fader.msg() {
                MidiAction::Note(msg) => {
                    if msg == data1 {
                        let cmd = match status {
                            NOTE_ON => {
                                Some(MidiCommand::ExecutorFaderSetValue { executor_id, value: 1.0 })
                            }
                            NOTE_OFF => {
                                Some(MidiCommand::ExecutorFaderSetValue { executor_id, value: 0.0 })
                            }
                            _ => None,
                        };
                        commands.extend(cmd);
                    };
                }
                MidiAction::ControlChange(msg) => {
                    if msg == data1 {
                        let value = data2 as f32 / 127.0;
                        let cmd = match status {
                            CC => Some(MidiCommand::ExecutorFaderSetValue { executor_id, value }),
                            _ => None,
                        };
                        commands.extend(cmd);
                    };
                }
            }
        }
    }

    commands
}

#[derive(Debug, Clone, Copy)]
pub enum MidiCommand {
    ExecutorButtonPress { executor_id: ExecutorId },
    ExecutorButtonRelease { executor_id: ExecutorId },
    ExecutorFaderSetValue { executor_id: ExecutorId, value: f32 },
}
