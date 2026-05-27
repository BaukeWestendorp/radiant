use zeevonk::project::FixtureId;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Selection {
    pub(crate) fixtures: Vec<FixtureId>,
}

impl Selection {
    pub(crate) fn new() -> Self {
        Self { fixtures: Vec::new() }
    }

    pub fn fixtures(&self) -> &[FixtureId] {
        &self.fixtures
    }

    pub fn is_empty(&self) -> bool {
        self.fixtures.is_empty()
    }

    pub fn len(&self) -> usize {
        self.fixtures.len()
    }

    pub fn contains(&self, fixture: &FixtureId) -> bool {
        self.fixtures.contains(fixture)
    }
}
