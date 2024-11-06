use std::fs;
use std::path::Path;

use crate::effect_graph::EffectGraph;
use crate::fixture::{Fixture, FixtureId};

pub mod effect_graph;
pub mod fixture;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Show {
    fixtures: Vec<Fixture>,

    effect_graph: EffectGraph,
}

impl Show {
    pub fn fixtures(&self) -> impl Iterator<Item = &Fixture> {
        self.fixtures.iter()
    }

    pub fn fixture(&self, id: &FixtureId) -> Option<&Fixture> {
        self.fixtures.iter().find(|f| f.id() == id)
    }

    pub fn effect_graph(&self) -> &EffectGraph {
        &self.effect_graph
    }

    pub fn effect_graph_mut(&mut self) -> &mut EffectGraph {
        &mut self.effect_graph
    }

    pub fn load_from_file(path: &Path) -> anyhow::Result<Self> {
        let show_json = fs::read_to_string(path)?;
        let show: Self = serde_json::from_str(&show_json)?;
        Ok(show)
    }

    pub fn save_to_file(&self, path: &Path) -> anyhow::Result<()> {
        let show_json = serde_json::to_string_pretty(self)?;

        fs::write(path, show_json)?;

        Ok(())
    }
}

impl Default for Show {
    fn default() -> Self {
        Self {
            fixtures: vec![Fixture::new(FixtureId(0))],
            effect_graph: EffectGraph::default(),
        }
    }
}

#[derive(Clone, Default)]
pub struct FixtureGroup {
    fixtures: Vec<FixtureId>,
}

impl FixtureGroup {
    pub fn new(fixtures: Vec<FixtureId>) -> Self {
        Self { fixtures }
    }

    pub fn fixtures(&self) -> &[FixtureId] {
        &self.fixtures
    }

    pub fn push_fixture(&mut self, fixture: FixtureId) {
        self.fixtures.push(fixture);
    }

    pub fn len(&self) -> usize {
        self.fixtures.len()
    }
}
