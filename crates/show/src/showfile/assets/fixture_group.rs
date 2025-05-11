use crate::patch::FixtureId;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Default)]
pub struct FixtureGroup {
    pub label: String,
    pub fixtures: Vec<FixtureId>,
}
