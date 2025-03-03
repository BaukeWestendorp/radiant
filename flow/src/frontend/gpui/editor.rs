use super::graph::GraphView;
use crate::{Graph, GraphDef, frontend::GraphEvent};
use gpui::*;

pub struct GraphEditorView<D: GraphDef> {
    pub graph_view: Entity<GraphView<D>>,
}

impl<D: GraphDef + 'static> GraphEditorView<D> {
    pub fn build(graph: Entity<Graph<D>>, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            let graph_view = GraphView::build(graph, cx);

            let graph = graph_view.read(cx).graph().clone();
            cx.subscribe(&graph, |_editor, _graph, event: &GraphEvent, _cx| {
                dbg!(&event);
            })
            .detach();

            Self { graph_view }
        })
    }

    pub fn graph<'a>(&'a self, cx: &'a App) -> &'a Entity<Graph<D>> {
        self.graph_view.read(cx).graph()
    }
}

impl<D: GraphDef + 'static> Render for GraphEditorView<D> {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<'_, Self>) -> impl IntoElement {
        div().child(self.graph_view.clone())
    }
}
