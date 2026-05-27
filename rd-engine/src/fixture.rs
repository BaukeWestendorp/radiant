use zeevonk::project::FixtureId;

use crate::ObjectId;

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum FixtureCollection {
    Single(FixtureId),
    Multiple(Vec<FixtureId>),
    Group(ObjectId),
    Groups(Vec<ObjectId>),
    All,
}
