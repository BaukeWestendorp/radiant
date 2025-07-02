use std::{fs, io, path::PathBuf};

use eyre::Context;

use crate::Result;

/// Contains all information regarding the mapping of fixtures to the DMX universes.
#[derive(Default)]
#[derive(serde::Deserialize)]
pub struct Patch {
    gdtf_files: Vec<String>,
    fixtures: Vec<Fixture>,
}

impl Patch {
    pub fn gdtf_files(&self) -> &[String] {
        &self.gdtf_files
    }

    pub fn fixtures(&self) -> &[Fixture] {
        &self.fixtures
    }

    /// Reads a patch from a file at the given path.
    pub fn read_from_file(path: &PathBuf) -> Result<Self> {
        let file = fs::File::open(path)
            .with_context(|| format!("failed to open patch file at '{}'", path.display()))?;
        let reader = io::BufReader::new(file);
        serde_yaml::from_reader(reader)
            .with_context(|| format!("failed to read patch file at '{}'", path.display()))
    }
}

/// Represents information about a single mapped fixture.
#[derive(serde::Deserialize)]
pub struct Fixture {
    id: u32,
    gdtf_file_index: usize,
    universe: u16,
    channel: u16,
    dmx_mode: String,
}

impl Fixture {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn gdtf_file_index(&self) -> usize {
        self.gdtf_file_index
    }

    pub fn universe(&self) -> u16 {
        self.universe
    }

    pub fn channel(&self) -> u16 {
        self.channel
    }

    pub fn dmx_mode(&self) -> &str {
        &self.dmx_mode
    }
}
