use std::collections::HashSet;

use crate::patch::{Attribute, FixtureId, Patch};

super::define_object_id!(FixtureGroupId);

/// A list of [FixtureId]s that create a sequential group of fixtures.
///
/// Fixture groups allow for organizing and controlling multiple fixtures as a
/// single unit, providing convenient access to collections of related lighting
/// fixtures.
#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Deserialize)]
pub struct FixtureGroup {
    id: FixtureGroupId,
    pub(crate) name: String,
    pub(crate) fixtures: Vec<FixtureId>,
}

impl FixtureGroup {
    /// Creates a new [FixtureGroup] with the specified id.
    ///
    /// The group is initialized with a default name and an empty fixture list.
    pub fn new(id: impl Into<FixtureGroupId>) -> Self {
        Self { id: id.into(), name: "New Fixture Group".to_string(), fixtures: Vec::new() }
    }

    /// Returns this fixture group's unique id.
    pub fn id(&self) -> FixtureGroupId {
        self.id
    }

    /// Returns the name of this fixture group.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns a slice of all [FixtureId]s in this group.
    ///
    /// The fixtures are returned in the order they were added to the group.
    pub fn fixtures(&self) -> &[FixtureId] {
        &self.fixtures
    }

    /// Returns the number of fixtures in this group.
    pub fn len(&self) -> usize {
        self.fixtures.len()
    }

    /// Returns `true` if the group contains no fixtures.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns `true` if the group contains the specified fixture.
    pub fn contains(&self, fixture_id: &FixtureId) -> bool {
        self.fixtures.contains(fixture_id)
    }

    /// Returns all unique attributes that are supported by each fixture type in
    /// the [FixtureGroup].
    pub fn supported_attributes<'a>(&self, patch: &'a Patch) -> Vec<&'a Attribute> {
        let mut attrs = HashSet::new();

        for fid in self.fixtures() {
            let Some(fixture) = patch.fixture(*fid) else { continue };
            attrs.extend(fixture.supported_attributes());
        }

        let mut attrs = attrs.into_iter().collect::<Vec<_>>();
        attrs.sort();
        attrs
    }
}
