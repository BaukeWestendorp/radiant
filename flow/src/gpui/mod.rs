use crate::{GraphDef, InputSocket, NodeId, OutputSocket};

pub mod editor;
mod graph;
mod node;

#[derive(Debug, Clone)]
pub enum GraphEvent {
    NodeAdded(NodeId),
    NodeRemoved(NodeId),
    EdgeAdded { target: InputSocket, source: OutputSocket },
    EdgeRemoved { target: InputSocket },
}

impl<D: GraphDef + 'static> gpui::EventEmitter<GraphEvent> for crate::Graph<D> {}
