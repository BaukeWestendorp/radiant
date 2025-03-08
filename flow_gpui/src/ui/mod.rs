use flow::{Edge, GraphDef, NodeId, Socket};

pub mod editor;
mod graph;
mod node;

#[derive(Debug, Clone)]
pub enum GraphEvent {
    NodeAdded(NodeId),
    NodeRemoved(NodeId),
    EdgeAdded { edge: Edge },
    EdgeRemoved { source: Socket },
}

impl<D: GraphDef + 'static> gpui::EventEmitter<GraphEvent> for crate::Graph<D> {}
