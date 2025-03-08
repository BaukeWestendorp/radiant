use crate::{Edge, NodeId, Socket};

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
