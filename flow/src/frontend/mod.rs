use crate::{Edge, NodeId, Socket};

#[cfg(feature = "frontend_gpui")]
pub mod gpui;

pub trait Frontend {
    fn emit_event(&mut self, event: GraphEvent);
}

impl Frontend for () {
    fn emit_event(&mut self, _event: GraphEvent) {}
}

#[derive(Debug)]
pub enum GraphEvent {
    NodeAdded(NodeId),
    NodeRemoved(NodeId),
    EdgeAdded { edge: Edge },
    EdgeRemoved { source: Socket },
}
