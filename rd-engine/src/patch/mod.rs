use std::{collections::HashMap, sync::Arc};

use crate::mvr_gdtf::gdtf::{Gdtf, resource::ResourceKey};

mod definition;
mod fixture;

pub use definition::*;
pub use fixture::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Patch {
    fixture_definitions: Vec<FixtureDefinition>,

    fixtures: Vec<Fixture>,
    fixtures_by_id: HashMap<FixtureId, usize>,
}

impl Patch {
    pub fn new(
        definition: PatchDefinition,
        gdtfs: &HashMap<ResourceKey, Arc<Gdtf>>,
    ) -> anyhow::Result<Self> {
        let mut patch = Self {
            fixture_definitions: definition.fixtures,
            fixtures: Vec::new(),
            fixtures_by_id: HashMap::new(),
        };

        for def in patch.fixture_definitions.clone() {
            let built = fixture::builder::FixtureBuilder::new(def, gdtfs)?.build()?;
            for fixture in built {
                let fixture_id = fixture.id();
                if patch.fixtures_by_id.contains_key(&fixture_id) {
                    anyhow::bail!("Fixture with id '{}' already exists in patch", fixture_id);
                }
                let idx = patch.fixtures.len();
                patch.fixtures.push(fixture);
                patch.fixtures_by_id.insert(fixture_id, idx);
            }
        }

        Ok(patch)
    }

    pub fn fixtures(&self) -> &[Fixture] {
        &self.fixtures
    }

    pub fn fixture_definitions(&self) -> &[FixtureDefinition] {
        &self.fixture_definitions
    }

    pub fn fixture_ids(&self) -> impl Iterator<Item = &FixtureId> {
        self.fixtures_by_id.keys()
    }

    pub fn fixture(&self, fixture_id: &FixtureId) -> Option<&Fixture> {
        self.fixtures_by_id.get(fixture_id).map(|&idx| &self.fixtures[idx])
    }
}
