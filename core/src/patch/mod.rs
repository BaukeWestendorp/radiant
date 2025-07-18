//! Patch data types and fixture mapping.
//!
//! This module provides types and logic for working with the show's
//! patch, including fixture mapping, attributes, and DMX addressing.
//! It is responsible for representing and managing the structure of the
//! lighting patch, enabling dynamic reconfiguration and querying of fixtures.

pub use attr::*;
pub use fixture::*;

mod attr;
mod fixture;

/// Contains all information regarding the mapping of fixtures to the DMX
/// universes.
#[derive(Debug, Default, Clone)]
pub struct Patch {
    pub(crate) gdtfs: Vec<String>,
    pub(crate) fixtures: Vec<Fixture>,
}

impl Patch {
    /// Get all patched [Fixture]s.
    pub fn fixtures(&self) -> &[Fixture] {
        &self.fixtures
    }

    /// Get the names of all loaded GDTF files.
    pub fn gdtfs(&self) -> &[String] {
        &self.gdtfs
    }

    /// Get a reference to a specific [Fixture] for the given [FixtureId], if it
    /// exists.
    pub fn fixture(&self, fixture_id: impl Into<FixtureId>) -> Option<&Fixture> {
        let fid = fixture_id.into();
        self.fixtures.iter().find(|f| f.id() == fid)
    }

    /// Get a mutable reference to a specific [Fixture] for the given
    /// [FixtureId], if it exists.
    pub(crate) fn fixture_mut(&mut self, fixture_id: impl Into<FixtureId>) -> Option<&mut Fixture> {
        let fid = fixture_id.into();
        self.fixtures.iter_mut().find(|f| f.id() == fid)
    }

    pub(crate) fn remove_fixture(&mut self, id: FixtureId) {
        self.fixtures.retain(|f| f.id() != id);
    }
}
