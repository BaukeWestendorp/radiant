use std::collections::HashMap;
use std::marker::PhantomData;

use crate::show::object::{
    PresetBeam, PresetColor, PresetControl, PresetDimmer, PresetFocus, PresetGobo, PresetPosition,
    PresetShapers, PresetVideo,
};
use crate::show::patch::{Attribute, AttributeValue, FixtureId, FixtureTypeId};

macro_rules! preset_kind_and_content {
    ($kind:ident, $preset:ident) => {
        #[derive(Debug, Clone, Default)]
        #[derive(serde::Deserialize)]
        pub struct $kind;

        impl PresetKind for $kind {}

        impl $preset {
            pub fn content(&self) -> &PresetContent<$kind> {
                &self.content
            }
        }
    };
}

preset_kind_and_content!(Dimmer, PresetDimmer);
preset_kind_and_content!(Position, PresetPosition);
preset_kind_and_content!(Gobo, PresetGobo);
preset_kind_and_content!(Color, PresetColor);
preset_kind_and_content!(Beam, PresetBeam);
preset_kind_and_content!(Focus, PresetFocus);
preset_kind_and_content!(Control, PresetControl);
preset_kind_and_content!(Shapers, PresetShapers);
preset_kind_and_content!(Video, PresetVideo);

pub trait PresetKind {}

#[derive(Debug, Clone)]
#[derive(serde::Deserialize)]
pub enum PresetContent<PresetKind> {
    Universal(UniversalPreset<PresetKind>),
    Global(GlobalPreset<PresetKind>),
    Selective(SelectivePreset<PresetKind>),
}

impl<K: Default> Default for PresetContent<K> {
    fn default() -> Self {
        Self::Universal(UniversalPreset::default())
    }
}

#[derive(Debug, Clone)]
#[derive(serde::Deserialize)]
pub struct UniversalPreset<K> {
    #[serde(skip)]
    _kind: PhantomData<K>,
    values: HashMap<Attribute, AttributeValue>,
}

impl<K: Default> Default for UniversalPreset<K> {
    fn default() -> Self {
        Self { _kind: PhantomData::default(), values: HashMap::default() }
    }
}

#[derive(Debug, Clone)]
#[derive(serde::Deserialize)]
pub struct GlobalPreset<K> {
    _kind: K,
    values: HashMap<(FixtureTypeId, Attribute), AttributeValue>,
}

impl<K: Default> Default for GlobalPreset<K> {
    fn default() -> Self {
        Self { _kind: K::default(), values: HashMap::default() }
    }
}

#[derive(Debug, Clone)]
#[derive(serde::Deserialize)]
pub struct SelectivePreset<K> {
    _kind: K,
    values: HashMap<(FixtureId, Attribute), AttributeValue>,
}

impl<K: Default> Default for SelectivePreset<K> {
    fn default() -> Self {
        Self { _kind: K::default(), values: HashMap::default() }
    }
}
