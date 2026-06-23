use std::{collections::HashMap, sync::Arc};

use crate::mvr_gdtf::gdtf::{Gdtf, resource::ResourceKey};

mod definition;
mod fixture;

pub use definition::*;
pub use fixture::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Patch {
    definition: PatchDefinition,

    fixtures: Vec<Fixture>,
    fixtures_by_id: HashMap<FixtureId, usize>,

    gdtfs: HashMap<ResourceKey, Arc<Gdtf>>,
}

impl Patch {
    pub fn new(
        definition: PatchDefinition,
        gdtfs: HashMap<ResourceKey, Arc<Gdtf>>,
    ) -> anyhow::Result<Self> {
        let mut patch =
            Self { definition, fixtures: Vec::new(), fixtures_by_id: HashMap::new(), gdtfs };

        for def in &patch.definition.fixtures {
            let built =
                fixture::builder::FixtureBuilder::new(def.clone(), &patch.gdtfs)?.build()?;
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

    pub fn definition(&self) -> &PatchDefinition {
        &self.definition
    }

    pub fn fixtures(&self) -> &[Fixture] {
        &self.fixtures
    }

    pub fn root_fixtures(&self) -> impl Iterator<Item = &Fixture> {
        self.fixtures.iter().filter(|f| f.id().is_root())
    }

    pub fn fixture_ids(&self) -> impl Iterator<Item = &FixtureId> {
        self.fixtures_by_id.keys()
    }

    pub fn fixture(&self, fixture_id: &FixtureId) -> Option<&Fixture> {
        self.fixtures_by_id.get(fixture_id).map(|&idx| &self.fixtures[idx])
    }

    pub fn gdtfs(&self) -> &HashMap<ResourceKey, Arc<Gdtf>> {
        &self.gdtfs
    }
}
