use zeevonk::project::FixtureId;

use crate::{Object, ObjectId, Slot};

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Group {
    id: ObjectId,
    slot: Slot,
    name: String,
    fixture_ids: Vec<FixtureId>,
}

impl Group {
    pub fn new(id: ObjectId, slot: Slot, name: String) -> Self {
        Self { id, slot, name, fixture_ids: Vec::new() }
    }

    pub fn fixture_ids(&self) -> &[FixtureId] {
        &self.fixture_ids
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
