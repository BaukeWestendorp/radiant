use std::str::FromStr;

use crate::{
    AnyObjectId, AnyPresetId, Attribute, AttributeValue, CueId, DmxMode, ExecutorButtonMode,
    ExecutorFaderMode, ExecutorId, FixtureGroupId, FixtureId, Recipe, SequenceId, dmx,
};

mod lexer;
mod parser;

#[macro_export]
macro_rules! cmd {
    ($cmd_str:expr) => {{
        use std::str::FromStr;
        $crate::engine::cmd::Command::from_str($cmd_str).expect("failed to parse command")
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
    Add { ids: Vec<FixtureId> },
    ReplaceAt { index: usize, id: FixtureId },
    Remove { id: FixtureId },
    RemoveAt { index: usize },
    Clear,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExecutorCommand {
    Button(ExecutorButtonCommand),
    Fader(ExecutorFaderCommand),
    SetSequence { sequence_id: SequenceId },
    Clear,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExecutorButtonCommand {
    SetMode { mode: ExecutorButtonMode },
    Press,
    Release,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExecutorFaderCommand {
    SetMode { mode: ExecutorFaderMode },
    SetLevel { level: f32 },
}

#[derive(Debug, Clone, PartialEq)]
pub enum SequenceCommand {
    Add { cue_ids: Vec<CueId> },
    ReplaceAt { index: usize, cue_id: CueId },
    Remove { cue_id: CueId },
    RemoveAt { index: usize },
    Clear,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CueCommand {
    Add { recipes: Vec<Recipe> },
    ReplaceAt { index: usize, recipe: Recipe },
    RemoveAt { index: usize },
    Clear,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PresetCommand {
    Store,
    Clear,
}
