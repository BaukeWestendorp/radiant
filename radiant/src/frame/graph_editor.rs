use flow::GraphDef;
use flow_gpui::{GpuiFrontend, GpuiGraph, GpuiGraphState, GraphEditorView, NodePosition};

use gpui::*;

pub struct GraphEditor<D: GraphDef<State = GpuiGraphState>> {
    graph_editor_view: Entity<GraphEditorView<D>>,
}

impl<D: GraphDef<State = GpuiGraphState> + 'static> GraphEditor<D> {
    pub fn build(effect_graph: Entity<GpuiGraph<D>>, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            let graph_editor_view = GraphEditorView::build(effect_graph, cx);
            Self { graph_editor_view }
        })
    }
}

impl<D: GraphDef<State = GpuiGraphState> + 'static> Render for GraphEditor<D> {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .child(self.graph_editor_view.clone())
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|editor, _, _, cx| {
                    editor.graph_editor_view.read(cx).graph(cx).clone().update(cx, |graph, cx| {
                        let position = NodePosition { x: 50.0, y: 75.0 };
                        graph.add_node(
                            flow::Node::new("new_node"),
                            position,
                            &mut GpuiFrontend::from(cx),
                        );
                    });
                }),
            )
            .on_mouse_down(
                MouseButton::Right,
                cx.listener(|editor, _, _, cx| {
                    editor.graph_editor_view.read(cx).graph(cx).clone().update(cx, |graph, cx| {
                        graph.remove_node(graph.node_ids().last().unwrap(), cx.into());
                    });
                }),
            )
    }
}
