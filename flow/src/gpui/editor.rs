use super::{GraphEvent, graph::GraphView, node::NodeMeasurements};
use crate::GraphDef;
use gpui::*;
use prelude::FluentBuilder;
use ui::{Pannable, PannableEvent, theme::ActiveTheme, z_stack};

const KEY_CONTEXT: &str = "GraphEditor";

actions!(graph_editor, [OpenNewNodeMenu]);

pub fn init(app: &mut App) {
    app.bind_keys([KeyBinding::new("space", OpenNewNodeMenu, Some(KEY_CONTEXT))]);
}

pub struct GraphEditorView<D: GraphDef> {
    graph_view: Entity<Pannable>,
    graph: Entity<crate::Graph<D>>,
    visual_graph_offset: Point<Pixels>,

    focus_handle: FocusHandle,
}

impl<D: GraphDef + 'static> GraphEditorView<D> {
    pub fn build(
        graph: Entity<crate::Graph<D>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<Self> {
        cx.new(|cx| {
            let graph_view = GraphView::build(graph.clone(), window, cx);

            window
                .subscribe(&graph, cx, {
                    let graph_view = graph_view.clone();
                    move |graph, event: &GraphEvent, window, cx| {
                        graph_view.update(cx, |graph_view, cx| match event {
                            GraphEvent::NodeAdded(node_id) => {
                                graph_view.add_node(*node_id, window, cx)
                            }
                            GraphEvent::NodeRemoved(node_id) => graph_view.remove_node(node_id, cx),
                            GraphEvent::EdgeAdded { .. } => {}
                            GraphEvent::EdgeRemoved { .. } => {}
                        });
                        cx.notify(graph.entity_id());
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

            Self {
                graph_view: pannable,
                graph,
                visual_graph_offset: graph_offset,
                focus_handle: cx.focus_handle(),
            }
        })
    }

    pub fn graph(&self) -> &Entity<crate::Graph<D>> {
        &self.graph
    }
}

impl<D: GraphDef + 'static> GraphEditorView<D> {
    fn handle_open_new_node_menu(
        &mut self,
        _: &OpenNewNodeMenu,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        dbg!("open new node menu");
    }
}

impl<D: GraphDef + 'static> Render for GraphEditorView<D> {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let NodeMeasurements { snap_size, .. } = NodeMeasurements::new(window);

        let grid = ui::scrollable_line_grid(
            &self.visual_graph_offset,
            snap_size,
            cx.theme().line_grid_color,
        )
        .size_full();

        let focused = self.focus_handle.is_focused(window);

        z_stack([grid.into_any_element(), self.graph_view.clone().into_any_element()])
            .track_focus(&self.focus_handle)
            .key_context(KEY_CONTEXT)
            .relative()
            .size_full()
            .overflow_hidden()
            .when(focused, |e| e.on_action(cx.listener(Self::handle_open_new_node_menu)))
    }
}

impl<D: GraphDef + 'static> Focusable for GraphEditorView<D> {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
