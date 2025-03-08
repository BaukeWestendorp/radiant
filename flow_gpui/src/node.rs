use flow::{GraphDef, NodeId};
use gpui::*;
use ui::theme::ActiveTheme;

use crate::{EventEmittingGraph, GpuiGraphState};

pub struct NodeView<D>
where
    D: GraphDef<State = GpuiGraphState>,
{
    node_id: NodeId,
    graph: Entity<EventEmittingGraph<D>>,
}

impl<D> NodeView<D>
where
    D: GraphDef<State = GpuiGraphState> + 'static,
{
    pub fn build(
        node_id: NodeId,
        graph: Entity<EventEmittingGraph<D>>,
        cx: &mut App,
    ) -> Entity<Self> {
        cx.new(|_cx| Self { node_id, graph })
    }
}

impl<D> Render for NodeView<D>
where
    D: GraphDef<State = GpuiGraphState> + 'static,
{
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let node_position = self.graph.read(cx).state().node_position(&self.node_id);
        dbg!(node_position);

        div()
            .w(px(200.0))
            .h(px(100.0))
            .border_1()
            .border_color(cx.theme().border_color)
            .rounded(cx.theme().radius)
    }
}
