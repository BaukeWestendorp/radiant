//! Command system for interacting with the engine and showfile.
//!
//! This module defines the [Command] system,
//! which is the primary interface for interacting with the
//! [Engine][crate::engine::Engine]. Commands represent actions such as
//! modifying the patch, controlling executors, or updating the programmer. This
//! module also provides parsing utilities and command variants for all
//! supported operations.

use std::str::FromStr;

use crate::object::{
    AnyObjectId, AnyPresetId, CueId, ExecutorButtonMode, ExecutorFaderMode, ExecutorId,
    FixtureGroupId, Recipe, SequenceId,
};
use crate::patch::{Attribute, AttributeValue, DmxMode, FixtureId};

mod lexer;
mod parser;

/// Parses a string into a [Command].
///
/// # Panic
/// This will panic if the provided command is not valid.
#[macro_export]
macro_rules! cmd {
    ($cmd_str:expr) => {{
        use std::str::FromStr;
        $crate::cmd::Command::from_str($cmd_str).expect("failed to parse command")
    }};
}

/// The interface between the engine and the backend.
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    /// Modify the [Patch][crate::patch::Patch].
    Patch(PatchCommand),
    /// Modify the programmer.
    Programmer(ProgrammerCommand),
    /// Create a new object with the given id and optional name.
    Create {
        /// The object's id.
        id: AnyObjectId,
        /// Optional name for the object.
        name: Option<String>,
    },
    /// Remove an object with the given id.
    Remove {
        /// The object's id.
        id: AnyObjectId,
    },
    /// Rename an object with the given id.
    Rename {
        /// The object's id.
        id: AnyObjectId,
        /// The object's new name.
        name: String,
    },

    /// Modify a [FixtureGroup][crate::object::FixtureGroup] with the given id.
    FixtureGroup(FixtureGroupId, FixtureGroupCommand),
    /// Modify an [Executor][crate::object::Executor] with the given id.
    Executor(ExecutorId, ExecutorCommand),
    /// Modify a [Sequence][crate::object::Sequence] with the given id.
    Sequence(SequenceId, SequenceCommand),
    /// Modify a [Cue][crate::object::Cue] with the given id.
    Cue(CueId, CueCommand),
    /// Modify a preset with the given id.
    Preset(AnyPresetId, PresetCommand),
}

impl FromStr for Command {
    type Err = eyre::Error;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        parser::Parser::new(source).parse()
    }
}

/// A sub-command to modify the [Patch][crate::patch::Patch].
#[derive(Debug, Clone, PartialEq)]
pub enum PatchCommand {
    /// Add a fixture to the [Patch][crate::patch::Patch].
    Add {
        /// The [FixtureId] for the new fixture.
        id: FixtureId,
        /// The [dmx::Address] for the new fixture.
        address: dmx::Address,
        /// The associated GDTF file name for the new fixture.
        gdtf_file_name: String,
        /// The [DmxMode] for the new fixture.
        mode: DmxMode,
    },
    /// Set the [dmx::Address] of a fixture in the [Patch][crate::patch::Patch].
    SetAddress {
        /// The id of the fixture to modify.
        id: FixtureId,
        /// The new [dmx::Address].
        address: dmx::Address,
    },
    /// Set the [DmxMode] of a fixture.
    SetMode {
        /// The id of the fixture to modify.
        id: FixtureId,
        /// The new [DmxMode].
        mode: DmxMode,
    },
    /// Set the associated GDTF file name of a fixture in the
    /// [Patch][crate::patch::Patch].
    SetGdtfFileName {
        /// The id of the fixture to modify.
        id: FixtureId,
        /// The associated GDTF file name.
        name: String,
    },
    /// Remove a fixture from the [Patch][crate::patch::Patch].
    Remove {
        /// The id of the fixture to remove.
        id: FixtureId,
    },
}

/// A sub-command to modify the programmer.
#[derive(Debug, Clone, PartialEq)]
pub enum ProgrammerCommand {
    /// Set a value in the programmer.
    Set(ProgrammerSetCommand),
    /// Clear all values in the programmer.
    Clear,
}

/// A sub-command to set values in the programmer.
#[derive(Debug, Clone, PartialEq)]
pub enum ProgrammerSetCommand {
    /// Set a DMX value directly into the programmer.
    Direct {
        /// The [dmx::Address] to modify.
        address: dmx::Address,
        /// The new [dmx::Value] at the given address.
        value: dmx::Value,
    },
    /// Sets the [AttributeValue] of an [Attribute] for the given [FixtureId].
    Attribute {
        /// The id of the fixture.
        id: FixtureId,
        /// The [Attribute] to change.
        attribute: Attribute,
        /// The new [AttributeValue] for the given [Attribute].
        value: AttributeValue,
    },
}

