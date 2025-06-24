use crate::backend::patch::fixture::{Fixture, FixtureId};

pub mod attr;
pub mod fixture;

/// Contains all information regarding the mapping of fixtures to the DMX universes.
#[derive(Debug, Default, Clone)]
pub struct Patch {
    pub(in crate::backend) gdtf_file_names: Vec<String>,
    pub(in crate::backend) fixtures: Vec<Fixture>,
}

impl Patch {
    /// Get all patched [Fixture]s.
    pub fn fixtures(&self) -> &[Fixture] {
        &self.fixtures
    }

    pub fn gdtf_file_names(&self) -> &[String] {
        &self.gdtf_file_names
    }

    /// Get a reference to a specific [Fixture] for the given [FixtureId], if it exists.
    pub fn fixture(&self, fixture_id: impl Into<FixtureId>) -> Option<&Fixture> {
        let fid = fixture_id.into();
        self.fixtures.iter().find(|f| f.id() == fid)
    }

    /// Get a mutable reference to a specific [Fixture] for the given [FixtureId], if it exists.
    pub(in crate::backend) fn fixture_mut(
        &mut self,
        fixture_id: impl Into<FixtureId>,
    ) -> Option<&mut Fixture> {
        let fid = fixture_id.into();
        self.fixtures.iter_mut().find(|f| f.id() == fid)
    }

    pub(in crate::backend) fn remove_fixture(&mut self, id: FixtureId) {
        self.fixtures.retain(|f| f.id() != id);
    }
}
