use gpui::*;
use ui::z_stack;

use crate::{DataType, Graph, NodeId, NodeKind};

use super::node::{self, NodeView, Socket};

pub struct GraphView<D, V, N>
where
    D: DataType<Value = V>,
    N: NodeKind<DataType = D, Value = V>,
{
    graph: Model<Graph<D, V, N>>,
    node_views: Vec<View<node::NodeView<D, V, N>>>,
}

impl<D, V, N> GraphView<D, V, N>
where
    D: DataType<Value = V> + 'static,
    V: 'static,
    N: NodeKind<DataType = D, Value = V> + 'static,
{
    pub fn build(graph: Model<Graph<D, V, N>>, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            // FIXME: Fuck this.
            let node_ids = graph
                .read(cx)
                .nodes
                .values()
                .map(|node| node.id)
                .collect::<Vec<_>>();
            let node_views = node_ids
                .into_iter()
                .map(|id| NodeView::build(id, graph.clone(), cx))
                .collect::<Vec<_>>();

            Self { graph, node_views }
        })
    }

    fn render_connections(&self, cx: &ViewContext<Self>) -> impl IntoElement {
        let connections = &self.graph.read(cx).connections;

        z_stack(connections.iter().map(|(target_id, source_id)| {
            let target = self.graph.read(cx).input(target_id);
            let source = self.graph.read(cx).output(*source_id);

            let target_pos = self.get_socket_position(target.node, &Socket::Input(target_id), cx);
            let source_pos = self.get_socket_position(source.node, &Socket::Output(*source_id), cx);

            self.render_connection(
                &target_pos,
                &source_pos,
                &target.data_type,
                &source.data_type,
            )
        }))
    }

    fn render_connection(
        &self,
        source_pos: &Point<Pixels>,
        target_pos: &Point<Pixels>,
        target_data_type: &D,
        source_data_type: &D,
    ) -> impl IntoElement {
        // FIXME: This is a mess. Once GPUI supports actual bezier paths, use them.

        let x_dist = source_pos.x - target_pos.x;
        let y_dist = source_pos.y - target_pos.y;

        let source_horizontal = div()
            .absolute()
            .left(if source_pos.x < target_pos.x {
                source_pos.x
            } else {
                target_pos.x + x_dist.abs() / 2.0
            })
            .w(x_dist.abs() / 2.0)
            .top(source_pos.y)
            .h_px()
            .bg(source_data_type.color());

        let source_vertical = div()
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
            .bg(source_data_type.color());

        let target_vertical = div()
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
            .bg(target_data_type.color());

        let target_horizontal = div()
            .absolute()
            .left(if source_pos.x < target_pos.x {
                source_pos.x + x_dist.abs() / 2.0
            } else {
                target_pos.x
            })
            .w(x_dist.abs() / 2.0)
            .top(target_pos.y)
            .h_px()
            .bg(target_data_type.color());

        z_stack([
            source_horizontal,
            source_vertical,
            target_vertical,
            target_horizontal,
        ])
    }

    fn get_socket_position(
        &self,
        node_id: NodeId,
        socket: &Socket,
        cx: &AppContext,
    ) -> Point<Pixels> {
        // FIXME: This is a bit hacky. It might be possible to get the node position from the layout.
        //        Just trying to get it working for now...

        let node_position = self.graph.read(cx).node(node_id).position;
        let node = self.graph.read(cx).node(node_id);
        let socket_index = match socket {
            Socket::Input(id) => node.inputs.iter().position(|(_, i)| i == id).unwrap(),
            Socket::Output(id) => {
                node.inputs.len() + // Move past all input sockets.
                node.outputs.iter().position(|(_, i)| i == id).unwrap()
            }
        };

        let x_offset = match socket {
            Socket::Input(_) => 0.0, // Move to the left edge of the node for input sockets.
            Socket::Output(_) => node::NODE_WIDTH.into(), // Move to the right edge of the node for output sockets.
        };
        let y_offset = node::HEADER_HEIGHT.0 + // Move below the header.
            node::NODE_CONTENT_Y_PADDING.0 + // Move below the content's vertical padding.
            socket_index as f32 * (node::SOCKET_HEIGHT.0 + node::SOCKET_GAP.0) + // Move to the correct socket.
           node::SOCKET_HEIGHT.0 / 2.0 + // Move to the center of the socket.
            -1.0; // FIXME: Why do we need this to actually move it to the center?

        node_position + point(px(x_offset), px(y_offset))
    }
}

impl<D, V, N> Render for GraphView<D, V, N>
where
    D: DataType<Value = V> + 'static,
    V: 'static,
    N: NodeKind<DataType = D, Value = V> + 'static,
{
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        z_stack([
            div()
                .size_24()
                .children(self.node_views.clone())
                .into_any_element(),
            self.render_connections(cx).into_any_element(),
        ])
    }
}
