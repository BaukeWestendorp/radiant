use crate::{Result, showfile::MidiConfig};
use midir::{MidiInput, MidiInputConnection};
use std::sync::mpsc;

pub struct MidiAdapter {
    // Needs to be kept alive.
    _connections: Vec<MidiInputConnection<()>>,
}

impl MidiAdapter {
    pub fn new<'id>(config: &MidiConfig, midi_tx: mpsc::Sender<()>) -> Result<Self> {
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

            let midi_tx = midi_tx.clone();
            let connection = midi_input.connect(
                port,
                &format!("Radiant MIDI Input ({})", port_name),
                move |_stamp, _message, _| {
                    dbg!(_message);
                    let midi_cmd = ();
                    midi_tx
                        .send(midi_cmd)
                        .map_err(|err| {
                            log::error!("failed to send MIDI command from MIDI adapter: {err}");
                        })
                        .ok();
                },
                (),
            )?;

            connections.push(connection);
        }

        Ok(Self { _connections: connections })
    }
}
