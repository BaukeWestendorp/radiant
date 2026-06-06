pub mod cmd;
pub mod dmx;
pub mod event;
pub mod object;
pub mod output;
pub mod patch;
pub mod selection;
pub mod trigger;
pub mod value;

mod engine;
mod mvr_gdtf;
mod pipeline;
mod project;

pub use engine::*;
pub use mvr_gdtf::*;
pub use project::*;

use std::str;

use crate::{
    object::{ObjectId, Objects},
    patch::{FixtureId, Patch},
};

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
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
        patch: &'a Patch,
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
            FixtureCollection::All => Ok(Box::new(patch.fixture_ids())),
        }
    }
}
