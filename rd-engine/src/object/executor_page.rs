use crate::{Object, ObjectReference, SlotId};

use super::{ObjectId, ObjectKind};

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ExecutorPage {
    id: ObjectId,
    slot_id: SlotId,
    name: String,

    executors: [Executor; 18],
}

impl ExecutorPage {
    pub fn executors(&self) -> &[Executor; 18] {
        &self.executors
    }
}

impl Object for ExecutorPage {
    fn kind() -> ObjectKind {
        ObjectKind::ExecutorPage
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
pub struct Executor {
    content: Option<ExecutorContent>,
    enabled: bool,
    master: f32,
}

impl Executor {
    pub fn content(&self) -> Option<&ExecutorContent> {
        self.content.as_ref()
    }

    pub fn enabled(&self) -> bool {
        self.content.is_some() && self.enabled
    }

    pub fn master(&self) -> f32 {
        self.master.clamp(0.0, 1.0)
    }
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum ExecutorContent {
    CueList { cue_list: ObjectReference, cue_index: usize },
}
