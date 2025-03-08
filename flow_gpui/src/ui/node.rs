use flow::{GraphDef, NodeId};
use gpui::*;
use ui::theme::ActiveTheme;

pub struct NodeView<D: GraphDef> {
    node_id: NodeId,
    graph: Entity<crate::Graph<D>>,
}

impl<D: GraphDef + 'static> NodeView<D> {
    pub fn build(node_id: NodeId, graph: Entity<crate::Graph<D>>, cx: &mut App) -> Entity<Self> {
        cx.new(move |_cx| Self { node_id, graph })
    }

    pub fn node_id(&self) -> NodeId {
        self.node_id
    }
}

impl<D: GraphDef + 'static> Render for NodeView<D> {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let position = self.graph.read(cx).node_position(&self.node_id);

        div()
            .absolute()
            .left(px(position.0))
            .top(px(position.1))
            .w(px(200.0))
            .h(px(100.0))
            .border_1()
            .border_color(cx.theme().border_color)
            .rounded(cx.theme().radius)
    }
}
