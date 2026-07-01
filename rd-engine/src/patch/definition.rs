use crate::{dmx::Address, patch::FixtureIdPart};

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PatchDefinition {
    pub(crate) fixtures: Vec<FixtureDefinition>,
}

impl PatchDefinition {
    pub fn fixtures(&self) -> &[FixtureDefinition] {
        &self.fixtures
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq)]
pub struct FixtureDefinition {
    pub(crate) id: FixtureIdPart,
    pub(crate) name: String,
    pub(crate) dmx_address: Address,
    pub(crate) gdtf_file_name: String,
    pub(crate) gdtf_dmx_mode: String,
}

impl FixtureDefinition {
    pub fn new(
        id: FixtureIdPart,
        name: String,
        dmx_address: Address,
        gdtf_file_name: String,
        gdtf_dmx_mode: String,
    ) -> Self {
        Self { id, name, dmx_address, gdtf_file_name, gdtf_dmx_mode }
    }

    pub fn id(&self) -> FixtureIdPart {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn dmx_address(&self) -> Address {
        self.dmx_address
    }

    pub fn gdtf_file_name(&self) -> &str {
        &self.gdtf_file_name
    }

    pub fn gdtf_dmx_mode(&self) -> &str {
        &self.gdtf_dmx_mode
    }
}
