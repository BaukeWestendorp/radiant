use crate::showfile;

mod flow_graph;

use flow::Point;
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
    pub offset: Point,
}

impl super::Asset for EffectGraph {
    type Id = EffectGraphId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl EffectGraph {
    pub(crate) fn from_showfile(graph: showfile::EffectGraph) -> Self {
        Self {
            id: EffectGraphId(graph.id),
            label: graph.label,
            graph: graph.graph,
            offset: graph.offset,
        }
    }
}
