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

impl EffectGraph {
    pub fn new(id: EffectGraphId) -> Self {
        Self {
            id,
            label: "New Effect Graph".to_string(),
            graph: FlowEffectGraph::default(),
            offset: Point::default(),
        }
    }
}

impl super::Asset for EffectGraph {
    type Id = EffectGraphId;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn label(&self) -> &str {
        &self.label
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

    pub(crate) fn to_showfile(&self) -> showfile::EffectGraph {
        showfile::EffectGraph {
            id: self.id.0,
            label: self.label.clone(),
            graph: self.graph.clone(),
            offset: self.offset,
        }
    }
}
