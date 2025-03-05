use crate::{Graph, GraphDef, NodeId};
use gpui::*;
use ui::theme::ActiveTheme;

pub struct NodeView<D: GraphDef> {
    node_id: NodeId,

    graph: Entity<Graph<D>>,
}

impl<D: GraphDef + 'static> NodeView<D> {
    pub fn build(node_id: NodeId, graph: Entity<Graph<D>>, cx: &mut App) -> Entity<Self> {
        cx.new(|_cx| Self { node_id, graph })
    }
}

impl<D: GraphDef + 'static> Render for NodeView<D> {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .w(px(200.0))
            .h(px(100.0))
            .border_1()
            .border_color(cx.theme().border_color)
            .rounded(cx.theme().radius)
    }
}
