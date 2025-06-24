use crate::backend::object::{AnyPresetId, FixtureGroupId};

crate::define_object_id!(CueId);

/// A state of the stage output.
#[derive(Debug, Clone, PartialEq)]
pub struct Cue {
    id: CueId,
    pub name: String,
    pub(in crate::backend) recipes: Vec<Recipe>,
}

impl Cue {
    pub fn new(id: impl Into<CueId>) -> Self {
        Self { id: id.into(), name: "New Cue".to_string(), recipes: Vec::new() }
    }

    pub fn id(&self) -> CueId {
        self.id
    }

    pub fn recipes(&self) -> &[Recipe] {
        &self.recipes
    }
}

/// A list of [FixtureGroup]-[Recipe] combinations.
#[derive(Debug, Clone, PartialEq)]
pub struct Recipe {
    pub fixture_group_id: FixtureGroupId,
    pub content: RecipeContent,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RecipeContent {
    Preset(AnyPresetId),
}