/// A sub-command to modify a [FixtureGroup][crate::object::FixtureGroup].
#[derive(Debug, Clone, PartialEq)]
pub enum FixtureGroupCommand {
    /// Add a list of [FixtureId]s.
    Add {
        /// [FixtureId]s to add.
        ids: Vec<FixtureId>,
    },
    /// Replace a [FixtureId] at the given index.
    ReplaceAt {
        /// The index of the [FixtureId] you want to replace.
        index: usize,
        /// The new [FixtureId].
        id: FixtureId,
    },
    /// Remove a [FixtureId].
    Remove {
        /// The [FixtureId] to remove.
        id: FixtureId,
    },
    /// Remove a [FixtureId] at the given index.
    RemoveAt {
        /// The index of the [FixtureId] to remove.
        index: usize,
    },
    /// Remove all [FixtureId]s.
    Clear,
}

/// A sub-command to modify an [Executor][crate::object::Executor].
#[derive(Debug, Clone, PartialEq)]
pub enum ExecutorCommand {
    /// Modify or interact with the [Executor][crate::object::Executor]'s
    /// button.
    Button(ExecutorButtonCommand),
    /// Modify or interact with the [Executor][crate::object::Executor]'s fader.
    Fader(ExecutorFaderCommand),
    /// Set the sequence for this executor.
    SetSequence {
        /// The id of the sequence to associate with this executor.
        sequence_id: SequenceId,
    },
    /// Removes the associated sequence from the executor.
    Clear,
}

/// A sub-command to modify or interact with an
/// [Executor][crate::object::Executor]'s button.
#[derive(Debug, Clone, PartialEq)]
pub enum ExecutorButtonCommand {
    /// Set the mode of operation for the executor button.
    SetMode {
        /// The new mode for the button.
        mode: ExecutorButtonMode,
    },
    /// Simulate pressing the executor button.
    Press,
    /// Simulate releasing the executor button.
    Release,
}

/// A sub-command to modify or interact with an
/// [Executor][crate::object::Executor]'s fader.
#[derive(Debug, Clone, PartialEq)]
pub enum ExecutorFaderCommand {
    /// Set the mode of operation for the executor fader.
    SetMode {
        /// The new mode for the fader.
        mode: ExecutorFaderMode,
    },
    /// Set the level of the executor fader.
    SetLevel {
        /// The new level for the fader, in range 0.0 to 1.0.
        level: f32,
    },
}

/// A sub-command to modify a [Sequence][crate::object::Sequence].
#[derive(Debug, Clone, PartialEq)]
pub enum SequenceCommand {
    /// Add cues to the sequence.
    Add {
        /// The ids of cues to add to the sequence.
        cue_ids: Vec<CueId>,
    },
    /// Replace a cue at a specific index in the sequence.
    ReplaceAt {
        /// The index where the cue should be replaced.
        index: usize,
        /// The id of the new cue to insert at the given index.
        cue_id: CueId,
    },
    /// Remove a specific cue from the sequence.
    Remove {
        /// The id of the cue to remove.
        cue_id: CueId,
    },
    /// Remove a cue at a specific index in the sequence.
    RemoveAt {
        /// The index of the cue to remove.
        index: usize,
    },
    /// Remove all cues from the sequence.
    Clear,
}

/// A sub-command to modify a [Cue][crate::object::Cue].
#[derive(Debug, Clone, PartialEq)]
pub enum CueCommand {
    /// Add recipes to the cue.
    Add {
        /// The recipes to add to the cue.
        recipes: Vec<Recipe>,
    },
    /// Replace a recipe at a specific index in the cue.
    ReplaceAt {
        /// The index where the recipe should be replaced.
        index: usize,
        /// The new recipe to insert at the given index.
        recipe: Recipe,
    },
    /// Remove a recipe at a specific index in the cue.
    RemoveAt {
        /// The index of the recipe to remove.
        index: usize,
    },
    /// Remove all recipes from the cue.
    Clear,
}

/// A sub-command to modify a preset.
#[derive(Debug, Clone, PartialEq)]
pub enum PresetCommand {
    /// Store current programmer values into the preset.
    Store,
    /// Clear all values from the preset.
    Clear,
}
