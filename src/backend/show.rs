use std::{collections::HashMap, path::PathBuf};

use crate::backend::object::{
    AnyPreset, AnyPresetId, Cue, CueId, DimmerPreset, DimmerPresetId, Executor, ExecutorId,
    FixtureGroup, FixtureGroupId, Sequence, SequenceId,
};
use crate::backend::patch::Patch;
use crate::backend::pipeline::Pipeline;

#[derive(Debug, Default)]
pub struct Show {
    path: Option<PathBuf>,

    pub(in crate::backend) patch: Patch,

    /// The programmer contains WIP output data that can be saved to a preset.
    pub(in crate::backend) programmer: Pipeline,

    pub(in crate::backend) fixture_groups: HashMap<FixtureGroupId, FixtureGroup>,
    pub(in crate::backend) executors: HashMap<ExecutorId, Executor>,
    pub(in crate::backend) sequences: HashMap<SequenceId, Sequence>,
    pub(in crate::backend) cues: HashMap<CueId, Cue>,
    pub(in crate::backend) dimmer_presets: HashMap<DimmerPresetId, DimmerPreset>,
}

impl Show {
    pub fn new(path: Option<PathBuf>) -> Self {
        Self { path, ..Default::default() }
    }

    /// The path at which the [Showfile] is saved.
    /// Will be `None` if it has not been saved yet.
    pub fn path(&self) -> Option<&PathBuf> {
        self.path.as_ref()
    }

    /// Gets a [FixtureGroup].
    pub fn fixture_group(&self, id: &FixtureGroupId) -> Option<&FixtureGroup> {
        self.fixture_groups.get(id)
    }

    /// Gets an iterator all [FixtureGroup]s.
    pub fn fixture_groups(&self) -> impl IntoIterator<Item = &FixtureGroup> {
        self.fixture_groups.values()
    }

    /// Gets an [Executor].
    pub fn executor(&self, id: &ExecutorId) -> Option<&Executor> {
        self.executors.get(id)
    }

    /// Gets an iterator all [Executor]s.
    pub fn executors(&self) -> impl IntoIterator<Item = &Executor> {
        self.executors.values()
    }

    /// Gets a [Sequence].
    pub fn sequence(&self, id: &SequenceId) -> Option<&Sequence> {
        self.sequences.get(id)
    }

    /// Gets an iterator all [Sequence]s.
    pub fn sequences(&self) -> impl IntoIterator<Item = &Sequence> {
        self.sequences.values()
    }

    /// Gets a [Cue].
    pub fn cue(&self, id: &CueId) -> Option<&Cue> {
        self.cues.get(id)
    }

    /// Gets an iterator all [Cue]s.
    pub fn cues(&self) -> impl IntoIterator<Item = &Cue> {
        self.cues.values()
    }

    /// Gets any kind of preset from it's corresponding id.
    pub fn preset(&self, preset_id: &AnyPresetId) -> Option<&AnyPreset> {
        match preset_id {
            AnyPresetId::Dimmer(id) => Some(self.dimmer_presets.get(id)?.into()),
        }
    }
}
