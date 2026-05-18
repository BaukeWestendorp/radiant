use std::{collections::HashMap, ops, path::PathBuf};

use anyhow::Context as _;
use uuid::Uuid;
use zeevonk::project::FixtureId;

use crate::engine::{FixtureCollection, Object, ObjectId, ObjectReference, Parameter, SlotId};

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

impl ops::Deref for RecipeId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for RecipeId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum RecipeContent {
    Effect { effect: ObjectReference, options: HashMap<String, EffectOptionValue> },
    Static(Vec<Parameter>),
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum EffectOptionValue {
    Nil,
    Boolean(bool),
    Integer(i64),
    Number(f64),
    String(String),
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

    pub fn load_lua_source(&self, showfile_path: Option<&PathBuf>) -> anyhow::Result<String> {
        let showfile_path = showfile_path.context("no showfile to find lua files in")?;
        let effect_path = showfile_path.join("obj/effects/").join(&self.file_name);
        let source = std::fs::read_to_string(&effect_path)?;
        Ok(source)
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
