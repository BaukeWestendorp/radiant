pub mod graph;

pub use graph::*;

use crate::GroupId;

pub type EffectId = u32;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Effect {
    id: EffectId,
    pub group: GroupId,
    pub kind: EffectKind,
}

impl Effect {
    pub(crate) fn new(id: EffectId, group: GroupId, kind: EffectKind) -> Self {
        Self { id, group, kind }
    }

    pub fn id(&self) -> EffectId {
        self.id
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub enum EffectKind {
    Graph(EffectGraphId),
}
