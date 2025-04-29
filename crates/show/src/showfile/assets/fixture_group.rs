use crate::patch::FixtureId;

crate::define_asset!(FixtureGroup, FixtureGroupAsset, FixtureGroupId);

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Default)]
pub struct FixtureGroup {
    label: String,
    fixtures: Vec<FixtureId>,
}
