use gpui::{Context, Entity, prelude::*};
use zeevonk::project::FixtureId;

pub struct Selection {
    fixtures: Entity<Vec<FixtureId>>,
}

impl Selection {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self { fixtures: cx.new(|_| Vec::new()) }
    }

    pub fn fixtures(&self) -> Entity<Vec<FixtureId>> {
        self.fixtures.clone()
    }
}
