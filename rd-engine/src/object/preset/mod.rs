use std::{collections::HashMap, fmt, ops};

use crate::{
    mvr_gdtf::gdtf::{FixtureTypeId, attr::AttributeName},
    object::{Object, ObjectId, Slot},
    patch::FixtureId,
    value::AttributeValue,
};

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Preset {
    id: ObjectId,
    slot: Slot,
    name: String,
    pub(crate) universal: UniversalPresetContent,
    pub(crate) global: GlobalPresetContent,
    pub(crate) selective: SelectivePresetContent,
}

impl Preset {
    pub fn new(id: ObjectId, slot: Slot, name: String) -> Self {
        Self {
            id,
            slot,
            name,
            universal: Default::default(),
            global: Default::default(),
            selective: Default::default(),
        }
    }

    pub fn universal(&self) -> &HashMap<AttributeName, AttributeValue> {
        &self.universal
    }

    pub fn global(&self) -> &HashMap<FixtureTypeId, HashMap<AttributeName, AttributeValue>> {
        &self.global
    }

    pub fn selective(&self) -> &HashMap<FixtureId, HashMap<AttributeName, AttributeValue>> {
        &self.selective
    }
}

impl Object for Preset {
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

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct UniversalPresetContent {
    values: HashMap<AttributeName, AttributeValue>,
}

impl UniversalPresetContent {
    pub fn new(values: HashMap<AttributeName, AttributeValue>) -> Self {
        Self { values }
    }

    pub fn merge(&mut self, other: UniversalPresetContent) {
        for (attr_name, attr_value) in other.values {
            // FIXME: Implement multiple merge modes.
            self.values.insert(attr_name, attr_value);
        }
    }
}

impl ops::Deref for UniversalPresetContent {
    type Target = HashMap<AttributeName, AttributeValue>;

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl ops::DerefMut for UniversalPresetContent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.values
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct GlobalPresetContent {
    values: HashMap<FixtureTypeId, HashMap<AttributeName, AttributeValue>>,
}

impl GlobalPresetContent {
    pub fn new(values: HashMap<FixtureTypeId, HashMap<AttributeName, AttributeValue>>) -> Self {
        Self { values }
    }

    pub fn merge(&mut self, other: GlobalPresetContent) {
        for (fixture_type_id, attr_map) in other.values {
            self.values.entry(fixture_type_id).or_insert_with(HashMap::new).extend(attr_map);
        }
    }
}

impl ops::Deref for GlobalPresetContent {
    type Target = HashMap<FixtureTypeId, HashMap<AttributeName, AttributeValue>>;

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl ops::DerefMut for GlobalPresetContent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.values
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct SelectivePresetContent {
    values: HashMap<FixtureId, HashMap<AttributeName, AttributeValue>>,
}

impl SelectivePresetContent {
    pub fn new(values: HashMap<FixtureId, HashMap<AttributeName, AttributeValue>>) -> Self {
        Self { values }
    }

    pub fn merge(&mut self, other: SelectivePresetContent) {
        for (fixture_id, attr_map) in other.values {
            // FIXME: Implement multiple merge modes.
            self.values.entry(fixture_id).or_insert_with(HashMap::new).extend(attr_map);
        }
    }
}

impl ops::Deref for SelectivePresetContent {
    type Target = HashMap<FixtureId, HashMap<AttributeName, AttributeValue>>;

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl ops::DerefMut for SelectivePresetContent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.values
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PresetId {
    kind: PresetKind,
    object_id: ObjectId,
}

impl PresetId {
    pub fn new(kind: PresetKind, object_id: ObjectId) -> Self {
        Self { kind, object_id }
    }

    pub fn kind(&self) -> PresetKind {
        self.kind
    }

    pub fn object_id(&self) -> ObjectId {
        self.object_id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum PresetKind {
    Dimmer,
    Position,
    Gobo,
    Color,
    Beam,
    Focus,
    Control,
    Shapers,
    Video,
}

impl fmt::Display for PresetKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PresetKind::Dimmer => write!(f, "Dimmer"),
            PresetKind::Position => write!(f, "Position"),
            PresetKind::Gobo => write!(f, "Gobo"),
            PresetKind::Color => write!(f, "Color"),
            PresetKind::Beam => write!(f, "Beam"),
            PresetKind::Focus => write!(f, "Focus"),
            PresetKind::Control => write!(f, "Control"),
            PresetKind::Shapers => write!(f, "Shapers"),
            PresetKind::Video => write!(f, "Video"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum PresetContentKind {
    Universal,
    Global,
    Selective,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum PresetContent {
    Universal(UniversalPresetContent),
    Global(GlobalPresetContent),
    Selective(SelectivePresetContent),
}
