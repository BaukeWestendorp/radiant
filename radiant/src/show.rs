//! Show state and in-memory representation.
//!
//! Show state and in-memory representation.
//!
//! This module defines the [Show] type, which aggregates all objects, patch
//! data, and runtime state for a loaded show.

use std::collections::HashMap;
use std::path::PathBuf;

use crate::object::{
    AnyPreset, AnyPresetId, BeamPreset, BeamPresetId, ColorPreset, ColorPresetId, ControlPreset,
    ControlPresetId, Cue, CueId, DimmerPreset, DimmerPresetId, Executor, ExecutorId, FixtureGroup,
    FixtureGroupId, FocusPreset, FocusPresetId, GoboPreset, GoboPresetId, PositionPreset,
    PositionPresetId, PresetContent, Sequence, SequenceId, ShapersPreset, ShapersPresetId,
    VideoPreset, VideoPresetId,
};
use crate::patch::{Attribute, AttributeValue, FixtureId, Patch};

/// Contains the state of the entire show. This includes the programmer, patch
/// and objects.
#[derive(Debug, Clone, Default)]
pub struct Show {
    path: Option<PathBuf>,

    pub(crate) patch: Patch,

    pub(crate) programmer: Programmer,

    pub(crate) fixture_groups: HashMap<FixtureGroupId, FixtureGroup>,
    pub(crate) executors: HashMap<ExecutorId, Executor>,
    pub(crate) sequences: HashMap<SequenceId, Sequence>,
    pub(crate) cues: HashMap<CueId, Cue>,

    pub(crate) dimmer_presets: HashMap<DimmerPresetId, DimmerPreset>,
    pub(crate) position_presets: HashMap<PositionPresetId, PositionPreset>,
    pub(crate) gobo_presets: HashMap<GoboPresetId, GoboPreset>,
    pub(crate) color_presets: HashMap<ColorPresetId, ColorPreset>,
    pub(crate) beam_presets: HashMap<BeamPresetId, BeamPreset>,
    pub(crate) focus_presets: HashMap<FocusPresetId, FocusPreset>,
    pub(crate) control_presets: HashMap<ControlPresetId, ControlPreset>,
    pub(crate) shapers_presets: HashMap<ShapersPresetId, ShapersPreset>,
    pub(crate) video_presets: HashMap<VideoPresetId, VideoPreset>,

    pub(crate) selected_fixture_ids: Vec<FixtureId>,
}

impl Show {
    /// Creates a new [Show].
    pub fn new(path: Option<PathBuf>) -> Self {
        Self { path, ..Default::default() }
    }

    /// The path at which the [Showfile][crate::showfile::Showfile] is saved.
    /// Will be `None` if it has not been saved yet.
    pub fn path(&self) -> Option<&PathBuf> {
        self.path.as_ref()
    }

    /// Gets this shows [Patch].
    pub fn patch(&self) -> &Patch {
        &self.patch
    }

    /// Gets this shows [Programmer].
    pub fn programmer(&self) -> &Programmer {
        &self.programmer
    }

    /// Gets a [FixtureGroup].
    pub fn fixture_group(&self, id: impl Into<FixtureGroupId>) -> Option<&FixtureGroup> {
        self.fixture_groups.get(&id.into())
    }

    /// Gets a mutable [FixtureGroup].
    pub fn fixture_group_mut(
        &mut self,
        id: impl Into<FixtureGroupId>,
    ) -> Option<&mut FixtureGroup> {
        self.fixture_groups.get_mut(&id.into())
    }

    /// Gets an iterator all [FixtureGroup]s.
    pub fn fixture_groups(&self) -> impl IntoIterator<Item = &FixtureGroup> {
        self.fixture_groups.values()
    }

    /// Gets an [Executor].
    pub fn executor(&self, id: impl Into<ExecutorId>) -> Option<&Executor> {
        self.executors.get(&id.into())
    }

    /// Gets a mutable [Executor].
    pub fn executor_mut(&mut self, id: impl Into<ExecutorId>) -> Option<&mut Executor> {
        self.executors.get_mut(&id.into())
    }

