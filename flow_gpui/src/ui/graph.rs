use flow::{Edge, GraphDef, NodeId, Socket};
use gpui::*;
use ui::utils::z_stack;

use super::node::NodeView;

pub struct GraphView<D: GraphDef> {
    graph: Entity<crate::Graph<D>>,
    node_views: Vec<Entity<NodeView<D>>>,
}

impl<D: GraphDef + 'static> GraphView<D> {
    pub fn build(graph: Entity<crate::Graph<D>>, cx: &mut App) -> Entity<Self> {
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

    pub fn remove_node(&mut self, node_id: NodeId, cx: &mut Context<Self>) {
        self.node_views.retain(|view| view.read(cx).node_id() != node_id);
        cx.notify();
    }

    pub fn add_edge(&mut self, _edge: Edge, _cx: &mut Context<Self>) {
        todo!();
    }

    pub fn remove_edge(&mut self, _source: &Socket, _cx: &mut Context<Self>) {
        todo!();
    }

    pub fn graph(&self) -> &Entity<crate::Graph<D>> {
        &self.graph
    }
}

impl<D: GraphDef + 'static> Render for GraphView<D> {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let nodes = div().children(self.node_views.clone()).relative().size_full();

        z_stack([nodes]).size_full()
    }
}
