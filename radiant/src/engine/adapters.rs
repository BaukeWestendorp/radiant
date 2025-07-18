use std::sync::mpsc::{self};

use crate::adapters::midi::{MidiAdapter, MidiCommand};
use crate::cmd::{Command, ExecutorCommand, ObjectCommand};
use crate::error::Result;
use crate::showfile::MidiConfiguration;

pub struct Adapters {
    /// Receives MIDI commands from the MIDI adapter.
    rx: mpsc::Receiver<MidiCommand>,

    // Needs to be kept alive.
    _midi_adapter: MidiAdapter,
}

impl Adapters {
    pub fn new(config: &MidiConfiguration) -> Result<Self> {
        let (tx, rx) = mpsc::channel();
        let midi_adapter = MidiAdapter::new(config, tx)?;
        Ok(Self { _midi_adapter: midi_adapter, rx })
    }

    pub fn handle_input(&self) -> Result<Vec<Command>> {
        let mut commands = Vec::new();

        for midi_message in self.rx.try_iter().collect::<Vec<_>>() {
            let cmd = match midi_message {
                MidiCommand::ExecutorButtonPress { executor_id } => Command::Object(
                    ObjectCommand::Executor(executor_id, ExecutorCommand::ButtonPress),
                ),
                MidiCommand::ExecutorButtonRelease { executor_id } => Command::Object(
                    ObjectCommand::Executor(executor_id, ExecutorCommand::ButtonRelease),
                ),
                MidiCommand::ExecutorFaderSetLevel { executor_id, level } => Command::Object(
                    ObjectCommand::Executor(executor_id, ExecutorCommand::FaderSetLevel { level }),
                ),
            };
            commands.push(cmd);
        }

        Ok(commands)
    }
}