    /// Gets an iterator all [Executor]s.
    pub fn executors(&self) -> impl IntoIterator<Item = &Executor> {
        self.executors.values()
    }

    /// Gets a [Sequence].
    pub fn sequence(&self, id: impl Into<SequenceId>) -> Option<&Sequence> {
        self.sequences.get(&id.into())
    }

    /// Gets a mutable [Sequence].
    pub fn sequence_mut(&mut self, id: impl Into<SequenceId>) -> Option<&mut Sequence> {
        self.sequences.get_mut(&id.into())
    }

    /// Gets an iterator all [Sequence]s.
    pub fn sequences(&self) -> impl IntoIterator<Item = &Sequence> {
        self.sequences.values()
    }

    /// Gets a [Cue].
    pub fn cue(&self, id: impl Into<CueId>) -> Option<&Cue> {
        self.cues.get(&id.into())
    }

    /// Gets a mutable [Cue].
    pub fn cue_mut(&mut self, id: impl Into<CueId>) -> Option<&mut Cue> {
        self.cues.get_mut(&id.into())
    }

    /// Gets an iterator all [Cue]s.
    pub fn cues(&self) -> impl IntoIterator<Item = &Cue> {
        self.cues.values()
    }

    /// Gets any kind of preset from its corresponding id.
    pub fn preset(&self, preset_id: impl Into<AnyPresetId>) -> Option<AnyPreset> {
        match preset_id.into() {
            AnyPresetId::Dimmer(id) => Some(self.dimmer_presets.get(&id)?.clone().into_any()),
            AnyPresetId::Position(id) => Some(self.position_presets.get(&id)?.clone().into_any()),
            AnyPresetId::Gobo(id) => Some(self.gobo_presets.get(&id)?.clone().into_any()),
            AnyPresetId::Color(id) => Some(self.color_presets.get(&id)?.clone().into_any()),
            AnyPresetId::Beam(id) => Some(self.beam_presets.get(&id)?.clone().into_any()),
            AnyPresetId::Focus(id) => Some(self.focus_presets.get(&id)?.clone().into_any()),
            AnyPresetId::Control(id) => Some(self.control_presets.get(&id)?.clone().into_any()),
            AnyPresetId::Shapers(id) => Some(self.shapers_presets.get(&id)?.clone().into_any()),
            AnyPresetId::Video(id) => Some(self.video_presets.get(&id)?.clone().into_any()),
        }
    }

    pub(crate) fn preset_content_mut(
        &mut self,
        preset_id: impl Into<AnyPresetId>,
    ) -> Option<&mut PresetContent> {
        match preset_id.into() {
            AnyPresetId::Dimmer(id) => Some(&mut self.dimmer_presets.get_mut(&id)?.content),
            AnyPresetId::Position(id) => Some(&mut self.position_presets.get_mut(&id)?.content),
            AnyPresetId::Gobo(id) => Some(&mut self.gobo_presets.get_mut(&id)?.content),
            AnyPresetId::Color(id) => Some(&mut self.color_presets.get_mut(&id)?.content),
            AnyPresetId::Beam(id) => Some(&mut self.beam_presets.get_mut(&id)?.content),
            AnyPresetId::Focus(id) => Some(&mut self.focus_presets.get_mut(&id)?.content),
            AnyPresetId::Control(id) => Some(&mut self.control_presets.get_mut(&id)?.content),
            AnyPresetId::Shapers(id) => Some(&mut self.shapers_presets.get_mut(&id)?.content),
            AnyPresetId::Video(id) => Some(&mut self.video_presets.get_mut(&id)?.content),
        }
    }

