use std::time::{Duration, Instant};

use crate::{MidiMessage, MidiPacket};

pub struct MidiInputServiceRunner {
    midi_input: midir::MidiInput,
}

impl MidiInputServiceRunner {
    pub fn new() -> crate::Result<Self> {
        let midi_input = midir::MidiInput::new(&format!("{}-scanner", env!("CARGO_CRATE_NAME")))?;
        Ok(Self { midi_input })
    }
}

impl rd_service::Runner for MidiInputServiceRunner {
    type Error = crate::Error;
    type Data = MidiPacket;

    fn start(
        &mut self,
        stop_rx: flume::Receiver<()>,
        notify_tx: flume::Sender<Self::Data>,
    ) -> Result<(), Self::Error> {
        let mut connections = Vec::new();

        for port in self.midi_input.ports() {
            let port_name = self.midi_input.port_name(&port)?;
            let port_id = port.id();

            let midi_input = midir::MidiInput::new(&port_name)?;

            let connection = midi_input.connect(
                &port,
                &port_name,
                {
                    let notify_tx = notify_tx.clone();
                    let port_name = port_name.clone();
                    let port_id = port_id.clone();

                    let mut base_time = None;

                    move |ts_micros, message, _| {
                        let message = match MidiMessage::decode(message) {
                            Ok(msg) => msg,
                            Err(err) => {
                                eprintln!(
                                    "Failed to decode MIDI message from port {}: {:?}",
                                    port_name, err
                                );
                                return;
                            }
                        };

                        let (base_micros, base_instant) =
                            *base_time.get_or_insert_with(|| (ts_micros, Instant::now()));

                        let timestamp = if ts_micros >= base_micros {
                            base_instant + Duration::from_micros(ts_micros - base_micros)
                        } else {
                            base_instant - Duration::from_micros(base_micros - ts_micros)
                        };

                        let _ = notify_tx.send(MidiPacket {
                            port_name: port_name.clone(),
                            port_id: port_id.clone(),
                            timestamp,
                            message,
                        });
                    }
                },
                (),
            )?;

            connections.push(connection);
        }

        let _ = stop_rx.recv();

        Ok(())
    }
}
