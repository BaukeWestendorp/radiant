use std::collections::HashMap;

use crate::show::object::{
    PresetBeam, PresetColor, PresetControl, PresetDimmer, PresetFocus, PresetGobo, PresetPosition,
    PresetShapers, PresetVideo,
};
use crate::show::patch::{Attribute, AttributeValue, FixtureId, FixtureTypeId};
use crate::show::{AnyObject, AnyObjectId, ObjectId};

macro_rules! preset_kind_and_content {
    ($preset:ident) => {
        impl $preset {
            pub fn content(&self) -> &PresetContent {
                &self.content
            }
        }
    };
}

preset_kind_and_content!(PresetDimmer);
preset_kind_and_content!(PresetPosition);
preset_kind_and_content!(PresetGobo);
preset_kind_and_content!(PresetColor);
preset_kind_and_content!(PresetBeam);
preset_kind_and_content!(PresetFocus);
preset_kind_and_content!(PresetControl);
preset_kind_and_content!(PresetShapers);
preset_kind_and_content!(PresetVideo);

pub trait PresetKind {}

#[derive(Debug, Clone, Copy)]
#[derive(serde::Deserialize)]
pub enum AnyPresetId {
    Dimmer(ObjectId<PresetDimmer>),
    Position(ObjectId<PresetPosition>),
    Gobo(ObjectId<PresetGobo>),
    Color(ObjectId<PresetColor>),
    Beam(ObjectId<PresetBeam>),
    Focus(ObjectId<PresetFocus>),
    Control(ObjectId<PresetControl>),
    Shapers(ObjectId<PresetShapers>),
    Video(ObjectId<PresetVideo>),
}

impl From<AnyPresetId> for AnyObjectId {
    fn from(any_preset_id: AnyPresetId) -> Self {
        match any_preset_id {
            AnyPresetId::Dimmer(id) => AnyObjectId::PresetDimmer(*id),
            AnyPresetId::Position(id) => AnyObjectId::PresetPosition(*id),
            AnyPresetId::Gobo(id) => AnyObjectId::PresetGobo(*id),
            AnyPresetId::Color(id) => AnyObjectId::PresetColor(*id),
            AnyPresetId::Beam(id) => AnyObjectId::PresetBeam(*id),
            AnyPresetId::Focus(id) => AnyObjectId::PresetFocus(*id),
            AnyPresetId::Control(id) => AnyObjectId::PresetControl(*id),
            AnyPresetId::Shapers(id) => AnyObjectId::PresetShapers(*id),
            AnyPresetId::Video(id) => AnyObjectId::PresetVideo(*id),
        }
    }
}

impl std::convert::TryFrom<AnyObjectId> for AnyPresetId {
    type Error = ();

