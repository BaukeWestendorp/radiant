use std::path::PathBuf;

use eyre::Context;

use crate::Result;

#[derive(Default)]
#[derive(facet::Facet)]
pub struct Adapters {
    midi: MidiIo,
}

impl Adapters {
    pub fn midi(&self) -> &MidiIo {
        &self.midi
    }

    /// Reads a io from a file at the given path.
    pub fn read_from_file(path: &PathBuf) -> Result<Self> {
        let yaml_str = std::fs::read_to_string(&path)
            .with_context(|| format!("failed to open io file at '{}'", path.display()))?;
        facet_yaml::from_str(&yaml_str)
            .with_context(|| format!("failed to read io file at '{}'", path.display()))
    }
}

#[derive(Default)]
#[derive(facet::Facet)]
pub struct MidiIo {
    active_devices: Vec<String>,
}

impl MidiIo {
    pub fn active_devices(&self) -> &[String] {
        &self.active_devices
    }
}
