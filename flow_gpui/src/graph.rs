use super::node::NodeView;
use crate::{GpuiGraph, GpuiGraphState};
use flow::{GraphDef, NodeId};
use gpui::*;
use ui::utils::z_stack;

pub struct GraphView<D: GraphDef<State = GpuiGraphState>> {
    graph: Entity<GpuiGraph<D>>,

    node_views: Vec<Entity<NodeView<D>>>,
}

impl<D: GraphDef<State = GpuiGraphState> + 'static> GraphView<D> {
    pub fn build(graph: Entity<GpuiGraph<D>>, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            let node_views = Self::build_nodes(&graph, cx);

            Self { graph, node_views }
        })
    }

    pub fn graph(&self) -> &Entity<GpuiGraph<D>> {
        &self.graph
    }

    fn build_nodes(graph: &Entity<GpuiGraph<D>>, cx: &mut App) -> Vec<Entity<NodeView<D>>> {
        let node_ids = graph.read(cx).node_ids().cloned().collect::<Vec<_>>();
        node_ids.into_iter().map(|node_id| Self::build_node(node_id, graph.clone(), cx)).collect()
    }

    fn build_node(
        node_id: NodeId,
        graph: Entity<GpuiGraph<D>>,
        cx: &mut App,
    ) -> Entity<NodeView<D>> {
        NodeView::build(node_id, graph, cx)
    }
}

impl<D: GraphDef<State = GpuiGraphState> + 'static> Render for GraphView<D> {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        z_stack([div().children(self.node_views.clone())]).size_full()
    }
}
