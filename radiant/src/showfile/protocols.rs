use std::net::IpAddr;
use std::path::PathBuf;
use std::{fs, io};

use eyre::Context;

use crate::error::Result;

/// Represents the protocols configuration for a
/// [Showfile][crate::showfile::Showfile].
///
/// The [Protocols] struct contains configuration for all supported output
/// protocols, such as sACN. It is responsible for describing how DMX data is
/// sent to external systems and devices.
#[derive(Default)]
#[derive(serde::Deserialize)]
pub struct Protocols {
    pub(crate) sacn_source_configurations: Vec<SacnSourceConfiguration>,
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
}

/// Configuration for a single sACN source.
///
/// A [SacnSourceConfiguration] describes how one or more local DMX universes
/// are mapped to a destination universe and transmitted, including network
/// addressing and output type.
#[derive(Debug, Clone)]
#[derive(serde::Deserialize)]
pub struct SacnSourceConfiguration {
    pub(crate) name: String,
    pub(crate) priority: u8,
    pub(crate) preview_data: bool,
    pub(crate) r#type: SacnOutputType,
}

/// Specifies the output type for an sACN source.
///
/// The [SacnOutputType] determines how DMX data is transmitted over the
/// network, such as unicast to a specific IP address.
#[derive(Debug, Clone)]
#[derive(serde::Deserialize)]
pub enum SacnOutputType {
    /// Unicast output to a specific destination IP address.
    Unicast {
        /// The destination IP.
        destination_ip: IpAddr,
    },
}
