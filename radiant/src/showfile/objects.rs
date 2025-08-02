use std::path::PathBuf;
use std::{fs, io};

use eyre::Context;

use crate::error::Result;
use crate::show::{
    Executor, Group, PresetBeam, PresetColor, PresetControl, PresetDimmer, PresetFocus, PresetGobo,
    PresetPosition, PresetShapers, PresetVideo, Sequence,
};

/// A collection of lighting control objects loaded from configuration.
///
/// The `Objects` struct contains all the major elements used in the lighting
/// control system, such as executors, sequences, cues, fixture groups, and
/// presets.
#[derive(Default)]
#[derive(serde::Deserialize)]
pub struct Objects {
    pub(crate) groups: Vec<Group>,
    pub(crate) sequences: Vec<Sequence>,
    pub(crate) executors: Vec<Executor>,

    pub(crate) dimmer_presets: Vec<PresetDimmer>,
    pub(crate) position_presets: Vec<PresetPosition>,
    pub(crate) gobo_presets: Vec<PresetGobo>,
    pub(crate) color_presets: Vec<PresetColor>,
    pub(crate) beam_presets: Vec<PresetBeam>,
    pub(crate) focus_presets: Vec<PresetFocus>,
    pub(crate) shapers_presets: Vec<PresetShapers>,
    pub(crate) control_presets: Vec<PresetControl>,
    pub(crate) video_presets: Vec<PresetVideo>,
}

impl Objects {
    /// Reads the [Objects] configuration from a file at the given path.
    ///
    /// The file must be in YAML format and match the
    /// [Patch][crate::patch::Patch] structure.
    pub fn read_from_file(path: &PathBuf) -> Result<Self> {
        let file = fs::File::open(path)
            .with_context(|| format!("failed to open objects file at '{}'", path.display()))?;
        let reader = io::BufReader::new(file);
        serde_yaml::from_reader(reader)
            .with_context(|| format!("failed to read objects file at '{}'", path.display()))
    }
}
