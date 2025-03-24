use crate::GraphDef;

use super::{GraphEvent, graph::GraphView, node::SNAP_GRID_SIZE};
use gpui::*;
use ui::{Pannable, PannableEvent, theme::ActiveTheme, z_stack};

pub struct GraphEditorView<D: GraphDef> {
    graph_view: Entity<Pannable>,
    graph: Entity<crate::Graph<D>>,
    visual_graph_offset: Point<Pixels>,
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
                        GraphEvent::NodeRemoved(node_id) => graph.remove_node(node_id, cx),
                        GraphEvent::EdgeAdded { .. } => {}
                        GraphEvent::EdgeRemoved { .. } => {}
                    });
                }
            })
            .detach();

            let graph_offset = *graph.read(cx).offset();
            let pannable = cx.new(|_cx| Pannable::new("graph", graph_offset, graph_view.clone()));

            cx.subscribe(&pannable, |editor: &mut Self, _pannable, event: &PannableEvent, cx| {
                match event {
                    PannableEvent::OffsetChanged(position) => {
                        editor.visual_graph_offset = *position;
                    }
                    PannableEvent::OffsetCommitted(position) => {
                        editor.graph().update(cx, |graph, cx| {
                            graph.set_offset(*position);
                            cx.notify();
                        });
                    }
                }
            })
            .detach();

            Self { graph_view: pannable, graph, visual_graph_offset: graph_offset }
        })
    }

    pub fn graph(&self) -> &Entity<crate::Graph<D>> {
        &self.graph
    }
}

impl<D: GraphDef + 'static> Render for GraphEditorView<D> {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let grid = ui::scrollable_line_grid(
            &self.visual_graph_offset,
            SNAP_GRID_SIZE,
            cx.theme().line_grid_color,
        )
        .size_full();

        z_stack([grid.into_any_element(), self.graph_view.clone().into_any_element()])
            .relative()
            .size_full()
            .overflow_hidden()
    }
}
