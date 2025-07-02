use std::{collections::HashMap, path::PathBuf};

use eyre::Context;

use crate::Result;

#[derive(Default)]
#[derive(facet::Facet)]
pub struct Adapters {
    midi: MidiConfig,
}

impl Adapters {
    pub fn midi(&self) -> &MidiConfig {
        &self.midi
    }

    /// Reads a io from a file at the given path.
    pub fn read_from_file(path: &PathBuf) -> Result<Self> {
        let yaml_str = std::fs::read_to_string(&path)
            .with_context(|| format!("failed to open adapters file at '{}'", path.display()))?;
        facet_yaml::from_str(&yaml_str)
            .with_context(|| format!("failed to read adapters file at '{}'", path.display()))
    }
}

#[derive(Default)]
#[derive(facet::Facet)]
pub struct MidiConfig {
    active_devices: Vec<String>,
    actions: MidiActions,
}

impl MidiConfig {
    pub fn active_devices(&self) -> &[String] {
        &self.active_devices
    }
}

#[derive(Default)]
#[derive(facet::Facet)]
pub struct MidiActions {
    executors: HashMap<u32, MidiExecutorAction>,
}

impl MidiActions {
    pub fn executors(&self) -> &HashMap<u32, MidiExecutorAction> {
        &self.executors
    }
}

#[derive(facet::Facet)]
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

#[derive(facet::Facet)]
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

#[derive(facet::Facet)]
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
