pub mod graph;

pub use graph::{
    Control as EffectGraphControl, DataType as EffectGraphDataType,
    GraphDefinition as EffectGraphDefinition, NodeData as EffectGraphNodeData,
    NodeKind as EffectGraphNodeKind, ProcessingContext as EffectGraphProcessingContext,
    Value as EffectGraphValue,
};

use graph::EffectGraphId;

use super::GroupId;

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
