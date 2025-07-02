use std::collections::HashMap;
use std::path::PathBuf;

use crate::pipeline::Pipeline;
use crate::{
    AnyPreset, AnyPresetId, ColorPreset, ColorPresetId, Cue, CueId, DimmerPreset, DimmerPresetId,
    Executor, ExecutorId, FixtureGroup, FixtureGroupId, Patch, Sequence, SequenceId,
};

#[derive(Debug, Clone, Default)]
pub struct Show {
    path: Option<PathBuf>,

    pub(crate) patch: Patch,

    /// The programmer contains WIP output data that can be saved to a preset.
    pub(crate) programmer: Pipeline,

    pub(crate) fixture_groups: HashMap<FixtureGroupId, FixtureGroup>,
    pub(crate) executors: HashMap<ExecutorId, Executor>,
    pub(crate) sequences: HashMap<SequenceId, Sequence>,
    pub(crate) cues: HashMap<CueId, Cue>,
    pub(crate) dimmer_presets: HashMap<DimmerPresetId, DimmerPreset>,
    pub(crate) color_presets: HashMap<ColorPresetId, ColorPreset>,
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

    pub fn patch(&self) -> &Patch {
        &self.patch
    }
    /// Gets a [FixtureGroup].
    pub fn fixture_group(&self, id: impl Into<FixtureGroupId>) -> Option<&FixtureGroup> {
        self.fixture_groups.get(&id.into())
    }

    /// Gets a mutable [FixtureGroup].
    pub fn fixture_group_mut(
        &mut self,
        id: impl Into<FixtureGroupId>,
    ) -> Option<&mut FixtureGroup> {
        self.fixture_groups.get_mut(&id.into())
    }

    /// Gets an iterator all [FixtureGroup]s.
    pub fn fixture_groups(&self) -> impl IntoIterator<Item = &FixtureGroup> {
        self.fixture_groups.values()
    }

    /// Gets an [Executor].
    pub fn executor(&self, id: impl Into<ExecutorId>) -> Option<&Executor> {
        self.executors.get(&id.into())
    }

    /// Gets a mutable [Executor].
    pub fn executor_mut(&mut self, id: impl Into<ExecutorId>) -> Option<&mut Executor> {
        self.executors.get_mut(&id.into())
    }

    /// Gets an iterator all [Executor]s.
    pub fn executors(&self) -> impl IntoIterator<Item = &Executor> {
        self.executors.values()
    }

    /// Gets a [Sequence].
    pub fn sequence(&self, id: impl Into<SequenceId>) -> Option<&Sequence> {
        self.sequences.get(&id.into())
    }

    /// Gets a mutable [Sequence].
    pub fn sequence_mut(&mut self, id: impl Into<SequenceId>) -> Option<&mut Sequence> {
        self.sequences.get_mut(&id.into())
    }

    /// Gets an iterator all [Sequence]s.
    pub fn sequences(&self) -> impl IntoIterator<Item = &Sequence> {
        self.sequences.values()
    }

    /// Gets a [Cue].
    pub fn cue(&self, id: impl Into<CueId>) -> Option<&Cue> {
        self.cues.get(&id.into())
    }

    /// Gets a mutable [Cue].
    pub fn cue_mut(&mut self, id: impl Into<CueId>) -> Option<&mut Cue> {
        self.cues.get_mut(&id.into())
    }

    /// Gets an iterator all [Cue]s.
    pub fn cues(&self) -> impl IntoIterator<Item = &Cue> {
        self.cues.values()
    }

    /// Gets any kind of preset from it's corresponding id.
    pub fn preset(&self, preset_id: impl Into<AnyPresetId>) -> Option<AnyPreset> {
        match preset_id.into() {
            AnyPresetId::Dimmer(id) => Some(self.dimmer_presets.get(&id)?.clone().into_any()),
            AnyPresetId::Color(id) => Some(self.color_presets.get(&id)?.clone().into_any()),
        }
    }

    pub fn preset_dimmer(&self, id: impl Into<DimmerPresetId>) -> Option<&DimmerPreset> {
        self.dimmer_presets.get(&id.into())
    }

    pub fn preset_color(&self, id: impl Into<ColorPresetId>) -> Option<&ColorPreset> {
        self.color_presets.get(&id.into())
    }
}
