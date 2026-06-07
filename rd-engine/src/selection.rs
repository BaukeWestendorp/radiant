use crate::{
    gdtf::{Gdtf, dmx::DmxMode},
    patch::{Fixture, FixtureId, Patch},
};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Selection {
    pub(crate) fixture_ids: Vec<FixtureId>,
}

impl Selection {
    pub(crate) fn new() -> Self {
        Self { fixture_ids: Vec::new() }
    }

    pub fn fixture_ids(&self) -> &[FixtureId] {
        &self.fixture_ids
    }

    pub fn fixtures<'a>(&'a self, patch: &'a Patch) -> impl Iterator<Item = &'a Fixture> {
        self.fixture_ids().iter().filter_map(|fixture_id| patch.fixture(fixture_id))
    }

    pub fn unique_dmx_modes<'a>(&'a self, patch: &'a Patch) -> Vec<(&'a Gdtf, &'a DmxMode)> {
        let mut unique = Vec::new();
        for fixture in self.fixtures(patch) {
            let gdtf = fixture.gdtf();
            let dmx_mode = fixture.dmx_mode();
            if !unique.iter().any(|(g, d)| *g == gdtf && *d == dmx_mode) {
                unique.push((gdtf, dmx_mode));
            }
        }
        unique
    }

    pub fn is_empty(&self) -> bool {
        self.fixture_ids.is_empty()
    }

    pub fn len(&self) -> usize {
        self.fixture_ids.len()
    }

    pub fn contains(&self, fixture: &FixtureId) -> bool {
        self.fixture_ids.contains(fixture)
    }
}
