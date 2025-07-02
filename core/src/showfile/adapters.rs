use std::{collections::HashMap, fs, io, path::PathBuf};

use eyre::Context;

use crate::{ExecutorId, Result};

#[derive(Default)]
#[derive(serde::Deserialize)]
pub struct Adapters {
    midi: MidiConfiguration,
}

impl Adapters {
    pub fn midi(&self) -> &MidiConfiguration {
        &self.midi
    }

    /// Reads the adapters configuration from a file at the given path.
    pub fn read_from_file(path: &PathBuf) -> Result<Self> {
        let file = fs::File::open(path)
            .with_context(|| format!("failed to open adapters file at '{}'", path.display()))?;
        let reader = io::BufReader::new(file);
        serde_yaml::from_reader(reader)
            .with_context(|| format!("failed to read adapters file at '{}'", path.display()))
    }
}

#[derive(Clone, Default)]
#[derive(serde::Deserialize)]
pub struct MidiConfiguration {
    active_devices: Vec<String>,
    actions: MidiActions,
}

impl MidiConfiguration {
    pub fn active_devices(&self) -> &[String] {
        &self.active_devices
    }

    pub fn actions(&self) -> &MidiActions {
        &self.actions
    }
}

#[derive(Clone, Default)]
#[derive(serde::Deserialize)]
pub struct MidiActions {
    executors: HashMap<ExecutorId, MidiExecutorAction>,
}

impl MidiActions {
    pub fn executors(&self) -> &HashMap<ExecutorId, MidiExecutorAction> {
        &self.executors
    }
}

#[derive(Debug, Clone, Copy)]
#[derive(serde::Deserialize)]
pub struct MidiExecutorAction {
    button: Option<MidiExecutorControl>,
    fader: Option<MidiExecutorControl>,
}

impl MidiExecutorAction {
    pub fn button(&self) -> Option<&MidiExecutorControl> {
        self.button.as_ref()
    }

    pub fn fader(&self) -> Option<&MidiExecutorControl> {
        self.fader.as_ref()
    }
}

#[derive(Debug, Clone, Copy)]
#[derive(serde::Deserialize)]
pub struct MidiExecutorControl {
    channel: u8,
    msg: MidiAction,
}

impl MidiExecutorControl {
    pub fn channel(&self) -> u8 {
        self.channel
    }

    pub fn msg(&self) -> MidiAction {
        self.msg
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(serde::Deserialize)]
pub enum MidiAction {
    #[serde(rename = "note")]
    Note(u8),
    #[serde(rename = "cc")]
    ControlChange(u8),
}
