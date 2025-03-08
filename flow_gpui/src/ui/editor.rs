use super::{GraphEvent, graph::GraphView};
use flow::GraphDef;
use gpui::*;

pub struct GraphEditorView<D: GraphDef> {
    pub graph_view: Entity<GraphView<D>>,
}

impl<D: GraphDef + 'static> GraphEditorView<D> {
    pub fn build(graph: Entity<crate::Graph<D>>, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            let graph_view = GraphView::build(graph.clone(), cx);

            cx.subscribe(&graph, |editor: &mut Self, _graph, event: &GraphEvent, cx| {
                editor.graph_view.update(cx, |graph, cx| match event {
                    GraphEvent::NodeAdded(node_id) => graph.add_node(*node_id, cx),
                    GraphEvent::NodeRemoved(node_id) => graph.remove_node(*node_id, cx),
                    GraphEvent::EdgeAdded { edge } => graph.add_edge(edge.clone(), cx),
                    GraphEvent::EdgeRemoved { source } => graph.remove_edge(source, cx),
                });
            })
            .detach();

            Self { graph_view }
        })
    }

    pub fn graph(&self, cx: &App) -> Entity<crate::Graph<D>> {
        self.graph_view.read(cx).graph().clone()
    }
}

impl<D: GraphDef + 'static> Render for GraphEditorView<D> {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().child(self.graph_view.clone())
    }
}
