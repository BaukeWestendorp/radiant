use std::collections::HashMap;

use flow::{Edge, GraphDef, NodeId, Socket};
use gpui::*;
use ui::{
    elements::{Draggable, DraggableEvent},
    utils::z_stack,
};

use super::node::NodeView;

pub struct GraphView<D: GraphDef> {
    graph: Entity<crate::Graph<D>>,
    node_views: HashMap<NodeId, Entity<Draggable>>,
}

impl<D: GraphDef + 'static> GraphView<D> {
    pub fn build(graph: Entity<crate::Graph<D>>, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            let mut this = Self { graph, node_views: HashMap::new() };

            let node_ids = this.graph.read(cx).node_ids().copied().collect::<Vec<_>>();
            for node_id in node_ids {
                this.add_node(node_id, cx);
            }

            this
        })
    }

    pub fn add_node(&mut self, node_id: NodeId, cx: &mut Context<Self>) {
        let draggable = cx.new(|cx| {
            Draggable::new(
                ElementId::NamedInteger("node".into(), node_id.0 as usize),
                self.graph.read(cx).node_position(&node_id).clone(),
                NodeView::build(node_id, self.graph.clone(), cx),
            )
        });

        cx.subscribe(&draggable, move |graph, _, event, cx| match event {
            DraggableEvent::PositionCommitted(position) => {
                graph.graph.update(cx, |graph, cx| {
                    graph.set_node_position(node_id, *position);
                    cx.notify();
                });
            }
        })
        .detach();

        self.node_views.insert(node_id, draggable);

        cx.notify();
    }

    pub fn remove_node(&mut self, node_id: NodeId, cx: &mut Context<Self>) {
        self.node_views.remove(&node_id);
        cx.notify();
    }

    pub fn add_edge(&mut self, _edge: Edge, _cx: &mut Context<Self>) {
        todo!();
    }

    pub fn remove_edge(&mut self, _source: &Socket, _cx: &mut Context<Self>) {
        todo!();
    }
}

impl<D: GraphDef + 'static> Render for GraphView<D> {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let nodes = div().children(self.node_views.values().cloned()).relative().size_full();

        z_stack([nodes]).size_full().text_sm()
    }
}
