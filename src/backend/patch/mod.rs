use crate::backend::patch::fixture::{Fixture, FixtureId};

pub mod attr;
pub mod fixture;

/// Contains all information regarding the mapping of fixtures to the DMX universes.
#[derive(Default)]
pub struct Patch {
    fixtures: Vec<Fixture>,
}

impl Patch {
    /// Get all patched [Fixture]s.
    pub fn fixtures(&self) -> &[Fixture] {
        &self.fixtures
    }

    /// Get a specific [Fixture] for the given [FixtureId], if it exists.
    pub fn fixture(&self, fixture_id: &FixtureId) -> Option<&Fixture> {
        self.fixtures.iter().find(|f| f.id == *fixture_id)
    }
}
