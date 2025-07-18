use std::path::PathBuf;
use std::{fs, io};

use eyre::Context;

use crate::error::Result;

/// Represents the patch configuration for a show, including all information
/// about the mapping of fixtures to DMX universes and their associated GDTF
/// files.
///
/// The [Patch] contains a list of GDTF file names and all [Fixture]s that are
/// mapped in the show. It is responsible for describing how logical fixtures
/// are assigned to physical DMX addresses and universes.
#[derive(Default)]
#[derive(serde::Deserialize)]
pub struct Patch {
    gdtf_files: Vec<String>,
    fixtures: Vec<Fixture>,
}

impl Patch {
    /// Returns the list of GDTF file names referenced by this patch.
    pub fn gdtf_files(&self) -> &[String] {
        &self.gdtf_files
    }

    /// Returns the list of [Fixture]s mapped in this patch.
    pub fn fixtures(&self) -> &[Fixture] {
        &self.fixtures
    }

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
#[derive(serde::Deserialize)]
pub struct Fixture {
    id: u32,
    gdtf_file_index: usize,
    universe: u16,
    channel: u16,
    dmx_mode: String,
}

impl Fixture {
    /// Returns the unique id of this fixture.
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Returns the index into the patch's GDTF file list for this fixture.
    pub fn gdtf_file_index(&self) -> usize {
        self.gdtf_file_index
    }

    /// Returns the DMX universe this fixture is mapped to.
    pub fn universe(&self) -> u16 {
        self.universe
    }

    /// Returns the starting DMX channel for this fixture within its universe.
    pub fn channel(&self) -> u16 {
        self.channel
    }

    /// Returns the DMX mode string for this fixture.
    pub fn dmx_mode(&self) -> &str {
        &self.dmx_mode
    }
}
