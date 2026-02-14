use zeevonk::project::FixtureId;

use crate::{
    object::{FixtureCollection, Object, ObjectId, ObjectReference, SlotId},
    parameter::Parameter,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum ObjectKind {
    CueList,
    Group,
    Effect,
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CueList {
    id: ObjectId,
    slot_id: SlotId,
    name: String,

    cues: Vec<Cue>,
}

impl CueList {
    pub fn cues(&self) -> &[Cue] {
        &self.cues
    }
}

impl Object for CueList {
    fn kind() -> ObjectKind {
        ObjectKind::CueList
    }

    fn id(&self) -> ObjectId {
        self.id
    }

    fn slot_id(&self) -> SlotId {
        self.slot_id
    }

    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Cue {
    recipes: Vec<Recipe>,
}

impl Cue {
    pub fn new() -> Self {
        Self { recipes: Vec::new() }
    }

    pub fn recipes(&self) -> &[Recipe] {
        &self.recipes
    }
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Recipe {
    fixture_collection: FixtureCollection,
    content: RecipeContent,
}

impl Recipe {
    pub fn new(fixture_collection: FixtureCollection, content: RecipeContent) -> Self {
        Self { fixture_collection, content }
    }

    pub fn fixture_collection(&self) -> &FixtureCollection {
        &self.fixture_collection
    }

    pub fn content(&self) -> &RecipeContent {
        &self.content
    }
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum RecipeContent {
    Effect(ObjectReference),
    Static(Vec<Parameter>),
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Effect {
    id: ObjectId,
    slot_id: SlotId,
    name: String,

    file_name: String,
}

impl Effect {
    pub fn file_name(&self) -> &str {
        &self.file_name
    }
}

impl Object for Effect {
    fn kind() -> ObjectKind {
        ObjectKind::Effect
    }

    fn id(&self) -> ObjectId {
        self.id
    }

    fn slot_id(&self) -> SlotId {
        self.slot_id
    }

    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Group {
    id: ObjectId,
    slot_id: SlotId,
    name: String,

    fixture_ids: Vec<FixtureId>,
}

impl Group {
    pub fn fixture_ids(&self) -> &[FixtureId] {
        &self.fixture_ids
    }
}

impl Object for Group {
    fn kind() -> ObjectKind {
        ObjectKind::Group
    }

    fn id(&self) -> ObjectId {
        self.id
    }

    fn slot_id(&self) -> SlotId {
        self.slot_id
    }

    fn name(&self) -> &str {
        &self.name
    }
}
