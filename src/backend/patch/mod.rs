use crate::backend::patch::fixture::{Fixture, FixtureId};

pub mod attr;
pub mod fixture;

#[derive(Default)]
pub struct Patch {
    fixtures: Vec<Fixture>,
}

impl Patch {
    pub fn fixtures(&self) -> &[Fixture] {
        &self.fixtures
    }

    pub fn fixture(&self, fixture_id: &FixtureId) -> Option<&Fixture> {
        self.fixtures.iter().find(|f| f.id == *fixture_id)
    }
}
