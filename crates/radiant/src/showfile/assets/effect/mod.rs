pub mod flow_graph;
pub mod graph;

#[allow(unused_imports)]
pub use flow_graph::{
    Control as EffectGraphControl, DataType as EffectGraphDataType,
    GraphDefinition as EffectGraphDefinition, NodeData as EffectGraphNodeData,
    NodeKind as EffectGraphNodeKind, ProcessingContext as EffectGraphProcessingContext,
    Value as EffectGraphValue,
};
pub use graph::*;

use gpui::SharedString;

use super::GroupId;

super::asset_id!(pub EffectId);

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Effect {
    id: EffectId,
    pub label: SharedString,
    pub group: GroupId,
    pub kind: EffectKind,
}

impl Effect {
    pub(crate) fn new(id: EffectId, label: SharedString, group: GroupId, kind: EffectKind) -> Self {
        Self {
            id,
            label,
            group,
            kind,
        }
    }

    pub fn id(&self) -> EffectId {
        self.id
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub enum EffectKind {
    Graph(EffectGraphId),
}
