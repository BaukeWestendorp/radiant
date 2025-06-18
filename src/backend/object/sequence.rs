use crate::backend::object::{AnyPresetId, FixtureGroupId};

crate::define_object_id!(SequenceId);

/// A sequence of [Cue]s that can be activated using an [Executor].
#[derive(Debug, Clone, PartialEq)]
pub struct Sequence {
    pub id: SequenceId,
    pub name: String,
    pub cues: Vec<Cue>,
}

impl Sequence {
    pub fn new(id: impl Into<SequenceId>) -> Self {
        Self { id: id.into(), name: "New Sequence".to_string(), cues: Vec::new() }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn with_cue(mut self, cue: Cue) -> Self {
        self.cues.push(cue);
        self
    }

    pub fn len(&self) -> usize {
        self.cues.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn cue(&self, index: usize) -> Option<&Cue> {
        self.cues.get(index)
    }
}

/// A state of the stage output.
#[derive(Debug, Clone, PartialEq)]
pub struct Cue {
    pub name: String,
    pub content: CueContent,
}

impl Cue {
    pub fn new(content: CueContent) -> Self {
        Self { name: "New Cue".to_string(), content }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }
}

/// Contents of a [Cue].
#[derive(Debug, Clone, PartialEq)]
pub enum CueContent {
    Recipe(Recipe),
}

/// A list of [FixtureGroup]-[Preset] combinations.
#[derive(Debug, Clone, PartialEq)]
pub struct Recipe {
    pub combinations: Vec<RecipeCombination>,
}

impl Recipe {
    pub fn new() -> Self {
        Self { combinations: Vec::new() }
    }

    pub fn with_combination(
        mut self,
        fixture_group_id: FixtureGroupId,
        preset_id: impl Into<AnyPresetId>,
    ) -> Self {
        self.combinations.push(RecipeCombination { fixture_group_id, preset_id: preset_id.into() });
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RecipeCombination {
    pub fixture_group_id: FixtureGroupId,
    pub preset_id: AnyPresetId,
}
