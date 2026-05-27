use zeevonk::project::{FixtureId, Stage};

use crate::{ObjectId, Objects};

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum FixtureCollection {
    Single(FixtureId),
    Multiple(Vec<FixtureId>),
    Group(ObjectId),
    Groups(Vec<ObjectId>),
    All,
}

impl Default for FixtureCollection {
    fn default() -> Self {
        Self::Multiple(Vec::default())
    }
}

impl FixtureCollection {
    pub fn fixture_ids<'a>(
        &'a self,
        objects: &'a Objects,
        stage: &'a Stage,
    ) -> anyhow::Result<Box<dyn Iterator<Item = &'a FixtureId> + 'a>> {
        match self {
            FixtureCollection::Single(fixture_id) => Ok(Box::new(std::iter::once(fixture_id))),
            FixtureCollection::Multiple(fixture_ids) => Ok(Box::new(fixture_ids.iter())),
            FixtureCollection::Group(object_id) => {
                let group = objects
                    .groups()
                    .get_by_object_id(object_id)
                    .map_err(|_| anyhow::anyhow!("group object not found: {:?}", object_id))?;
                Ok(Box::new(group.fixture_ids().iter()))
            }
            FixtureCollection::Groups(object_ids) => {
                let mut all = Vec::new();
                for object_id in object_ids {
                    let group = objects
                        .groups()
                        .get_by_object_id(object_id)
                        .map_err(|_| anyhow::anyhow!("group object not found: {:?}", object_id))?;
                    all.extend(group.fixture_ids());
                }
                Ok(Box::new(all.into_iter()))
            }
            FixtureCollection::All => Ok(Box::new(stage.fixtures().keys())),
        }
    }
}
