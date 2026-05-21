use uuid::Uuid;

use crate::{FixtureCollection, Object, ObjectId, ObjectKind, ObjectReference, Parameter, SlotId};

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
    name: String,

    recipes: Vec<Recipe>,
}

impl Cue {
    pub fn new() -> Self {
        Self { name: "Cue".to_string(), recipes: Vec::new() }
    }

    pub fn recipes(&self) -> &[Recipe] {
        &self.recipes
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Recipe {
    #[serde(skip, default = "RecipeId::new")]
    id: RecipeId,

    fixture_collection: FixtureCollection,
    content: RecipeContent,
}

impl Recipe {
    pub fn new(fixture_collection: FixtureCollection, content: RecipeContent) -> Self {
        Self { id: RecipeId::new(), fixture_collection, content }
    }

    pub fn id(&self) -> RecipeId {
        self.id
    }

    pub fn fixture_collection(&self) -> &FixtureCollection {
        &self.fixture_collection
    }

    pub fn content(&self) -> &RecipeContent {
        &self.content
    }
}

/// Used to identify effect runners.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(derive_more::Deref, derive_more::DerefMut)]
pub struct RecipeId(pub Uuid);

impl RecipeId {
    pub fn new() -> Self {
        RecipeId(Uuid::new_v4())
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }

    pub fn into_inner(self) -> Uuid {
        self.0
    }
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum RecipeContent {
    Effect { effect: ObjectReference },
    Static(Vec<Parameter>),
}
