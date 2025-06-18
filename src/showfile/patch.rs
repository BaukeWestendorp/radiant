use std::path::PathBuf;

use eyre::Context;

use crate::error::Result;

/// Contains all information regarding the mapping of fixtures to the DMX universes.
#[derive(Default)]
#[derive(facet::Facet)]
pub struct Patch {
    pub gdtf_files: Vec<String>,
    pub fixtures: Vec<Fixture>,
}

impl Patch {
    /// Reads a patch from a file at the given path.
    pub fn read_from_file(path: PathBuf) -> Result<Patch> {
        let yaml_str = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to open patch file at '{}'", path.display()))?;
        facet_yaml::from_str(&yaml_str)
            .with_context(|| format!("Failed to read patch file at '{}'", path.display()))
    }
}

/// Represents information about a single mapped fixture.
#[derive(facet::Facet)]
pub struct Fixture {
    pub id: u32,
    pub gdtf_file_index: usize,
    pub universe: u16,
    pub channel: u16,
    pub dmx_mode: String,
}
