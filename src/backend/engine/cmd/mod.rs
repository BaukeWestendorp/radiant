use std::str::FromStr;

use crate::{
    backend::{
        object::{
            ActivationMode, AnyObjectId, AnyPresetId, CueId, ExecutorId, FixtureGroupId, Recipe,
            SequenceId, TerminationMode,
        },
        patch::{
            attr::{Attribute, AttributeValue},
            fixture::{DmxMode, FixtureId},
        },
    },
    dmx,
};

mod lexer;
mod parser;

#[macro_export]
macro_rules! cmd {
    ($cmd_str:expr) => {{
        use std::str::FromStr;
        $crate::backend::engine::cmd::Command::from_str($cmd_str).expect("Failed to parse command")
    }};
}

/// A [Command] is the interface between the engine and the backend.
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    // Functions.
    Patch(PatchCommand),
    Programmer(ProgrammerCommand),
    Create { id: AnyObjectId, name: Option<String> },
    Remove { id: AnyObjectId },
    Rename { id: AnyObjectId, name: String },

    // Objects.
    FixtureGroup(FixtureGroupId, FixtureGroupCommand),
    Executor(ExecutorId, ExecutorCommand),
    Sequence(SequenceId, SequenceCommand),
    Cue(CueId, CueCommand),
    Preset(AnyPresetId, PresetCommand),
}

impl FromStr for Command {
    type Err = eyre::Error;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        parser::Parser::new(source).parse()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PatchCommand {
    Add { id: FixtureId, address: dmx::Address, gdtf_file_name: String, mode: DmxMode },
    SetAddress { id: FixtureId, address: dmx::Address },
    SetMode { id: FixtureId, mode: DmxMode },
    SetGdtfFileName { id: FixtureId, name: String },
    Remove { id: FixtureId },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProgrammerCommand {
    Set(ProgrammerSetCommand),
    Clear,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProgrammerSetCommand {
    Direct { address: dmx::Address, value: dmx::Value },
    Attribute { id: FixtureId, attribute: Attribute, value: AttributeValue },
}

#[derive(Debug, Clone, PartialEq)]
pub enum FixtureGroupCommand {
    Add { id: FixtureId },
    ReplaceAt { index: usize, id: FixtureId },
    Remove { id: FixtureId },
    RemoveAt { index: usize },
    Clear,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExecutorCommand {
    SetActivationMode { mode: ActivationMode },
    SetTerminationMode { mode: TerminationMode },
    SetSequenceId { sequence_id: SequenceId },
    Clear,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SequenceCommand {
    Add { cue_id: CueId },
    ReplaceAt { index: usize, cue_id: CueId },
    Remove { cue_id: CueId },
    RemoveAt { index: usize },
    Clear,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CueCommand {
    Add { recipe: Recipe },
    ReplaceAt { index: usize, recipe: Recipe },
    RemoveAt { index: usize },
    Clear,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PresetCommand {
    Store,
    Clear,
}
