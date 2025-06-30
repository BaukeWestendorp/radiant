use crate::FixtureId;

crate::define_object_id!(FixtureGroupId);

/// A list of [FixtureId]s that create a sequential group of fixtures.
#[derive(Debug, Clone, PartialEq)]
pub struct FixtureGroup {
    id: FixtureGroupId,
    pub name: String,
    pub(crate) fixtures: Vec<FixtureId>,
}

impl FixtureGroup {
    pub fn new(id: impl Into<FixtureGroupId>) -> Self {
        Self { id: id.into(), name: "New Fixture Group".to_string(), fixtures: Vec::new() }
    }

    pub fn id(&self) -> FixtureGroupId {
        self.id
    }

    pub fn fixtures(&self) -> &[FixtureId] {
        &self.fixtures
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
