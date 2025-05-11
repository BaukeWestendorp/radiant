use crate::patch::FixtureId;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Default)]
pub struct FixtureGroup {
    pub fixtures: Vec<FixtureId>,
}
