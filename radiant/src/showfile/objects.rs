use std::path::PathBuf;
use std::{fs, io};

use eyre::Context;

use crate::error::Result;
use crate::show::ObjectContainer;

/// A collection of lighting control objects loaded from configuration.
///
/// The `Objects` struct contains all the major elements used in the lighting
/// control system, such as executors, sequences, cues, fixture groups, and
/// presets.
#[derive(Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Objects {
    #[serde(default)]
    pub(crate) object_container: ObjectContainer,
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

    pub fn write_to_file(&self, path: &PathBuf) -> Result<()> {
        let file = fs::File::create(path)
            .with_context(|| format!("failed to create objects file at '{}'", path.display()))?;
        let writer = io::BufWriter::new(file);
        serde_yaml::to_writer(writer, self)
            .with_context(|| format!("failed to write objects file at '{}'", path.display()))
    }
}