    fn try_from(any_object_id: AnyObjectId) -> Result<Self, Self::Error> {
        match any_object_id {
            AnyObjectId::PresetDimmer(id) => Ok(AnyPresetId::Dimmer(id.into())),
            AnyObjectId::PresetPosition(id) => Ok(AnyPresetId::Position(id.into())),
            AnyObjectId::PresetGobo(id) => Ok(AnyPresetId::Gobo(id.into())),
            AnyObjectId::PresetColor(id) => Ok(AnyPresetId::Color(id.into())),
            AnyObjectId::PresetBeam(id) => Ok(AnyPresetId::Beam(id.into())),
            AnyObjectId::PresetFocus(id) => Ok(AnyPresetId::Focus(id.into())),
            AnyObjectId::PresetControl(id) => Ok(AnyPresetId::Control(id.into())),
            AnyObjectId::PresetShapers(id) => Ok(AnyPresetId::Shapers(id.into())),
            AnyObjectId::PresetVideo(id) => Ok(AnyPresetId::Video(id.into())),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
#[derive(serde::Deserialize)]
pub enum AnyPreset {
    Dimmer(PresetDimmer),
    Position(PresetPosition),
    Gobo(PresetGobo),
    Color(PresetColor),
    Beam(PresetBeam),
    Focus(PresetFocus),
    Control(PresetControl),
    Shapers(PresetShapers),
    Video(PresetVideo),
}

impl AnyPreset {
    pub fn content(&self) -> &PresetContent {
        match self {
            AnyPreset::Dimmer(preset) => preset.content(),
            AnyPreset::Position(preset) => preset.content(),
            AnyPreset::Gobo(preset) => preset.content(),
            AnyPreset::Color(preset) => preset.content(),
            AnyPreset::Beam(preset) => preset.content(),
            AnyPreset::Focus(preset) => preset.content(),
            AnyPreset::Control(preset) => preset.content(),
            AnyPreset::Shapers(preset) => preset.content(),
            AnyPreset::Video(preset) => preset.content(),
        }
    }
}

impl From<AnyPreset> for AnyObject {
    fn from(preset_id: AnyPreset) -> Self {
        match preset_id {
            AnyPreset::Dimmer(preset) => AnyObject::PresetDimmer(preset),
            AnyPreset::Position(preset) => AnyObject::PresetPosition(preset),
            AnyPreset::Gobo(preset) => AnyObject::PresetGobo(preset),
            AnyPreset::Color(preset) => AnyObject::PresetColor(preset),
            AnyPreset::Beam(preset) => AnyObject::PresetBeam(preset),
            AnyPreset::Focus(preset) => AnyObject::PresetFocus(preset),
            AnyPreset::Control(preset) => AnyObject::PresetControl(preset),
            AnyPreset::Shapers(preset) => AnyObject::PresetShapers(preset),
            AnyPreset::Video(preset) => AnyObject::PresetVideo(preset),
        }
    }
}

impl std::convert::TryFrom<AnyObject> for AnyPreset {
    type Error = ();

    fn try_from(any_object: AnyObject) -> Result<Self, Self::Error> {
        match any_object {
            AnyObject::PresetDimmer(preset) => Ok(AnyPreset::Dimmer(preset)),
            AnyObject::PresetPosition(preset) => Ok(AnyPreset::Position(preset)),
            AnyObject::PresetGobo(preset) => Ok(AnyPreset::Gobo(preset)),
            AnyObject::PresetColor(preset) => Ok(AnyPreset::Color(preset)),
            AnyObject::PresetBeam(preset) => Ok(AnyPreset::Beam(preset)),
            AnyObject::PresetFocus(preset) => Ok(AnyPreset::Focus(preset)),
            AnyObject::PresetControl(preset) => Ok(AnyPreset::Control(preset)),
            AnyObject::PresetShapers(preset) => Ok(AnyPreset::Shapers(preset)),
            AnyObject::PresetVideo(preset) => Ok(AnyPreset::Video(preset)),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
#[derive(serde::Deserialize)]
pub enum PresetContent {
    Universal(UniversalPreset),
    Global(GlobalPreset),
    Selective(SelectivePreset),
}

impl Default for PresetContent {
    fn default() -> Self {
        Self::Universal(UniversalPreset::default())
    }
}

#[derive(Debug, Clone)]
#[derive(serde::Deserialize)]
pub struct UniversalPreset {
    values: HashMap<Attribute, AttributeValue>,
}

impl UniversalPreset {
    pub fn values(&self) -> &HashMap<Attribute, AttributeValue> {
        &self.values
    }
}

impl Default for UniversalPreset {
    fn default() -> Self {
        Self { values: HashMap::default() }
    }
}

#[derive(Debug, Clone)]
#[derive(serde::Deserialize)]
pub struct GlobalPreset {
    values: HashMap<(FixtureTypeId, Attribute), AttributeValue>,
}

impl GlobalPreset {
    pub fn values(&self) -> &HashMap<(FixtureTypeId, Attribute), AttributeValue> {
        &self.values
    }
}

impl Default for GlobalPreset {
    fn default() -> Self {
        Self { values: HashMap::default() }
    }
}

#[derive(Debug, Clone)]
#[derive(serde::Deserialize)]
pub struct SelectivePreset {
    values: HashMap<(FixtureId, Attribute), AttributeValue>,
}

impl SelectivePreset {
    pub fn values(&self) -> &HashMap<(FixtureId, Attribute), AttributeValue> {
        &self.values
    }
}

impl Default for SelectivePreset {
    fn default() -> Self {
        Self { values: HashMap::default() }
    }
}
