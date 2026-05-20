use crate::{Object, ObjectId, ObjectKind, ObjectReference, SlotId};

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Executor {
    id: ObjectId,
    slot_id: SlotId,
    name: String,

    content: Option<ExecutorContent>,
    enabled: bool,
    master: f32,
}

impl Executor {
    pub fn content(&self) -> Option<&ExecutorContent> {
        self.content.as_ref()
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn master(&self) -> f32 {
        self.master
    }
}

impl Object for Executor {
    fn kind() -> ObjectKind {
        ObjectKind::Executor
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

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum ExecutorContent {
    CueList { cue_list: ObjectReference, cue_index: usize },
}
