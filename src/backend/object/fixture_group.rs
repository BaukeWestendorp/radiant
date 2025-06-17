crate::define_object_id!(FixtureGroupId);

#[derive(Debug, Clone, PartialEq)]
pub struct FixtureGroup {
    pub id: FixtureGroupId,
    pub name: String,
}

impl FixtureGroup {
    pub fn new(id: impl Into<FixtureGroupId>) -> Self {
        Self { id: id.into(), name: "New Fixture Group".to_string() }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }
}
