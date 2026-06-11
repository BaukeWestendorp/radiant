use std::net::SocketAddr;

use crate::dmx::UniverseId;

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OutputDefinition {
    pub(crate) sacn: SacnDmxOutputDefinition,
    pub(crate) enttec: EnttecDmxOutputDefinition,
}

impl OutputDefinition {
    pub fn sacn(&self) -> &SacnDmxOutputDefinition {
        &self.sacn
    }

    pub fn enttec(&self) -> &EnttecDmxOutputDefinition {
        &self.enttec
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SacnDmxOutputDefinition {
    pub(crate) instances: Vec<SacnDmxOutputInstanceDefinition>,
}

impl SacnDmxOutputDefinition {
    pub fn instances(&self) -> &[SacnDmxOutputInstanceDefinition] {
        &self.instances
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SacnDmxOutputInstanceDefinition {
    pub(crate) name: String,
    pub(crate) universe_ids: Vec<UniverseId>,
    pub(crate) preview_mode: bool,
    pub(crate) priority: u8,
    pub(crate) target_address: SocketAddr,
}

impl SacnDmxOutputInstanceDefinition {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn universe_ids(&self) -> &[UniverseId] {
        &self.universe_ids
    }

    pub fn preview_mode(&self) -> bool {
        self.preview_mode
    }

    pub fn priority(&self) -> u8 {
        self.priority
    }

    pub fn target_address(&self) -> SocketAddr {
        self.target_address
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EnttecDmxOutputDefinition {
    pub(crate) instances: Vec<EnttecDmxOutputInstanceDefinition>,
}

impl EnttecDmxOutputDefinition {
    pub fn instances(&self) -> &[EnttecDmxOutputInstanceDefinition] {
        &self.instances
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EnttecDmxOutputInstanceDefinition {
    pub(crate) universe_id: UniverseId,
    pub(crate) serial_number: String,
}
