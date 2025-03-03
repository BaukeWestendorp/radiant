use gpui::*;

use crate::{Edge, Graph, GraphDef, NodeId, Socket};

pub struct GraphView<D: GraphDef> {
    pub graph: Entity<Graph<D>>,
}

impl<D: GraphDef + 'static> GraphView<D> {
    pub fn build(graph: Entity<Graph<D>>, cx: &mut App) -> Entity<Self> {
        cx.new(|_cx| Self { graph })
    }

    pub fn graph(&self) -> &Entity<Graph<D>> {
        &self.graph
    }
}

impl<D: GraphDef + 'static> Render for GraphView<D> {
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

impl<D: GraphDef + 'static> EventEmitter<GraphEvent> for Graph<D> {}
