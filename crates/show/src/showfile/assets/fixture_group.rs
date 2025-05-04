use crate::patch::FixtureId;

crate::define_asset!(FixtureGroup, FixtureGroupAsset, FixtureGroupId);

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Default)]
pub struct FixtureGroup {
    pub label: String,
    pub fixtures: Vec<FixtureId>,
}
