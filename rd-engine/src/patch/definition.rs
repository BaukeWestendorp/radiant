use crate::{
    dmx::Address,
    patch::{FixtureIdPart, FixtureKind},
};

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PatchDefinition {
    pub fixtures: Vec<FixtureDefinition>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq)]
pub struct FixtureDefinition {
    pub id: FixtureIdPart,
    pub name: String,
    pub dmx_address: Address,
    pub fixture_kind: FixtureKind,
}
