use crate::object::{AnyPresetId, FixtureGroupId};

super::define_object_id!(CueId);

/// A state of the stage output.
#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Deserialize)]
pub struct Cue {
    id: CueId,
    pub(crate) name: String,
    pub(crate) recipes: Vec<Recipe>,
}

impl Cue {
    /// Creates a new [Cue] with the specified id.
    pub fn new(id: impl Into<CueId>) -> Self {
        Self { id: id.into(), name: "New Cue".to_string(), recipes: Vec::new() }
    }

    /// Returns this cue's id.
    pub fn id(&self) -> CueId {
        self.id
    }

    /// Returns the name of this cue.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the recipes contained in this cue.
    pub fn recipes(&self) -> &[Recipe] {
        &self.recipes
    }
}

/// A list of fixture group to content combinations.
#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Deserialize)]
pub struct Recipe {
    /// The id of the [FixtureGroup][crate::object::FixtureGroup] this recipe
    /// applies to.
    pub fixture_group: FixtureGroupId,
    /// The content of this recipe.
    pub content: RecipeContent,
}

/// Represents the different types of content that can be included in a recipe.
#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum RecipeContent {
    /// A preset to be applied.
    Preset(AnyPresetId),
}
