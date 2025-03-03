use gpui::*;

use crate::Graph;

use super::{graph::GraphView, graph_event::GraphEvent};

pub struct GraphEditorView<State: Default, Value: Clone> {
    pub graph_view: Entity<GraphView<State, Value>>,
}

impl<State: Default + 'static, Value: Clone + 'static> GraphEditorView<State, Value> {
    pub fn build(graph: Entity<Graph<State, Value>>, cx: &mut App) -> Entity<Self> {
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

    pub fn graph<'a>(&'a self, cx: &'a App) -> &'a Entity<Graph<State, Value>> {
        self.graph_view.read(cx).graph()
    }
}

impl<State: Default + 'static, Value: Clone + 'static> Render for GraphEditorView<State, Value> {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<'_, Self>) -> impl IntoElement {
        div().child("Graph")
    }
}
