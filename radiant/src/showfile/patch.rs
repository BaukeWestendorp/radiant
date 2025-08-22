use std::path::PathBuf;
use std::{fs, io};

use eyre::Context;

use crate::error::Result;
use crate::show::FixtureTypeId;

/// Represents the patch configuration for a show, including all information
/// about the mapping of fixtures to DMX universes and their associated GDTF
/// files.
///
/// The [Patch] contains a list of GDTF file names and all [Fixture]s that are
/// mapped in the show. It is responsible for describing how logical fixtures
/// are assigned to physical DMX addresses and universes.
#[derive(Debug, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Patch {
    pub(crate) fixtures: Vec<Fixture>,
}

impl Patch {
    /// Reads the [Patch] configuration from a file at the given path.
    ///
    /// The file must be in YAML format and match the [Patch] structure.
    pub fn read_from_file(path: &PathBuf) -> Result<Self> {
        let file = fs::File::open(path)
            .with_context(|| format!("failed to open patch file at '{}'", path.display()))?;
        let reader = io::BufReader::new(file);
        serde_yaml::from_reader(reader)
            .with_context(|| format!("failed to read patch file at '{}'", path.display()))
    }
}

/// Represents a single fixture mapped in the [Patch].
///
/// A [Fixture] describes the logical-to-physical mapping for a fixture,
/// including its unique id, the index of its GDTF file, DMX universe and
/// channel, and the DMX mode used for addressing.
#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Fixture {
    pub(crate) fid: u32,
    pub(crate) gdtf_type_id: FixtureTypeId,
    pub(crate) universe: u16,
    pub(crate) channel: u16,
    pub(crate) dmx_mode: String,
}
