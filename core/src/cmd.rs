//! Command system for interacting with the engine and showfile.
//!
//! This module defines the [Command] system,
//! which is the primary interface for interacting with the
//! [Engine][crate::engine::Engine]. Commands represent actions such as
//! modifying the patch, controlling executors, or updating the programmer. This
//! module also provides parsing utilities and command variants for all
//! supported operations.

use crate::object::{
    AnyObjectId, AnyPresetId, CueId, ExecutorButtonMode, ExecutorFaderMode, ExecutorId,
    FixtureGroupId, Recipe, SequenceId,
};
use crate::patch::{Attribute, AttributeValue, DmxMode, FixtureId};

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

    /// Modify an object.
    Object(ObjectCommand),
}

/// A sub-command to modify the [Patch][crate::patch::Patch].
#[derive(Debug, Clone, PartialEq)]
pub enum PatchCommand {
    /// Add a fixture to the [Patch][crate::patch::Patch].
    Add {
        /// The [FixtureId] for the new fixture.
        fid: FixtureId,
        /// The [dmx::Address] for the new fixture.
        address: dmx::Address,
        /// The associated GDTF file name for the new fixture.
        gdtf: String,
        /// The [DmxMode] for the new fixture.
        mode: DmxMode,
    },
    /// Set the [dmx::Address] of a fixture in the [Patch][crate::patch::Patch].
    SetAddress {
        /// The id of the fixture to modify.
        fid: FixtureId,
        /// The new [dmx::Address].
        address: dmx::Address,
    },
    /// Set the [DmxMode] of a fixture.
    SetMode {
        /// The id of the fixture to modify.
        fid: FixtureId,
        /// The new [DmxMode].
        mode: DmxMode,
    },
    /// Set the associated GDTF file name of a fixture in the
    /// [Patch][crate::patch::Patch].
    SetGdtf {
        /// The id of the fixture to modify.
        fid: FixtureId,
        /// The associated GDTF file name.
        name: String,
    },
    /// Remove a fixture from the [Patch][crate::patch::Patch].
    Remove {
        /// The id of the fixture to remove.
        fid: FixtureId,
    },
}

/// A sub-command to modify the programmer.
#[derive(Debug, Clone, PartialEq)]
pub enum ProgrammerCommand {
    /// Set a DMX value directly into the programmer.
    SetAddress {
        /// The [dmx::Address] to modify.
        address: dmx::Address,
        /// The new [dmx::Value] at the given address.
        value: dmx::Value,
    },
    /// Sets the [AttributeValue] of an [Attribute] for the given [FixtureId].
    SetAttribute {
        /// The id of the fixture.
        fid: FixtureId,
        /// The [Attribute] to change.
        attribute: Attribute,
        /// The new [AttributeValue] for the given [Attribute].
        value: AttributeValue,
    },
    /// Clear all values in the programmer.
    Clear,
}

/// A sub-command to modify an object.
#[derive(Debug, Clone, PartialEq)]
pub enum ObjectCommand {
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

/// A sub-command to modify a [FixtureGroup][crate::object::FixtureGroup].
#[derive(Debug, Clone, PartialEq)]
pub enum FixtureGroupCommand {
    /// Add a list of [FixtureId]s.
    Add {
        /// [FixtureId] to add.
        fid: FixtureId,
    },
    /// Replace a [FixtureId] at the given index.
    ReplaceAt {
        /// The index of the [FixtureId] you want to replace.
        index: usize,
        /// The new [FixtureId].
        fid: FixtureId,
    },
    /// Remove a [FixtureId].
    Remove {
        /// The [FixtureId] to remove.
        fid: FixtureId,
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
    /// Set the mode of operation for the executor button.
    ButtonSetMode {
        /// The new mode for the button.
        mode: ExecutorButtonMode,
    },
    /// Simulate pressing the executor button.
    ButtonPress,
    /// Simulate releasing the executor button.
    ButtonRelease,
    /// Set the mode of operation for the executor fader.
    FaderSetMode {
        /// The new mode for the fader.
        mode: ExecutorFaderMode,
    },
    /// Set the level of the executor fader.
    FaderSetLevel {
        /// The new level for the fader, in range 0.0 to 1.0.
        level: f32,
    },
    /// Set the sequence for this executor.
    SetSequence {
        /// The id of the sequence to associate with this executor.
        sequence_id: SequenceId,
    },
    /// Removes the associated sequence from the executor.
    Clear,
}

/// A sub-command to modify a [Sequence][crate::object::Sequence].
#[derive(Debug, Clone, PartialEq)]
pub enum SequenceCommand {
    /// Add cues to the sequence.
    Add {
        /// The id of cue to add to the sequence.
        cue_id: CueId,
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
        /// The recipe to add to the cue.
        recipe: Recipe,
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
