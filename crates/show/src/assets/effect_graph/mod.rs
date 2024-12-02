use crate::showfile;

mod flow_graph;

pub use flow_graph::{
    FlowEffectGraph, GraphDefinition as EffectGraphDefinition,
    ProcessingContext as EffectGraphProcessingContext,
};

super::asset_id!(pub EffectGraphId);

#[derive(Clone)]
pub struct EffectGraph {
    pub id: EffectGraphId,
    pub label: String,
    pub graph: FlowEffectGraph,
}

impl super::Asset for EffectGraph {
    type Id = EffectGraphId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl From<showfile::EffectGraph> for EffectGraph {
    fn from(graph: showfile::EffectGraph) -> Self {
        Self {
            id: EffectGraphId(graph.id),
            label: graph.label,
            graph: graph.graph,
        }
    }
}
