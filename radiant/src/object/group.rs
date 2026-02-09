use zeevonk::project::stage::FixtureId;

pub type GroupId = u32;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq)]
pub struct Group {
    pub name: String,
    pub fixture_ids: Vec<FixtureId>,
}
