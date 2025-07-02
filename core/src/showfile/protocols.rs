use std::{fs, io, net::IpAddr, path::PathBuf};

use eyre::Context;

use crate::Result;

#[derive(Default)]
#[derive(serde::Deserialize)]
pub struct Protocols {
    sacn: SacnConfiguration,
}

impl Protocols {
    pub fn sacn(&self) -> &SacnConfiguration {
        &self.sacn
    }

    /// Reads the protocols configuration from a file at the given path.
    pub fn read_from_file(path: &PathBuf) -> Result<Self> {
        let file = fs::File::open(path)
            .with_context(|| format!("failed to open protocols file at '{}'", path.display()))?;
        let reader = io::BufReader::new(file);
        serde_yaml::from_reader(reader)
            .with_context(|| format!("failed to read protocols file at '{}'", path.display()))
    }
}

#[derive(Default)]
#[derive(serde::Deserialize)]
pub struct SacnConfiguration {
    sources: Vec<SacnSourceConfiguration>,
}

impl SacnConfiguration {
    pub fn sources(&self) -> &[SacnSourceConfiguration] {
        &self.sources
    }
}

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
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn local_universes(&self) -> &[dmx::UniverseId] {
        &self.local_universes
    }

    pub fn destination_universe(&self) -> u16 {
        self.destination_universe
    }

    pub fn priority(&self) -> u8 {
        self.priority
    }

    pub fn preview_data(&self) -> bool {
        self.preview_data
    }

    pub fn r#type(&self) -> &SacnOutputType {
        &self.r#type
    }
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SacnOutputType {
    Unicast { destination_ip: Option<IpAddr> },
}
