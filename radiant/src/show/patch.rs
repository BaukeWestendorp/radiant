use gdtf::fixture_type::FixtureType;

mod attribute;
mod fixture;

pub use attribute::*;
pub use fixture::*;

/// Contains all information regarding the mapping of fixtures to the DMX
/// universes.
#[derive(Debug, Default)]
pub struct Patch {
    pub(crate) fixture_types: Vec<FixtureType>,
    pub(crate) fixtures: Vec<Fixture>,
}

impl Patch {
    /// Get all patched [Fixture]s.
    pub fn fixtures(&self) -> Vec<Fixture> {
        self.fixtures.clone()
    }

    /// Get all [FixtureType]s.
    pub fn fixture_types(&self) -> &[FixtureType] {
        &self.fixture_types
    }

    /// Gets the GDTF [FixtureType] this fixture is based on.
    pub fn fixture_type(&self, id: FixtureTypeId) -> Option<&FixtureType> {
        self.fixture_types.iter().find(|ft| ft.fixture_type_id == id)
    }

    /// Get a reference to a specific [Fixture] for the given [FixtureId], if it
    /// exists.
    pub fn fixture(&self, fixture_id: impl Into<FixtureId>) -> Option<Fixture> {
        let fid = fixture_id.into();
        self.fixtures.iter().find(|f| f.fid() == fid).cloned()
    }

    pub(crate) fn insert_fixture(
        &mut self,
        fid: FixtureId,
        address: dmx::Address,
        type_id: FixtureTypeId,
        dmx_mode: String,
    ) {
        self.fixtures.push(Fixture::new(fid, address, type_id, dmx_mode))
    }
}
