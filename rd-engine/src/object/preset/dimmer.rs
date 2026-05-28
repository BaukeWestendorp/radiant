use std::collections::HashMap;

use zeevonk::{AttributeName, FixtureTypeId, project::FixtureId, value::AttributeValue};

use crate::{Object, ObjectId, Preset, Slot};

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DimmerPreset {
    id: ObjectId,
    slot: Slot,
    name: String,
    universal: HashMap<AttributeName, AttributeValue>,
    global: HashMap<FixtureTypeId, HashMap<AttributeName, AttributeValue>>,
    selective: HashMap<FixtureId, HashMap<AttributeName, AttributeValue>>,
}

impl DimmerPreset {
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
}

impl Object for DimmerPreset {
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

impl Preset for DimmerPreset {
    fn universal(&self) -> &HashMap<AttributeName, AttributeValue> {
        &self.universal
    }

    fn global(&self) -> &HashMap<FixtureTypeId, HashMap<AttributeName, AttributeValue>> {
        &self.global
    }

    fn selective(&self) -> &HashMap<FixtureId, HashMap<AttributeName, AttributeValue>> {
        &self.selective
    }
}
