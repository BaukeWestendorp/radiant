use crate::{
    backend::patch::fixture::{DmxMode, Fixture, FixtureId},
    dmx,
    error::Result,
};

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

    /// Patches a new [Fixture].
    pub fn patch_fixture(
        &mut self,
        id: FixtureId,
        address: dmx::Address,
        dmx_mode: DmxMode,
        gdtf_file_name: String,
        fixture_type: &gdtf::fixture_type::FixtureType,
    ) -> Result<()> {
        let fixture = Fixture::new(id, address, dmx_mode, gdtf_file_name, fixture_type)?;
        self.fixtures.push(fixture);
        Ok(())
    }
}
