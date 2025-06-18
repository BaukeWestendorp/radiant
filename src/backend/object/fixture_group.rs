use crate::backend::patch::fixture::FixtureId;

crate::define_object_id!(FixtureGroupId);

/// A list of [FixtureId]s that create a sequential group of fixtures.
#[derive(Debug, Clone, PartialEq)]
pub struct FixtureGroup {
    pub id: FixtureGroupId,
    pub name: String,
    pub fixtures: Vec<FixtureId>,
}

impl FixtureGroup {
    pub fn new(id: impl Into<FixtureGroupId>) -> Self {
        Self { id: id.into(), name: "New Fixture Group".to_string(), fixtures: Vec::new() }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn with_fixture(mut self, fixture_id: FixtureId) -> Self {
        self.fixtures.push(fixture_id);
        self
    }

    pub fn len(&self) -> usize {
        self.fixtures.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn contains(&self, fixture_id: &FixtureId) -> bool {
        self.fixtures.contains(fixture_id)
    }
}
