use flow_gpui::{Graph, editor::GraphEditorView, flow::GraphDef};
use gpui::*;

pub struct GraphEditor<D: GraphDef> {
    graph_editor_view: Entity<GraphEditorView<D>>,
}

impl<D: GraphDef + 'static> GraphEditor<D> {
    pub fn build(effect_graph: Entity<Graph<D>>, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            let graph_editor_view = GraphEditorView::build(effect_graph, cx);
            Self { graph_editor_view }
        })
    }
}

impl<D: GraphDef + 'static> Render for GraphEditor<D> {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .child(self.graph_editor_view.clone())
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|editor, _, _, cx| {
                    let graph = editor.graph_editor_view.read(cx).graph(cx).clone();
                    graph.update(cx, |graph, cx| {
                        let position = (50.0, 75.0);
                        graph.add_node(flow_gpui::flow::Node::new("new_node"), position, cx);
                    });
                }),
            )
            .on_mouse_down(
                MouseButton::Right,
                cx.listener(|editor, _, _, cx| {
                    let graph = editor.graph_editor_view.read(cx).graph(cx).clone();
                    graph.update(cx, |graph, cx| {
                        let Some(node_id) = graph.node_ids().last().copied() else { return };
                        graph.remove_node(&node_id, cx);
                    });
                }),
            )
    }
}
