use super::node::NodeView;
use crate::{GpuiGraph, GpuiGraphState};
use flow::{Edge, GraphDef, NodeId, Socket};
use gpui::*;
use ui::utils::z_stack;

pub struct GraphView<D: GraphDef<State = GpuiGraphState>> {
    graph: Entity<GpuiGraph<D>>,

    node_views: Vec<Entity<NodeView<D>>>,
}

impl<D: GraphDef<State = GpuiGraphState> + 'static> GraphView<D> {
    pub fn build(graph: Entity<GpuiGraph<D>>, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            let mut this = Self { graph, node_views: Vec::new() };

            let node_ids = this.graph.read(cx).node_ids().copied().collect::<Vec<_>>();
            for node_id in node_ids {
                this.add_node(node_id, cx);
            }

            this
        })
    }

    pub fn add_node(&mut self, node_id: NodeId, cx: &mut Context<Self>) {
        let node_view = NodeView::build(node_id, self.graph.clone(), cx);
        self.node_views.push(node_view);
        cx.notify();
    }

    pub fn remove_node(&mut self, _node_id: NodeId, _cx: &mut Context<Self>) {
        todo!();
    }

    pub fn add_edge(&mut self, _edge: Edge, _cx: &mut Context<Self>) {
        todo!();
    }

    pub fn remove_edge(&mut self, _source: &Socket, _cx: &mut Context<Self>) {
        todo!();
    }

    pub fn graph(&self) -> &Entity<GpuiGraph<D>> {
        &self.graph
    }
}

impl<D: GraphDef<State = GpuiGraphState> + 'static> Render for GraphView<D> {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let nodes = div().children(self.node_views.clone()).relative().size_full();

        z_stack([nodes]).size_full()
    }
}
