pub mod graph;

pub use graph::*;

pub type EffectId = u32;

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Effect {
    id: EffectId,
    pub graph: EffectGraph,
}

impl Effect {
    pub(crate) fn new(id: EffectId, graph: EffectGraph) -> Self {
        Self { id, graph }
    }
}
