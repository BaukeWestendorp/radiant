use gpui::SharedString;

use super::flow_graph::FlowEffectGraph;

crate::showfile::asset_id!(pub EffectGraphId);

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct EffectGraph {
    pub label: SharedString,
    pub graph: FlowEffectGraph,
}

impl EffectGraph {
    pub(crate) fn new(label: SharedString, graph: FlowEffectGraph) -> Self {
        Self { label, graph }
    }
}