    pub(crate) fn preset_name_mut(
        &mut self,
        preset_id: impl Into<AnyPresetId>,
    ) -> Option<&mut String> {
        match preset_id.into() {
            AnyPresetId::Dimmer(id) => Some(&mut self.dimmer_presets.get_mut(&id)?.name),
            AnyPresetId::Position(id) => Some(&mut self.position_presets.get_mut(&id)?.name),
            AnyPresetId::Gobo(id) => Some(&mut self.gobo_presets.get_mut(&id)?.name),
            AnyPresetId::Color(id) => Some(&mut self.color_presets.get_mut(&id)?.name),
            AnyPresetId::Beam(id) => Some(&mut self.beam_presets.get_mut(&id)?.name),
            AnyPresetId::Focus(id) => Some(&mut self.focus_presets.get_mut(&id)?.name),
            AnyPresetId::Control(id) => Some(&mut self.control_presets.get_mut(&id)?.name),
            AnyPresetId::Shapers(id) => Some(&mut self.shapers_presets.get_mut(&id)?.name),
            AnyPresetId::Video(id) => Some(&mut self.video_presets.get_mut(&id)?.name),
        }
    }

    /// Gets a [DimmerPreset].
    pub fn preset_dimmer(&self, id: impl Into<DimmerPresetId>) -> Option<&DimmerPreset> {
        self.dimmer_presets.get(&id.into())
    }

    /// Gets a [PositionPreset].
    pub fn preset_position(&self, id: impl Into<PositionPresetId>) -> Option<&PositionPreset> {
        self.position_presets.get(&id.into())
    }

    /// Gets a [GoboPreset].
    pub fn preset_gobo(&self, id: impl Into<GoboPresetId>) -> Option<&GoboPreset> {
        self.gobo_presets.get(&id.into())
    }

    /// Gets a [ColorPreset].
    pub fn preset_color(&self, id: impl Into<ColorPresetId>) -> Option<&ColorPreset> {
        self.color_presets.get(&id.into())
    }

    /// Gets a [BeamPreset].
    pub fn preset_beam(&self, id: impl Into<BeamPresetId>) -> Option<&BeamPreset> {
        self.beam_presets.get(&id.into())
    }

    /// Gets a [FocusPreset].
    pub fn preset_focus(&self, id: impl Into<FocusPresetId>) -> Option<&FocusPreset> {
        self.focus_presets.get(&id.into())
    }

    /// Gets a [ControlPreset].
    pub fn preset_control(&self, id: impl Into<ControlPresetId>) -> Option<&ControlPreset> {
        self.control_presets.get(&id.into())
    }

    /// Gets a [ShapersPreset].
    pub fn preset_shapers(&self, id: impl Into<ShapersPresetId>) -> Option<&ShapersPreset> {
        self.shapers_presets.get(&id.into())
    }

    /// Gets a [VideoPreset].
    pub fn preset_video(&self, id: impl Into<VideoPresetId>) -> Option<&VideoPreset> {
        self.video_presets.get(&id.into())
    }

    /// Returns a slice of the selected [FixtureId]s.
    pub fn selected_fixture_ids(&self) -> &[FixtureId] {
        &self.selected_fixture_ids
    }
}

/// Contains 'work in progress' values that can be stored into presets.
#[derive(Debug, Clone, Default)]
pub struct Programmer {
    values: HashMap<(FixtureId, Attribute), AttributeValue>,
}

impl Programmer {
    /// Sets an [AttributeValue] for a given (main) attribute [Attribute] on the
    /// fixture with the given [FixtureId]. The attribute has to be a main
    /// attribute to prevent double channel assignments.
    pub fn set_value(
        &mut self,
        fixture_id: FixtureId,
        main_attribute: Attribute,
        value: AttributeValue,
    ) {
        self.values.insert((fixture_id, main_attribute), value);
    }

    /// Gets an [AttributeValue] for the given (main) [Attribute] on the
    /// fixture with the given [FixtureId].
    pub fn value(
        &self,
        fixture_id: FixtureId,
        main_attribute: Attribute,
    ) -> Option<AttributeValue> {
        self.values.get(&(fixture_id, main_attribute)).copied()
    }

    /// Gets an iterator over all values.
    pub fn values(&self) -> impl IntoIterator<Item = (FixtureId, &Attribute, AttributeValue)> {
        self.values.iter().map(|((fid, attr), value)| (*fid, attr, *value))
    }

    /// Clears all values.
    pub fn clear(&mut self) {
        self.values.clear();
    }
}
