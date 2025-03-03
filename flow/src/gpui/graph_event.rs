use gpui::*;

use crate::{Edge, Graph, NodeId, Socket};

#[derive(Debug)]
pub enum GraphEvent {
    NodeAdded(NodeId),
    NodeRemoved(NodeId),
    EdgeAdded { edge: Edge },
    EdgeRemoved { source: Socket },
}

impl<State: Default + 'static, Value: Clone + 'static> EventEmitter<GraphEvent>
    for Graph<State, Value>
{
}
