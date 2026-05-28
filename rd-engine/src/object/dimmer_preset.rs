use std::collections::HashMap;

use zeevonk::{AttributeName, value::AttributeValue};

use crate::{Object, ObjectId, Slot};

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DimmerPreset {
    id: ObjectId,
    slot: Slot,
    name: String,
    values: HashMap<AttributeName, AttributeValue>,
}

impl DimmerPreset {
    pub fn new(id: ObjectId, slot: Slot, name: String) -> Self {
        Self { id, slot, name, values: HashMap::new() }
    }

    pub fn values(&self) -> &HashMap<AttributeName, AttributeValue> {
        &self.values
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
