use std::collections::HashMap;

use anyhow::Context;

use crate::{
    FixtureCollection,
    mvr_gdtf::gdtf::attr::AttributeName,
    object::{Object, ObjectId, PresetId, Slot},
    value::AttributeValue,
};

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Sequence {
    id: ObjectId,
    slot: Slot,
    name: String,
    cues: Vec<Cue>,
}

impl Sequence {
    pub fn new(id: ObjectId, slot: Slot, name: String) -> Self {
        Self { id, slot, name, cues: Vec::new() }
    }

    pub fn cues(&self) -> &[Cue] {
        &self.cues
    }

    pub fn cue(&self, index: usize) -> anyhow::Result<&Cue> {
        self.cues.get(index).with_context(|| format!("no cue at index {}", index))
    }
}

impl Object for Sequence {
    fn slot(&self) -> Slot {
        self.slot
    }

    fn id(&self) -> ObjectId {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Cue {
    name: String,
    recipes: Vec<Recipe>,
}

impl Cue {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn recipes(&self) -> &[Recipe] {
        &self.recipes
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Recipe {
    #[serde(default)]
    fixtures: FixtureCollection,
    #[serde(default)]
    content: RecipeContent,
}

impl Recipe {
    pub fn fixtures(&self) -> &FixtureCollection {
        &self.fixtures
    }

    pub fn content(&self) -> &RecipeContent {
        &self.content
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum RecipeContent {
    Static(HashMap<AttributeName, AttributeValue>),
    Preset(PresetId),
}

impl Default for RecipeContent {
    fn default() -> Self {
        Self::Static(HashMap::default())
    }
}
