use zeevonk::project::FixtureId;

use crate::{Object, ObjectId, ObjectKind, SlotId};

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Group {
    id: ObjectId,
    slot_id: SlotId,
    name: String,

    fixture_ids: Vec<FixtureId>,
}

impl Group {
    pub fn fixture_ids(&self) -> &[FixtureId] {
        &self.fixture_ids
    }
}

impl Object for Group {
    fn kind() -> ObjectKind {
        ObjectKind::Group
    }

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
