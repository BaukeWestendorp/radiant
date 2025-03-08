use flow::{GraphDef, NodeId};
use gpui::*;
use ui::theme::ActiveTheme;

use crate::{GpuiGraph, GpuiGraphState};

pub struct NodeView<D: GraphDef<State = GpuiGraphState>> {
    node_id: NodeId,
    graph: Entity<GpuiGraph<D>>,
}

impl<D: GraphDef<State = GpuiGraphState> + 'static> NodeView<D> {
    pub fn build(node_id: NodeId, graph: Entity<GpuiGraph<D>>, cx: &mut App) -> Entity<Self> {
        cx.new(|_cx| Self { node_id, graph })
    }
}

impl<D: GraphDef<State = GpuiGraphState> + 'static> Render for NodeView<D> {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = self.graph.read(cx).state();
        let node_position = state.node_position(&self.node_id).copied().unwrap_or_default();

        div()
            .absolute()
            .left(px(node_position.x))
            .top(px(node_position.y))
            .w(px(200.0))
            .h(px(100.0))
            .border_1()
            .border_color(cx.theme().border_color)
            .rounded(cx.theme().radius)
    }
}
