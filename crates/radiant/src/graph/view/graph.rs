use gpui::*;
use ui::{theme::ActiveTheme, z_stack};

use crate::graph::{node::Socket, DataType, Graph, InputId, NodeId, OutputId};

use super::node::{self, NodeView, SocketEvent};

pub struct GraphView {
    graph: Model<Graph>,
    nodes: Vec<View<NodeView>>,
    new_connection: (Option<OutputId>, Option<InputId>),
}

impl GraphView {
    pub fn build(graph: Model<Graph>, cx: &mut WindowContext) -> View<Self> {
        let graph = cx.new_view(|cx| Self {
            nodes: Self::build_nodes(&graph, cx),
            graph,
            new_connection: (None, None),
        });

        graph
    }

    fn build_nodes(graph: &Model<Graph>, cx: &mut ViewContext<Self>) -> Vec<View<NodeView>> {
        let nodes = graph.read(cx).node_ids().collect::<Vec<_>>();

        nodes
            .into_iter()
            .map(|id| {
                let node_view = NodeView::build(id, graph.clone(), cx);
                cx.subscribe(&node_view, |this, _, event: &SocketEvent, cx| {
                    this.handle_socket_event(event, cx);
                })
                .detach();
                node_view
            })
            .collect()
    }

    fn handle_socket_event(&mut self, event: &SocketEvent, cx: &mut ViewContext<Self>) {
        let end_position = cx.mouse_position();

        let square_dist = |a: Point<Pixels>, b: Point<Pixels>| {
            let dx = a.x - b.x;
            let dy = a.y - b.y;
            dx * dx + dy * dy
        };

        let graph = self.graph.read(cx);
        let input_ids = graph.inputs.keys().collect::<Vec<_>>();
        let output_ids = graph.outputs.keys().collect::<Vec<_>>();

        let find_closest_input = || {
            for input_id in input_ids {
                let input = graph.input(input_id);
                let position = self.get_socket_position(input.node, &Socket::Input(input_id), cx);

                if square_dist(position, end_position) < px(100.0) {
                    return Some(input_id);
                }
            }
            None
        };

        let find_closest_output = || {
            for output_id in output_ids {
                let output = graph.output(output_id);
                let position =
                    self.get_socket_position(output.node, &Socket::Output(output_id), cx);

                if square_dist(position, end_position) < px(100.0) {
                    return Some(output_id);
                }
            }
            None
        };

        match event {
            SocketEvent::StartNewConnection(socket) => {
                match socket {
                    Socket::Input(input_id) => {
                        let target_id = Some(*input_id);
                        let source_id = match find_closest_output() {
                            Some(closest_output) => Some(closest_output),
                            None => None,
                        };

                        if let Some(source_id) = source_id {
                            if !graph.check_connection_validity(*input_id, source_id) {
                                return;
                            }
                        }

                        self.new_connection = (source_id, target_id);

                        self.graph.update(cx, |graph, cx| {
                            graph.remove_connection(*input_id);
                            cx.notify();
                        });
                    }
                    Socket::Output(output_id) => {
                        let source_id = Some(*output_id);
                        let target_id = match find_closest_input() {
                            Some(closest_input) => Some(closest_input),
                            None => None,
                        };

                        if let Some(target_id) = target_id {
                            if !graph.check_connection_validity(target_id, *output_id) {
                                return;
                            }
                        }

                        self.new_connection = (source_id, target_id);

                        self.graph.update(cx, |graph, cx| {
                            if let Some(input_id) = graph.connection_target(*output_id) {
                                graph.remove_connection(input_id);
                                cx.notify();
                            }
                        });
                    }
                }
                cx.notify();
            }
            SocketEvent::EndNewConnection => {
                match self.new_connection {
                    (Some(source_id), None) => {
                        if let Some(closest_input) = find_closest_input() {
                            self.graph.update(cx, |graph, cx| {
                                graph.add_connection(closest_input, source_id);
                                cx.notify();
                            });
                            cx.notify();
                        }
                    }
                    (None, Some(target_id)) => {
                        if let Some(closest_output) = find_closest_output() {
                            self.graph.update(cx, |graph, cx| {
                                graph.add_connection(target_id, closest_output);
                                cx.notify();
                            });
                            cx.notify();
                        }
                    }
                    (Some(source_id), Some(target_id)) => {
                        self.graph.update(cx, |graph, cx| {
                            graph.add_connection(target_id, source_id);
                            cx.notify();
                        });
                        cx.notify();
                    }
                    _ => {}
                }

                self.new_connection = (None, None);
            }
        }
    }

    fn render_connections(&self, cx: &ViewContext<Self>) -> Div {
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

    fn render_new_connection(&self, cx: &ViewContext<Self>) -> Div {
        let (source_pos, target_pos, source_data_type, target_data_type) = match self.new_connection
        {
            (None, None) => return div(),
            (None, Some(target_id)) => {
                let target = self.graph.read(cx).input(target_id);
                let target_pos =
                    self.get_socket_position(target.node, &Socket::Input(target_id), cx);
                let source_pos = cx.mouse_position();
                let source_data_type = &target.data_type;
                let target_data_type = &target.data_type;
                (source_pos, target_pos, source_data_type, target_data_type)
            }
            (Some(source_id), None) => {
                let source = self.graph.read(cx).output(source_id);
                let source_pos =
                    self.get_socket_position(source.node, &Socket::Output(source_id), cx);
                let target_pos = cx.mouse_position();
                let source_data_type = &source.data_type;
                let target_data_type = &source.data_type;
                (source_pos, target_pos, source_data_type, target_data_type)
            }
            (Some(source_id), Some(target_id)) => {
                let source = self.graph.read(cx).output(source_id);
                let target = self.graph.read(cx).input(target_id);
                let source_pos =
                    self.get_socket_position(source.node, &Socket::Output(source_id), cx);
                let target_pos =
                    self.get_socket_position(target.node, &Socket::Input(target_id), cx);
                let source_data_type = &source.data_type;
                let target_data_type = &target.data_type;
                (source_pos, target_pos, source_data_type, target_data_type)
            }
        };

        self.render_connection(&source_pos, &target_pos, source_data_type, target_data_type)
    }

    fn render_connection(
        &self,
        source_pos: &Point<Pixels>,
        target_pos: &Point<Pixels>,
        target_data_type: &DataType,
        source_data_type: &DataType,
    ) -> Div {
        // FIXME: This is a mess. Once GPUI supports actual bezier paths, use them.

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

        z_stack([
            target_horizontal,
            target_vertical,
            source_vertical,
            source_horizontal,
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
            1.0; // FIXME: Why do we need this to actually move it to the center?

        node_position + point(px(x_offset), px(y_offset))
    }
}

impl Render for GraphView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        z_stack([
            div().children(self.nodes.clone()),
            self.render_connections(cx),
            self.render_new_connection(cx),
        ])
        .size_full()
        .bg(cx.theme().background)
        .text_color(cx.theme().foreground)
        .text_size(cx.theme().font_size)
        .font_family(cx.theme().font_family.clone())
    }
}
