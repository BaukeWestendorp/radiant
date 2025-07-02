use std::{collections::HashMap, fs, io, path::PathBuf};

use eyre::Context;

use crate::{ExecutorId, Result};

#[derive(Default)]
#[derive(serde::Deserialize)]
pub struct Adapters {
    midi: MidiConfig,
}

impl Adapters {
    pub fn midi(&self) -> &MidiConfig {
        &self.midi
    }

    /// Reads a io from a file at the given path.
    pub fn read_from_file(path: &PathBuf) -> Result<Self> {
        let file = fs::File::open(path)
            .with_context(|| format!("failed to open adapters file at '{}'", path.display()))?;
        let reader = io::BufReader::new(file);
        serde_yaml::from_reader(reader)
            .with_context(|| format!("failed to read adapters file at '{}'", path.display()))
    }
}

#[derive(Default)]
#[derive(serde::Deserialize)]
pub struct MidiConfig {
    active_devices: Vec<String>,
    actions: MidiActions,
}

impl MidiConfig {
    pub fn active_devices(&self) -> &[String] {
        &self.active_devices
    }

    pub fn actions(&self) -> &MidiActions {
        &self.actions
    }
}

#[derive(Default)]
#[derive(serde::Deserialize)]
pub struct MidiActions {
    executors: HashMap<ExecutorId, MidiExecutorAction>,
}

impl MidiActions {
    pub fn executors(&self) -> &HashMap<ExecutorId, MidiExecutorAction> {
        &self.executors
    }
}

#[derive(serde::Deserialize)]
pub struct MidiExecutorAction {
    button: MidiExecutorButtonAction,
    fader: MidiExecutorFaderAction,
}

impl MidiExecutorAction {
    pub fn button(&self) -> &MidiExecutorButtonAction {
        &self.button
    }

    pub fn fader(&self) -> &MidiExecutorFaderAction {
        &self.fader
    }
}

#[derive(serde::Deserialize)]
pub struct MidiExecutorButtonAction {
    channel: u8,
    note: u8,
}

impl MidiExecutorButtonAction {
    pub fn channel(&self) -> u8 {
        self.channel
    }

    pub fn note(&self) -> u8 {
        self.note
    }
}

#[derive(serde::Deserialize)]
pub struct MidiExecutorFaderAction {
    channel: u8,
    cc: u8,
}

impl MidiExecutorFaderAction {
    pub fn channel(&self) -> u8 {
        self.channel
    }

    pub fn cc(&self) -> u8 {
        self.cc
    }
}
