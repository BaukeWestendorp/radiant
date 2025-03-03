use gpui::*;

use crate::{Edge, Graph, NodeId, Socket, ValueImpl};

pub struct GraphView<State: Default, Value: ValueImpl> {
    pub graph: Entity<Graph<State, Value>>,
}

impl<State: Default + 'static, Value: ValueImpl + 'static> GraphView<State, Value> {
    pub fn build(graph: Entity<Graph<State, Value>>, cx: &mut App) -> Entity<Self> {
        cx.new(|_cx| Self { graph })
    }

    pub fn graph(&self) -> &Entity<Graph<State, Value>> {
        &self.graph
    }
}

impl<State: Default + 'static, Value: ValueImpl + 'static> Render for GraphView<State, Value> {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<'_, Self>) -> impl IntoElement {
        div().child("Graph")
    }
}

#[derive(Debug)]
pub enum GraphEvent {
    NodeAdded(NodeId),
    NodeRemoved(NodeId),
    EdgeAdded { edge: Edge },
    EdgeRemoved { source: Socket },
}

impl<State: Default + 'static, Value: ValueImpl + 'static> EventEmitter<GraphEvent>
    for Graph<State, Value>
{
}
