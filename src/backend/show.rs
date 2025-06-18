use std::{collections::HashMap, path::PathBuf};

use crate::backend::{
    object::{
        AnyPreset, AnyPresetId, DimmerPreset, DimmerPresetId, Executor, ExecutorId, FixtureGroup,
        FixtureGroupId, Sequence, SequenceId,
    },
    patch::Patch,
    pipeline::Pipeline,
};

#[derive(Default)]
pub struct Show {
    /// The path at which the [Showfile] is saved.
    /// Will be `None` if it has not been saved yet.
    path: Option<PathBuf>,

    pub patch: Patch,

    /// The programmer contains WIP output data that can be saved to a preset.
    pub programmer: Pipeline,

    pub executors: HashMap<ExecutorId, Executor>,
    pub sequences: HashMap<SequenceId, Sequence>,
    pub fixture_groups: HashMap<FixtureGroupId, FixtureGroup>,
    pub dimmer_presets: HashMap<DimmerPresetId, DimmerPreset>,
}

impl Show {
    pub fn new(path: Option<PathBuf>) -> Self {
        Self { path, ..Default::default() }
    }

    pub fn path(&self) -> Option<&PathBuf> {
        self.path.as_ref()
    }

    pub fn get_preset(&self, preset_id: &AnyPresetId) -> Option<&AnyPreset> {
        match preset_id {
            AnyPresetId::Dimmer(id) => Some(self.dimmer_presets.get(id)?.into()),
        }
    }
}
