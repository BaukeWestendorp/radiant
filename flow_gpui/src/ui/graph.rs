use std::collections::HashMap;

use flow::{AnySocket, Edge, GraphDef, NodeId, Socket};
use gpui::*;
use ui::{
    element::{Draggable, DraggableEvent},
    z_stack,
};

use crate::DataType;

use super::node::{self, NodeView, SNAP_GRID_SIZE};

pub struct GraphView<D: GraphDef> {
    graph: Entity<crate::Graph<D>>,

    node_views: HashMap<NodeId, Entity<Draggable>>,
}

impl<D: GraphDef + 'static> GraphView<D>
where
    D::DataType: crate::DataType<D>,
{
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
                *self.graph.read(cx).node_position(&node_id),
                Some(SNAP_GRID_SIZE),
                NodeView::build(node_id, self.graph.clone(), cx),
            )
        });

        cx.subscribe(&draggable, move |graph, _, event, cx| {
            graph.graph.update(cx, |graph, cx| {
                match event {
                    DraggableEvent::PositionChanged(position) => {
                        graph.update_visual_node_position(Some((node_id, *position)));
                    }
                    DraggableEvent::PositionCommitted(position) => {
                        graph.set_node_position(node_id, *position);
                        graph.update_visual_node_position(None);
                    }
                }
                cx.notify();
            });
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

    fn render_edges(&self, cx: &App) -> Div {
        let edges = self.graph.read(cx).edges();

        z_stack(edges.map(|Edge { target, source }| {
            let target_pos = self.get_socket_position(&AnySocket::Input(target.clone()), cx);
            let source_pos = self.get_socket_position(&AnySocket::Output(source.clone()), cx);

            let target = self.graph.read(cx).input(target);
            let source = self.graph.read(cx).output(source);

            self.render_edge(target_pos, source_pos, target.data_type(), source.data_type())
        }))
    }

    fn render_edge(
        &self,
        target_pos: Point<Pixels>,
        source_pos: Point<Pixels>,
        target_data_type: &D::DataType,
        source_data_type: &D::DataType,
    ) -> impl IntoElement {
        let x_dist = source_pos.x - target_pos.x;
        let y_dist = source_pos.y - target_pos.y;

        let target_horizontal = div()
            .absolute()
            .left(if source_pos.x < target_pos.x {
                source_pos.x
            } else {
                target_pos.x + x_dist.abs() / 2.0
            })
            .w(x_dist.abs() / 2.0)
            .top(source_pos.y)
            .h_px()
            .bg(target_data_type.color());

        let target_vertical = div()
            .absolute()
            .left(if source_pos.x < target_pos.x {
                source_pos.x + x_dist.abs() / 2.0
            } else {
                target_pos.x + x_dist.abs() / 2.0
            })
            .w_px()
            .top(if source_pos.y < target_pos.y {
                source_pos.y
            } else {
                target_pos.y + y_dist.abs() / 2.0
            })
            .h(y_dist.abs() / 2.0)
            .bg(target_data_type.color());

        let source_vertical = div()
            .absolute()
            .left(if source_pos.x < target_pos.x {
                target_pos.x - x_dist.abs() / 2.0
            } else {
                target_pos.x + x_dist.abs() / 2.0
            })
            .w_px()
            .top(if source_pos.y < target_pos.y {
                source_pos.y + y_dist.abs() / 2.0
            } else {
                target_pos.y
            })
            .h(y_dist.abs() / 2.0)
            .bg(source_data_type.color());

        let source_horizontal = div()
            .absolute()
            .left(if source_pos.x < target_pos.x {
                source_pos.x + x_dist.abs() / 2.0
            } else {
                target_pos.x
            })
            .w(x_dist.abs() / 2.0)
            .top(target_pos.y)
            .h_px()
            .bg(source_data_type.color());

        z_stack([target_horizontal, target_vertical, source_vertical, source_horizontal])
    }

    fn get_socket_position(&self, any_socket: &AnySocket, cx: &App) -> Point<Pixels> {
        // FIXME: This is a bit hacky. It might be possible to get the node position from the layout.
        //        Just trying to get it working for now...

        let node_id = &any_socket.socket().node_id;
        let node = self.graph.read(cx).node(node_id);
        let template = self.graph.read(cx).template(node.template_id());
        let node_position = ui::snap_point(
            *self.graph.read(cx).visual_node_position(node_id),
            node::SNAP_GRID_SIZE,
        );

        let socket_index = match any_socket {
            AnySocket::Input(input) => template
                .inputs()
                .iter()
                .position(|i| i.id() == input.id)
                .expect("should get index of input"),
            AnySocket::Output(output) => {
                template.inputs().len() + // Move past all input sockets.
                    template.outputs().iter().position(|o| o.id()== output.id).expect("should get index of output")
            }
        };

        let x_offset = match any_socket {
            AnySocket::Input(_) => px(0.0), // Move to the left edge of the node for input sockets.
            AnySocket::Output(_) => node::NODE_WIDTH, // Move to the right edge of the node for output sockets.
        };
        let y_offset = node::HEADER_HEIGHT + // Move below the header.
            node::NODE_CONTENT_Y_PADDING + // Move below the content's vertical padding.
            socket_index as f32 * (node::SOCKET_HEIGHT + node::SOCKET_GAP) + // Move to the correct socket.
            node::SOCKET_HEIGHT / 2.0 + // Move to the center of the socket.
            px(1.0);

        point(node_position.x + x_offset, node_position.y + y_offset)
    }
}

impl<D: GraphDef + 'static> Render for GraphView<D>
where
    D::DataType: crate::DataType<D>,
{
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let nodes = div().children(self.node_views.values().cloned()).relative().size_full();
        let edges = self.render_edges(cx);

        z_stack([nodes, edges]).size_full().text_sm()
    }
}
