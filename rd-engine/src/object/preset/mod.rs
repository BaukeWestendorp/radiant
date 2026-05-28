use std::{collections::HashMap, fmt};

use zeevonk::{AttributeName, FixtureTypeId, project::FixtureId, value::AttributeValue};

use crate::{Object, ObjectId, Slot};

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Preset {
    id: ObjectId,
    slot: Slot,
    name: String,
    universal: HashMap<AttributeName, AttributeValue>,
    global: HashMap<FixtureTypeId, HashMap<AttributeName, AttributeValue>>,
    selective: HashMap<FixtureId, HashMap<AttributeName, AttributeValue>>,
}

impl Preset {
    pub fn new(id: ObjectId, slot: Slot, name: String) -> Self {
        Self {
            id,
            slot,
            name,
            universal: HashMap::new(),
            global: HashMap::new(),
            selective: HashMap::new(),
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
