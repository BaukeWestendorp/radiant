use crate::{Object, ObjectId, Slot};

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Group {
    id: ObjectId,
    slot: Slot,
    name: String,
}

impl Group {
    pub fn new(id: ObjectId, slot: Slot, name: String) -> Self {
        Self { id, slot, name }
    }
}

impl Object for Group {
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
