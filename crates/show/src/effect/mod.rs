pub mod graph;

pub use graph::*;

use crate::GroupId;

pub type EffectId = u32;

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Effect {
    id: EffectId,
    pub group: GroupId,
    pub graph: EffectGraph,
}

impl Effect {
    pub(crate) fn new(id: EffectId, group: GroupId, graph: EffectGraph) -> Self {
        Self { id, group, graph }
    }
}
