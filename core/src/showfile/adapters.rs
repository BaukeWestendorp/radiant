use std::collections::HashMap;
use std::path::PathBuf;
use std::{fs, io};

use eyre::Context;

use crate::error::Result;
use crate::object::ExecutorId;

/// Represents the configuration for all adapters used in a
/// [Showfile][crate::showfile::Showfile].
///
/// This includes configuration for MIDI and potentially other external device
/// adapters.
#[derive(Default)]
#[derive(serde::Deserialize)]
pub struct Adapters {
    midi: MidiConfiguration,
}

impl Adapters {
    /// Returns the [MidiConfiguration] for this adapter set.
    pub fn midi(&self) -> &MidiConfiguration {
        &self.midi
    }

    /// Reads the adapters configuration from a file at the given path.
    ///
    /// The file must be in YAML format and match the [Adapters] structure.
    pub fn read_from_file(path: &PathBuf) -> Result<Self> {
        let file = fs::File::open(path)
            .with_context(|| format!("failed to open adapters file at '{}'", path.display()))?;
        let reader = io::BufReader::new(file);
        serde_yaml::from_reader(reader)
            .with_context(|| format!("failed to read adapters file at '{}'", path.display()))
    }
}

/// Configuration for MIDI adapters, including active devices and action
/// mappings.
#[derive(Clone, Default)]
#[derive(serde::Deserialize)]
pub struct MidiConfiguration {
    active_devices: Vec<String>,
    actions: MidiActions,
}

impl MidiConfiguration {
    /// Returns the list of active MIDI device identifiers.
    pub fn active_devices(&self) -> &[String] {
        &self.active_devices
    }

    /// Returns the [MidiActions] mapping for this configuration.
    pub fn actions(&self) -> &MidiActions {
        &self.actions
    }
}

/// Maps executor IDs to their corresponding MIDI actions.
#[derive(Clone, Default)]
#[derive(serde::Deserialize)]
pub struct MidiActions {
    executors: HashMap<ExecutorId, MidiExecutorAction>,
}

impl MidiActions {
    /// Returns a mapping from [ExecutorId] to [MidiExecutorAction].
    pub fn executors(&self) -> &HashMap<ExecutorId, MidiExecutorAction> {
        &self.executors
    }
}

/// Describes the MIDI controls mapped to a single executor, including button
/// and fader actions.
#[derive(Debug, Clone, Copy)]
#[derive(serde::Deserialize)]
pub struct MidiExecutorAction {
    button: Option<MidiExecutorControl>,
    fader: Option<MidiExecutorControl>,
}

impl MidiExecutorAction {
    /// Returns the [MidiExecutorControl] for the executor's button, if
    /// configured.
    pub fn button(&self) -> Option<&MidiExecutorControl> {
        self.button.as_ref()
    }

    /// Returns the [MidiExecutorControl] for the executor's fader, if
    /// configured.
    pub fn fader(&self) -> Option<&MidiExecutorControl> {
        self.fader.as_ref()
    }
}

/// Represents a MIDI control (button or fader) mapped to an executor,
/// specifying the MIDI channel and message type.
#[derive(Debug, Clone, Copy)]
#[derive(serde::Deserialize)]
pub struct MidiExecutorControl {
    channel: u8,
    msg: MidiAction,
}

impl MidiExecutorControl {
    /// Returns the MIDI channel for this control.
    pub fn channel(&self) -> u8 {
        self.channel
    }

    /// Returns the [MidiAction] associated with this control.
    pub fn msg(&self) -> MidiAction {
        self.msg
    }
}

/// Represents a MIDI message type used for mapping controls to executors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(serde::Deserialize)]
pub enum MidiAction {
    /// A MIDI Note On/Off message with the specified note number.
    Note(u8),
    /// A MIDI Control Change (CC) message with the specified controller number.
    ControlChange(u8),
}
