use std::{collections::HashMap, sync::Arc};

use rd_rigger::gdtf::{FixtureTypeId, Gdtf, Name};

use crate::Project;

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

impl FixtureKind {
    pub fn display(&self, project: &Project) -> String {
        let Some(gdtf) = project.patch.gdtfs.get(&self.fixture_type_id) else {
            return format!("{} [{}]", self.fixture_type_id, self.dmx_mode);
        };

        let Some(dmx_mode) = gdtf.dmx_mode(&Name::new(&self.dmx_mode)) else {
            return format!("{} {} [{}]", gdtf.manufacturer(), gdtf.name(), self.dmx_mode);
        };

        format!(
            "{} {} [{}, {}ch]",
            gdtf.manufacturer(),
            gdtf.name(),
            dmx_mode.name(),
            dmx_mode.max_channel_offset()
        )
    }
}

impl std::fmt::Display for FixtureKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [{}]", self.fixture_type_id, self.dmx_mode)
    }
}
