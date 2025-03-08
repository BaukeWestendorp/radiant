use super::graph::GraphView;
use crate::{GpuiGraph, GpuiGraphState};
use flow::{GraphDef, frontend::GraphEvent};
use gpui::*;

pub struct GraphEditorView<D>
where
    D: GraphDef<State = GpuiGraphState>,
{
    pub graph_view: Entity<GraphView<D>>,
}

impl<D> GraphEditorView<D>
where
    D: GraphDef<State = GpuiGraphState> + 'static,
{
    pub fn build(graph: Entity<GpuiGraph<D>>, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            let graph_view = GraphView::build(graph, cx);

            let graph = graph_view.read(cx).graph().clone();
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

    pub fn graph<'a>(&'a self, cx: &'a App) -> &'a Entity<GpuiGraph<D>> {
        self.graph_view.read(cx).graph()
    }
}

impl<D> Render for GraphEditorView<D>
where
    D: GraphDef<State = GpuiGraphState> + 'static,
{
    fn render(&mut self, _window: &mut Window, _x: &mut Context<Self>) -> impl IntoElement {
        div().child(self.graph_view.clone())
    }
}
