use std::collections::HashMap;

use crate::attr::{Attribute, AttributeValue};
use crate::builtin::{FixtureId, GdtfFixtureTypeId};

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Preset<F: FeatureGroup> {
    pub content: PresetContent,

    marker: std::marker::PhantomData<F>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum PresetContent {
    Universal(HashMap<Attribute, AttributeValue>),
    Global(HashMap<(GdtfFixtureTypeId, Attribute), AttributeValue>),
    Selective(HashMap<(FixtureId, Attribute), AttributeValue>),
}

impl Default for PresetContent {
    fn default() -> Self {
        Self::Universal(Default::default())
    }
}

pub trait FeatureGroup {}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Dimmer;
impl FeatureGroup for Dimmer {}
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Position;
impl FeatureGroup for Position {}
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Gobo;
impl FeatureGroup for Gobo {}
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Color;
impl FeatureGroup for Color {}
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Beam;
impl FeatureGroup for Beam {}
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Focus;
impl FeatureGroup for Focus {}
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Control;
impl FeatureGroup for Control {}
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Shapers;
impl FeatureGroup for Shapers {}
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Video;
impl FeatureGroup for Video {}
