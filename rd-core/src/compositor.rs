use std::sync::Arc;

use zeevonk::{
    project::{FixtureId, IntoFixtureId, IntoFixtureIds},
    value::AttributeValues,
};

use crate::{object::ObjectRegistry, programmer::Programmer};

pub struct Compositor {
    highlighted_fixtures: Vec<FixtureId>,

    programmer: Arc<Programmer>,
}

impl Compositor {
    pub fn new(programmer: Arc<Programmer>) -> Self {
        Self { highlighted_fixtures: Vec::new(), programmer }
    }

    /// Adds a fixture to the highlighted_fixtures list.
    pub fn highlight_fixture(&mut self, fixture_id: impl IntoFixtureId) {
        let Some(fixture_id) = fixture_id.into_fixture_id() else { return };
        if !self.highlighted_fixtures.contains(&fixture_id) {
            self.highlighted_fixtures.push(fixture_id);
        }
    }

    /// Adds multiple fixtures to the highlighted_fixtures list.
    pub fn highlight_fixtures(&mut self, fixture_ids: impl IntoFixtureIds) {
        for fixture_id in fixture_ids.into_fixture_ids() {
            if !self.highlighted_fixtures.contains(&fixture_id) {
                self.highlighted_fixtures.push(fixture_id);
            }
        }
    }

    /// Removes a fixture from the highlighted_fixtures list.
    pub fn unhighlight_fixture(&mut self, fixture_id: impl IntoFixtureId) {
        let Some(fixture_id) = fixture_id.into_fixture_id() else { return };
        self.highlighted_fixtures.retain(|id| id != &fixture_id);
    }

    /// Removes multiple fixtures from the highlighted_fixtures list.
    pub fn unhighlight_fixtures(&mut self, fixture_ids: impl IntoFixtureIds) {
        let fixture_ids = fixture_ids.into_fixture_ids().collect::<Vec<_>>();
        self.highlighted_fixtures.retain(|id| !fixture_ids.contains(id));
    }

    /// Checks if a fixture is highlighted.
    pub fn is_fixture_highlighted(&self, fixture_id: impl IntoFixtureId) -> bool {
        let Some(fixture_id) = fixture_id.into_fixture_id() else { return false };
        self.highlighted_fixtures.contains(&fixture_id)
    }

    /// Returns a slice of all highlighted fixtures.
    pub fn highlighted_fixtures(&self) -> &[FixtureId] {
        &self.highlighted_fixtures
    }

    pub fn compose<'a>(&'a self, _objects: &Arc<ObjectRegistry>) -> Composition<'a> {
        let attribute_values = self.programmer.programmed_values().clone();

        Composition { attribute_values, highlighted_fixtures: &self.highlighted_fixtures }
    }
}

pub struct Composition<'a> {
    pub attribute_values: AttributeValues,
    pub highlighted_fixtures: &'a [FixtureId],
}
