use crate::backend::object::{AnyPresetId, FixtureGroupId};

crate::define_object_id!(CueId);

/// A state of the stage output.
#[derive(Debug, Clone, PartialEq)]
pub struct Cue {
    id: CueId,
    pub name: String,
    pub content: Option<CueContent>,
}

impl Cue {
    pub fn new(id: impl Into<CueId>) -> Self {
        Self { id: id.into(), name: "New Cue".to_string(), content: None }
    }

    pub fn id(&self) -> CueId {
        self.id
    }
}

/// Contents of a [Cue].
#[derive(Debug, Clone, PartialEq)]
pub enum CueContent {
    Recipe(Recipe),
}

/// A list of [FixtureGroup]-[Preset] combinations.
#[derive(Debug, Clone, PartialEq)]
#[repr(transparent)]
pub struct Recipe(Vec<RecipeCombination>);

impl Default for Recipe {
    fn default() -> Self {
        Self::new()
    }
}

impl Recipe {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn combinations(&self) -> &[RecipeCombination] {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RecipeCombination {
    pub fixture_group_id: FixtureGroupId,
    pub preset_id: AnyPresetId,
}
