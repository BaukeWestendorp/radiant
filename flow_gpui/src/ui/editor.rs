use super::{GraphEvent, graph::GraphView};
use flow::GraphDef;
use gpui::*;
use ui::elements::{Pannable, PannableEvent};

pub struct GraphEditorView<D: GraphDef> {
    graph_view: Entity<Pannable>,
    graph: Entity<crate::Graph<D>>,
}

impl<D: GraphDef + 'static> GraphEditorView<D> {
    pub fn build(graph: Entity<crate::Graph<D>>, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            let graph_view = GraphView::build(graph.clone(), cx);

            cx.subscribe(&graph, {
                let graph_view = graph_view.clone();
                move |_editor, _graph, event: &GraphEvent, cx| {
                    graph_view.update(cx, |graph, cx| match event {
                        GraphEvent::NodeAdded(node_id) => graph.add_node(*node_id, cx),
                        GraphEvent::NodeRemoved(node_id) => graph.remove_node(*node_id, cx),
                        GraphEvent::EdgeAdded { edge } => graph.add_edge(edge.clone(), cx),
                        GraphEvent::EdgeRemoved { source } => graph.remove_edge(source, cx),
                    });
                }
            })
            .detach();

            let pannable =
                cx.new(|_cx| Pannable::new("graph", Point::default(), graph_view.clone()));

            cx.subscribe(&pannable, |editor: &mut Self, _pannable, event: &PannableEvent, cx| {
                editor.graph().update(cx, |graph, cx| {
                    match event {
                        PannableEvent::OffsetCommitted(position) => graph.set_offset(*position),
                    }
                    cx.notify();
                });
            })
            .detach();

            Self { graph_view: pannable, graph }
        })
    }

    pub fn graph(&self) -> &Entity<crate::Graph<D>> {
        &self.graph
    }
}

impl<D: GraphDef + 'static> Render for GraphEditorView<D> {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().child(self.graph_view.clone()).size_full().overflow_hidden()
    }
}
