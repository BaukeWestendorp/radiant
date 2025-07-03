use std::path::PathBuf;
use std::{fs, io};

use eyre::Context;

use crate::error::Result;
use crate::object::{ColorPreset, Cue, DimmerPreset, Executor, FixtureGroup, Sequence};

#[derive(Default)]
#[derive(serde::Deserialize)]
pub struct Objects {
    executors: Vec<Executor>,
    sequences: Vec<Sequence>,
    cues: Vec<Cue>,
    fixture_groups: Vec<FixtureGroup>,
    dimmer_presets: Vec<DimmerPreset>,
    color_presets: Vec<ColorPreset>,
}

impl Objects {
    pub fn executors(&self) -> &[Executor] {
        &self.executors
    }

    pub fn sequences(&self) -> &[Sequence] {
        &self.sequences
    }

    pub fn cues(&self) -> &[Cue] {
        &self.cues
    }

    pub fn fixture_groups(&self) -> &[FixtureGroup] {
        &self.fixture_groups
    }

    pub fn dimmer_presets(&self) -> &[DimmerPreset] {
        &self.dimmer_presets
    }

    pub fn color_presets(&self) -> &[ColorPreset] {
        &self.color_presets
    }

    /// Reads the [Objects] configuration from a file at the given path.
    ///
    /// The file must be in YAML format and match the [Patch] structure.
    pub fn read_from_file(path: &PathBuf) -> Result<Self> {
        let file = fs::File::open(path)
            .with_context(|| format!("failed to open objects file at '{}'", path.display()))?;
        let reader = io::BufReader::new(file);
        serde_yaml::from_reader(reader)
            .with_context(|| format!("failed to read objects file at '{}'", path.display()))
    }
}
