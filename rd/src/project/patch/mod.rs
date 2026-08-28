use std::{collections::HashMap, sync::Arc};

use rd_rigger::gdtf::{FixtureTypeId, Gdtf};

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PatchConfig {
    pub fixtures: Vec<FixtureConfig>,

    #[serde(skip)]
    pub gdtfs: HashMap<FixtureTypeId, Arc<Gdtf>>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FixtureConfig {
    pub id: u32,
    pub name: String,
    pub dmx_address: rd_dmx::Address,
    pub fixture_kind: FixtureKind,
}

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FixtureKind {
    pub fixture_type_id: FixtureTypeId,
    pub dmx_mode: String,
}

impl std::fmt::Display for FixtureKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [{}]", self.fixture_type_id, self.dmx_mode)
    }
}
