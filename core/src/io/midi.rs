use eyre::ContextCompat;
use midir::{MidiInput, MidiInputConnection};

use crate::Result;

pub struct MidiManager {
    connection: MidiInputConnection<()>,
}

impl MidiManager {
    pub fn new(port_id: &str) -> Result<Self> {
        let midi_input = MidiInput::new("Radiant MIDI Input").unwrap();

        let in_ports = midi_input.ports();
        log::debug!(
            "available midi port ids: {:?}",
            midi_input.ports().iter().map(|port| port.id()).collect::<Vec<_>>()
        );
        let port = in_ports
            .iter()
            .find(|port| port.id() == port_id)
            .wrap_err_with(|| format!("MIDI port with name '{port_id}' not found"))?;

        let port_name = midi_input.port_name(port)?;
        log::info!("using MIDI port '{port_name}'");

        let connection = midi_input.connect(
            port,
            "Radiant MIDI Input",
            move |stamp, message, _| {
                println!("{}: {:?} (len = {})", stamp, message, message.len());
            },
            (),
        )?;

        Ok(Self { connection })
    }
}
