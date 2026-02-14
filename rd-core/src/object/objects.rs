use std::collections::BTreeSet;

use zeevonk::project::FixtureId;

use crate::object::{Object, ObjectId, SlotId};

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Effect {
    id: ObjectId,
    slot_id: SlotId,
    name: String,

    file_name: String,
}

impl Effect {
    pub fn file_name(&self) -> &str {
        &self.file_name
    }
}

impl Object for Effect {
    fn id(&self) -> ObjectId {
        self.id
    }

    fn slot_id(&self) -> SlotId {
        self.slot_id
    }

    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Group {
    id: ObjectId,
    slot_id: SlotId,
    name: String,

    fixture_ids: BTreeSet<FixtureId>,
}

impl Group {
    pub fn fixture_ids(&self) -> &BTreeSet<FixtureId> {
        &self.fixture_ids
    }
}

impl Object for Group {
    fn id(&self) -> ObjectId {
        self.id
    }

    fn slot_id(&self) -> SlotId {
        self.slot_id
    }

    fn name(&self) -> &str {
        &self.name
    }
}
