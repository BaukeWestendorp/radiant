use zeevonk::project::stage::FixtureId;

pub type GroupId = u32;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq)]
pub struct Group {
    name: String,
    fixture_ids: Vec<FixtureId>,
}

impl Group {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn fixture_ids(&self) -> &[FixtureId] {
        &self.fixture_ids
    }
}
