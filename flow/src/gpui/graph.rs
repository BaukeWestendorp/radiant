use super::node::{NodeMeasurements, NodeView};
use crate::{AnySocket, DataType as _, GraphDef, InputSocket, NodeId, OutputSocket};
use gpui::*;
use std::collections::HashMap;
use ui::{Draggable, DraggableEvent, z_stack};

pub struct GraphView<D: GraphDef> {
    graph: Entity<crate::Graph<D>>,

    node_views: HashMap<NodeId, Entity<Draggable>>,
    new_edge: (Option<InputSocket>, Option<OutputSocket>),
}

impl<D: GraphDef + 'static> GraphView<D> {
    pub fn build(
        graph: Entity<crate::Graph<D>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<Self> {
        cx.new(|cx| {
            let mut this = Self { graph, node_views: HashMap::new(), new_edge: (None, None) };

            let node_ids = this.graph.read(cx).node_ids().copied().collect::<Vec<_>>();
            for node_id in node_ids {
                this.add_node(node_id, window, cx);
            }

            this
        })
    }

    pub fn graph(&self) -> &Entity<crate::Graph<D>> {
        &self.graph
    }

    pub fn add_node(&mut self, node_id: NodeId, window: &mut Window, cx: &mut Context<Self>) {
        let NodeMeasurements { snap_size, .. } = NodeMeasurements::new(window);

        let graph_view = cx.entity().clone();
        let draggable = cx.new(|cx| {
            Draggable::new(
                ElementId::NamedInteger("node".into(), node_id.0 as usize),
                *self.graph.read(cx).node_position(&node_id),
                Some(snap_size),
                NodeView::build(node_id, graph_view, self.graph.clone(), window, cx),
            )
        });

        cx.subscribe(&draggable, move |graph_view, _, event, cx| {
            graph_view.graph.update(cx, |graph, cx| {
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

    pub fn remove_node(&mut self, node_id: &NodeId, cx: &mut Context<Self>) {
        self.node_views.remove(node_id);
        cx.notify();
    }

    pub fn set_new_edge_socket(&mut self, from: &AnySocket, cx: &mut App) {
        match from {
            AnySocket::Input(input) => {
                // If the input already has an edge connected to it, remove it.
                self.graph.update(cx, |graph, cx| {
                    if graph.edge_source(input).is_some() {
                        graph.remove_edge(&input, cx);
                    }
                });

                self.new_edge.0 = Some(input.clone())
            }
            AnySocket::Output(output) => self.new_edge.1 = Some(output.clone()),
        }
    }

    pub fn drag_new_edge(
        &mut self,
        from_socket: &AnySocket,
        snap_distance: f32,
        window: &Window,
        cx: &mut App,
    ) {
        let end_position = window.mouse_position() - *self.graph.read(cx).offset();
        let squared_snap_distance = snap_distance * snap_distance;

        let square_dist = |a: Point<Pixels>, b: Point<Pixels>| {
            let dx = a.x - b.x;
            let dy = a.y - b.y;
            dx * dx + dy * dy
        };

        let node_ids = self.graph.read(cx).node_ids().cloned().collect::<Vec<_>>();
        match from_socket {
            AnySocket::Input(input_socket) => {
                let input = self.graph().read(cx).input(input_socket);

                // Find the closest output socket.
                for node_id in node_ids {
                    let node = self.graph.read(cx).node(&node_id);
                    let template = self.graph.read(cx).template(node.template_id());

                    for output in template.outputs() {
                        let source = OutputSocket::new(node_id, output.id().to_string());
                        let position = self.get_connector_position(
                            &AnySocket::Output(source.clone()),
                            window,
                            cx,
                        );

                        // If the edge is close enough to snap to the output socket
                        if square_dist(position, end_position) < px(squared_snap_distance) {
                            // And it's allowed to snap
                            if output.data_type().can_cast_to(&input.data_type()) {
                                // Snap to the output socket
                                self.new_edge.1 = Some(source);
                                return;
                            }
                        }
                    }
                }

                self.new_edge.1 = None;
            }
            AnySocket::Output(output_socket) => {
                let output = self.graph().read(cx).output(output_socket);

                // Find the closest input socket.
                for node_id in node_ids {
                    let node = self.graph.read(cx).node(&node_id);
                    let template = self.graph.read(cx).template(node.template_id());

                    for input in template.inputs() {
                        let target = InputSocket::new(node_id, input.id().to_string());
                        let position = self.get_connector_position(
                            &AnySocket::Input(target.clone()),
                            window,
                            cx,
                        );

                        // If the edge is close enough to snap to the output socket
                        if square_dist(position, end_position) < px(squared_snap_distance) {
                            // And it's allowed to snap
                            if output.data_type().can_cast_to(&input.data_type()) {
                                // Snap to the output socket
                                self.new_edge.0 = Some(target);
                                return;
                            }
                        }
                    }
                }

                self.new_edge.0 = None;
            }
        }
    }

    pub fn finish_new_edge(&mut self, cx: &mut Context<Self>) {
        match self.new_edge.clone() {
            (Some(target), Some(source)) => {
                self.graph().update(cx, |graph, cx| {
                    graph.add_edge(target, source, cx);
                    cx.notify();
                });
            }
            (None, Some(source)) => {
                cx.emit(VisualGraphEvent::EdgeTargetRequested { source: source.clone() });
            }
            (Some(target), None) => {
                cx.emit(VisualGraphEvent::EdgeSourceRequested { target: target.clone() });
            }
            _ => (),
        }
        self.new_edge = (None, None);
    }

    fn render_edges(&self, window: &Window, cx: &App) -> Div {
        let edges = self.graph.read(cx).edges();

        z_stack(edges.map(|(target, source)| {
            let target_pos =
                self.get_connector_position(&AnySocket::Input(target.clone()), window, cx);
            let source_pos =
                self.get_connector_position(&AnySocket::Output(source.clone()), window, cx);

            let target = self.graph.read(cx).input(target);
            let source = self.graph.read(cx).output(source);

            self.render_edge(target_pos, source_pos, &target.data_type(), source.data_type())
        }))
    }

    fn render_new_edge(&self, window: &Window, cx: &App) -> Div {
        let relative_mouse_pos = window.mouse_position() - *self.graph().read(cx).offset();
        let (source_pos, target_pos, source_type, target_type) = match &self.new_edge {
            (None, None) => return div(),
            (None, Some(source)) => {
                let source_pos =
                    self.get_connector_position(&AnySocket::Output(source.clone()), window, cx);
                let target_pos = relative_mouse_pos;

                let source = self.graph.read(cx).output(&source);
                (source_pos, target_pos, source.data_type(), source.data_type())
            }
            (Some(target), None) => {
                let target_pos =
                    self.get_connector_position(&AnySocket::Input(target.clone()), window, cx);
                let source_pos = relative_mouse_pos;

                let target = self.graph.read(cx).input(&target);
                (source_pos, target_pos, &target.data_type(), &target.data_type())
            }
            (Some(target), Some(source)) => {
                let source_pos =
                    self.get_connector_position(&AnySocket::Output(source.clone()), window, cx);
                let target_pos =
                    self.get_connector_position(&AnySocket::Input(target.clone()), window, cx);

                let source = self.graph.read(cx).output(&source);
                let target = self.graph.read(cx).input(&target);
                (source_pos, target_pos, &target.data_type(), source.data_type())
            }
        };

        self.render_edge(source_pos, target_pos, source_type, target_type)
    }

    fn render_edge(
        &self,
        target_pos: Point<Pixels>,
        source_pos: Point<Pixels>,
        target_data_type: &D::DataType,
        source_data_type: &D::DataType,
    ) -> Div {
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

    fn get_connector_position(
        &self,
        socket: &AnySocket,
        window: &Window,
        cx: &App,
    ) -> Point<Pixels> {
        // FIXME: This is a bit hacky. It might be possible to get the node position from the layout.
        //        Just trying to get it working for now...

        let node_id = match socket {
            AnySocket::Input(socket) => socket.node_id,
            AnySocket::Output(socket) => socket.node_id,
        };
        let node = self.graph.read(cx).node(&node_id);

        let NodeMeasurements {
            snap_size,
            sockets_padding_y,
            width,
            header_height,
            socket_height,
            socket_gap_y,
            ..
        } = NodeMeasurements::new(window);

        let template = self.graph.read(cx).template(node.template_id());
        let node_position =
            ui::snap_point(*self.graph.read(cx).visual_node_position(&node_id), snap_size);

        let socket_index = match socket {
            AnySocket::Input(input) => template
                .inputs()
                .iter()
                .position(|i| i.id() == input.id)
                .expect(&format!("should get index of input for socket {:?}", socket)),
            AnySocket::Output(output) => {
                template.inputs().len() + // Move past all input sockets.
                    template.outputs().iter().position(|o| o.id() == output.id)
                    .expect(&format!("should get index of input for socket {:?}", socket))
            }
        };

        let x_offset = match socket {
            AnySocket::Input(_) => px(0.0), // Move to the left edge of the node for input sockets.
            AnySocket::Output(_) => width, // Move to the right edge of the node for output sockets.
        };
        let y_offset = header_height + // Move below the header.
            sockets_padding_y + // Move below the content's vertical padding.
            socket_index as f32 * (socket_height + socket_gap_y) + // Move to the correct socket.
            socket_height / 2.0 + // Move to the center of the socket.
            px(1.0);

        point(node_position.x + x_offset, node_position.y + y_offset)
    }
}

impl<D: GraphDef + 'static> GraphView<D> {
    fn handle_mouse_up(
        &mut self,
        _event: &MouseUpEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.finish_new_edge(cx);
    }
}

impl<D: GraphDef + 'static> Render for GraphView<D> {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let nodes = div().children(self.node_views.values().cloned()).relative().size_full();
        let edges = self.render_edges(window, cx);
        let new_edge = self.render_new_edge(window, cx);

        z_stack([nodes, edges, new_edge])
            .size_full()
            .on_mouse_up(MouseButton::Left, cx.listener(Self::handle_mouse_up))
            .on_mouse_up_out(MouseButton::Left, cx.listener(Self::handle_mouse_up))
    }
}

#[derive(Debug, Clone)]
pub enum VisualGraphEvent {
    EdgeTargetRequested { source: OutputSocket },
    EdgeSourceRequested { target: InputSocket },
}

impl<D: GraphDef + 'static> EventEmitter<VisualGraphEvent> for GraphView<D> {}
