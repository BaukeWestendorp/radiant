use std::path::PathBuf;
use std::{fs, io};

use eyre::Context;

use crate::error::Result;
use crate::show::ProtocolConfig;

/// Represents the protocols configuration for a
/// [Showfile][crate::showfile::Showfile].
///
/// The [Protocols] struct contains configuration for all supported output
/// protocols, such as sACN. It is responsible for describing how DMX data is
/// sent to external systems and devices.
#[derive(Debug, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Protocols {
    pub(crate) protocol_config: ProtocolConfig,
}

impl Protocols {
    /// Reads the [Protocols] configuration from a file at the given path.
    ///
    /// The file must be in YAML format and match the [Protocols] structure.
    pub fn read_from_file(path: &PathBuf) -> Result<Self> {
        let file = fs::File::open(path)
            .with_context(|| format!("failed to open protocols file at '{}'", path.display()))?;
        let reader = io::BufReader::new(file);
        serde_yaml::from_reader(reader)
            .with_context(|| format!("failed to read protocols file at '{}'", path.display()))
    }

    pub fn write_to_file(&self, path: &PathBuf) -> Result<()> {
        let file = fs::File::create(path)
            .with_context(|| format!("failed to create protocols file at '{}'", path.display()))?;
        let writer = io::BufWriter::new(file);
        serde_yaml::to_writer(writer, self)
            .with_context(|| format!("failed to write protocols file at '{}'", path.display()))?;
        Ok(())
    }
}
