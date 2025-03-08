use crate::{EventEmittingGraph, GpuiGraphState};

use super::graph::GraphView;
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
    pub fn build(graph: Entity<EventEmittingGraph<D>>, cx: &mut App) -> Entity<Self> {
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

    pub fn graph<'a>(&'a self, cx: &'a App) -> &'a Entity<EventEmittingGraph<D>> {
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
