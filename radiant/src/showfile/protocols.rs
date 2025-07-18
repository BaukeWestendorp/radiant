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
    sacn: SacnConfiguration,
}

impl Protocols {
    /// Returns the [SacnConfiguration] for this protocol set.
    pub fn sacn(&self) -> &SacnConfiguration {
        &self.sacn
    }

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

/// Configuration for sACN protocol output.
///
/// The [SacnConfiguration] struct contains a list of sACN sources, each of
/// which describes how DMX universes are mapped and transmitted over the
/// network.
#[derive(Default)]
#[derive(serde::Deserialize)]
pub struct SacnConfiguration {
    sources: Vec<SacnSourceConfiguration>,
}

impl SacnConfiguration {
    /// Returns the list of [SacnSourceConfiguration]s for this sACN
    /// configuration.
    pub fn sources(&self) -> &[SacnSourceConfiguration] {
        &self.sources
    }
}

/// Configuration for a single sACN source.
///
/// A [SacnSourceConfiguration] describes how one or more local DMX universes
/// are mapped to a destination universe and transmitted, including network
/// addressing and output type.
#[derive(serde::Deserialize)]
pub struct SacnSourceConfiguration {
    name: String,
    local_universes: Vec<dmx::UniverseId>,
    destination_universe: u16,
    priority: u8,
    preview_data: bool,
    r#type: SacnOutputType,
}

impl SacnSourceConfiguration {
    /// Returns the name of this sACN source.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the list of local DMX universes mapped by this source.
    pub fn local_universes(&self) -> &[dmx::UniverseId] {
        &self.local_universes
    }

    /// Returns the destination universe number for this sACN source.
    pub fn destination_universe(&self) -> u16 {
        self.destination_universe
    }

    /// Returns the sACN priority for this source.
    pub fn priority(&self) -> u8 {
        self.priority
    }

    /// Returns whether preview data is enabled for this source.
    pub fn preview_data(&self) -> bool {
        self.preview_data
    }

    /// Returns the [SacnOutputType] for this source.
    pub fn r#type(&self) -> &SacnOutputType {
        &self.r#type
    }
}

/// Specifies the output type for an sACN source.
///
/// The [SacnOutputType] determines how DMX data is transmitted over the
/// network, such as unicast to a specific IP address.
#[derive(serde::Deserialize)]
pub enum SacnOutputType {
    /// Unicast output to a specific destination IP address.
    Unicast {
        /// The destination IP.
        destination_ip: IpAddr,
    },
}
