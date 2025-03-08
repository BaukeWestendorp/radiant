use flow::GraphDef;
use flow_gpui::{EventEmittingGraph, GpuiFrontend, GpuiGraphState, GraphEditorView};

use gpui::*;

pub struct GraphEditor<D>
where
    D: GraphDef<State = GpuiGraphState>,
{
    graph_editor_view: Entity<GraphEditorView<D>>,
}

impl<D> GraphEditor<D>
where
    D: GraphDef<State = GpuiGraphState> + 'static,
{
    pub fn build(effect_graph: Entity<EventEmittingGraph<D>>, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            let graph_editor_view = GraphEditorView::build(effect_graph, cx);
            Self { graph_editor_view }
        })
    }
}

impl<D> Render for GraphEditor<D>
where
    D: GraphDef<State = GpuiGraphState> + 'static,
{
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(self.graph_editor_view.clone()).on_mouse_down(
            MouseButton::Left,
            cx.listener(|editor, _, _, cx| {
                editor.graph_editor_view.read(cx).graph(cx).clone().update(cx, |graph, cx| {
                    graph.add_node(flow::Node::new("new_node"), &mut GpuiFrontend::from(cx));
                });
            }),
        )
    }
}
