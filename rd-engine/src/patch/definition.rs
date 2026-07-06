use crate::{
    dmx::Address,
    patch::{FixtureIdPart, FixtureKind},
};

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
    pub(crate) fixture_kind: FixtureKind,
}

impl FixtureDefinition {
    pub fn new(
        id: FixtureIdPart,
        name: String,
        dmx_address: Address,
        fixture_kind: FixtureKind,
    ) -> Self {
        Self { id, name, dmx_address, fixture_kind }
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

    pub fn fixture_kind(&self) -> &FixtureKind {
        &self.fixture_kind
    }
}
