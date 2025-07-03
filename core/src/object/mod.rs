//! Show object types and data model.
//!
//! This module defines all show object types, such as executors,
//! sequences, presets, fixture groups, cues, and more. Objects are entities
//! that can be referenced by an id and contain show-related data. This module
//! provides a unified model for accessing and manipulating all show entities.

pub use cue::*;
pub use executor::*;
pub use fixture_group::*;
pub use preset::*;
pub use sequence::*;

mod cue;
mod executor;
mod fixture_group;
mod preset;
mod sequence;

macro_rules! define_object_id {
    ($name:ident) => {
        #[doc = concat!("A unique identifier for a ", stringify!($name), " object")]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
        #[derive(
            derive_more::Display,
            derive_more::From,
            derive_more::Into,
            derive_more::Add,
            derive_more::Sub,
            derive_more::AddAssign,
            derive_more::SubAssign,
            derive_more::MulAssign,
            derive_more::DivAssign
        )]
        #[derive(serde::Serialize, serde::Deserialize)]
        #[serde(transparent)]
        pub struct $name(pub u32);
    };
}

use define_object_id;

/// Any object.
#[derive(Debug, Clone, PartialEq)]
#[derive(derive_more::From)]
pub enum AnyObject {
    /// An [Executor].
    #[from]
    Executor(Executor),
    /// A [Sequence].
    #[from]
    Sequence(Sequence),
    /// A [Cue].
    #[from]
    Cue(Cue),
    /// A [FixtureGroup].
    #[from]
    FixtureGroup(FixtureGroup),
    /// An [AnyPreset].
    #[from(forward)]
    Preset(AnyPreset),
}

/// Any object id.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(derive_more::Display, derive_more::From, derive_more::TryInto)]
pub enum AnyObjectId {
    /// An [ExecutorId].
    #[from]
    Executor(ExecutorId),
    /// A [SequenceId].
    #[from]
    Sequence(SequenceId),
    /// A [CueId].
    #[from]
    Cue(CueId),
    /// A [FixtureGroupId].
    #[from]
    FixtureGroup(FixtureGroupId),
    /// [Any preset id][AnyPresetId].
    #[from(forward)]
    Preset(AnyPresetId),
}
