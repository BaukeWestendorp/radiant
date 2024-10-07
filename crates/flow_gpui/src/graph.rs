use flow::{Graph, GraphNodeKind, InputId, Node, NodeId, OutputId};
use gpui::prelude::FluentBuilder;
use gpui::*;
use slotmap::SecondaryMap;
use ui::{z_stack, StyledExt};

use crate::geo;

const HEADER_HEIGHT: Pixels = px(24.0);
const SOCKET_HEIGHT: Pixels = px(12.0);
const NODE_CONTENT_Y_PADDING: Pixels = px(6.0);
const SOCKET_GAP: Pixels = px(12.0);
const NODE_WIDTH: Pixels = px(200.0);

pub enum Socket {
    Input(InputId),
    Output(OutputId),
}

pub trait VisualDataType {
    fn color(&self) -> Hsla;
}

pub trait VisualNode {
    fn label(&self) -> &str;
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct VisualGraphState {
    node_positions: SecondaryMap<NodeId, geo::Point>,
}

impl VisualGraphState {
    pub fn get_node_position(&self, node_id: NodeId) -> geo::Point {
        self.node_positions[node_id]
    }

    pub fn set_node_position(&mut self, node_id: NodeId, position: geo::Point) {
        self.node_positions.insert(node_id, position);
    }
}

struct DraggedNode {
    pub node_id: NodeId,
    pub start_node_position: Point<Pixels>,
    pub start_mouse_position: Point<Pixels>,
}

pub struct GraphView<D, V, N: GraphNodeKind<DataType = D, Value = V> + VisualNode> {
    graph: Model<Graph<D, V, N>>,
    visual_graph_state: VisualGraphState,
}

impl<
        D: VisualDataType + 'static,
        V: 'static,
        N: GraphNodeKind<DataType = D, Value = V> + VisualNode + 'static,
    > GraphView<D, V, N>
{
    pub fn build(
        graph: Model<Graph<D, V, N>>,
        visual_graph_state: VisualGraphState,
        cx: &mut WindowContext,
    ) -> View<Self> {
        cx.new_view(|_cx| Self {
            visual_graph_state,
            graph,
        })
    }

    fn render_node(&self, node: &Node<D, V, N>, cx: &ViewContext<Self>) -> impl IntoElement {
        let header = {
            let label = node.kind.label().to_owned();

            div()
                .h_flex()
                .gap_1()
                .px_1()
                .py_px()
                .border_b_1()
                .border_color(rgb(0x404040))
                .child(label)
        };

        let content = {
            let inputs = node.inputs.clone().into_iter().map(|(label, input_id)| {
                let input = self.graph.read(cx).input(input_id);

                div()
                    .h_flex()
                    .h(SOCKET_HEIGHT)
                    .gap_2()
                    .child(self.render_socket(&input.data_type, false))
                    .child(label.clone())
            });

            let outputs = node.outputs.clone().into_iter().map(|(label, output_id)| {
                let output = self.graph.read(cx).output(output_id);

                div()
                    .h_flex()
                    .h(SOCKET_HEIGHT)
                    .w_full()
                    .flex_row_reverse()
                    .gap_2()
                    .child(self.render_socket(&output.data_type, true))
                    .child(label.clone())
            });

            div()
                .min_h_10()
                .v_flex()
                .gap(SOCKET_GAP)
                .py(NODE_CONTENT_Y_PADDING)
                .children(inputs)
                .children(outputs)
        };

        let node_position = self.visual_graph_state.get_node_position(node.id);

        div()
            // FIXME: This way of creating an id feels hacky.
            .id(ElementId::Name(format!("node-{:?}", node.id).into()))
            .absolute()
            .left(px(node_position.x))
            .top(px(node_position.y))
            .w(NODE_WIDTH)
            .bg(rgb(0x181818))
            .border_1()
            .border_color(rgb(0x404040))
            .rounded_md()
            .hover(|v| v) // FIXME: This is a hack to prevent a weird movement issue when dragging for some reason?
            .child(header)
            .child(content)
            .on_drag(
                DraggedNode {
                    node_id: node.id,
                    start_node_position: node_position.into(),
                    start_mouse_position: cx.mouse_position(),
                },
                |_, cx| cx.new_view(|_cx| EmptyView),
            )
            .on_drag_move(cx.listener(Self::node_on_drag_move))
    }

    fn render_socket(&self, data_type: &D, left: bool) -> impl IntoElement {
        div()
            .w_1()
            .h(SOCKET_HEIGHT)
            .bg(data_type.color())
            .rounded_r_sm()
            .when(left, |e| e.rounded_r_none().rounded_l_sm())
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
        source_pos: &geo::Point,
        target_pos: &geo::Point,
        target_data_type: &D,
        source_data_type: &D,
    ) -> impl IntoElement {
        // FIXME: This is a mess. Once GPUI supports actual bezier paths, use them.

        let x_dist = px(source_pos.x - target_pos.x);
        let y_dist = px(source_pos.y - target_pos.y);

        let source_horizontal = div()
            .absolute()
            .left(if source_pos.x < target_pos.x {
                px(source_pos.x)
            } else {
                px(target_pos.x) + x_dist.abs() / 2.0
            })
            .w(x_dist.abs() / 2.0)
            .top(px(source_pos.y))
            .h_px()
            .bg(source_data_type.color());

        let source_vertical = div()
            .absolute()
            .left(if source_pos.x < target_pos.x {
                px(source_pos.x) + x_dist.abs() / 2.0
            } else {
                px(target_pos.x) + x_dist.abs() / 2.0
            })
            .w_px()
            .top(if source_pos.y < target_pos.y {
                px(source_pos.y)
            } else {
                px(target_pos.y) + y_dist.abs() / 2.0
            })
            .h(y_dist.abs() / 2.0)
            .bg(source_data_type.color());

        let target_vertical = div()
            .absolute()
            .left(if source_pos.x < target_pos.x {
                px(target_pos.x) - x_dist.abs() / 2.0
            } else {
                px(target_pos.x) + x_dist.abs() / 2.0
            })
            .w_px()
            .top(if source_pos.y < target_pos.y {
                px(source_pos.y) + y_dist.abs() / 2.0
            } else {
                px(target_pos.y)
            })
            .h(y_dist.abs() / 2.0)
            .bg(target_data_type.color());

        let target_horizontal = div()
            .absolute()
            .left(if source_pos.x < target_pos.x {
                px(source_pos.x) + x_dist.abs() / 2.0
            } else {
                px(target_pos.x)
            })
            .w(x_dist.abs() / 2.0)
            .top(px(target_pos.y))
            .h_px()
            .bg(target_data_type.color());

        z_stack([
            source_horizontal,
            source_vertical,
            target_vertical,
            target_horizontal,
        ])
    }

    fn get_socket_position(&self, node_id: NodeId, socket: &Socket, cx: &AppContext) -> geo::Point {
        // FIXME: This is a bit hacky. It might be possible to get the node position from the layout.
        //        Just trying to get it working for now...

        let node_position = self.visual_graph_state.get_node_position(node_id);
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
            Socket::Output(_) => NODE_WIDTH.into(), // Move to the right edge of the node for output sockets.
        };
        let y_offset = HEADER_HEIGHT.0 + // Move below the header.
            NODE_CONTENT_Y_PADDING.0 + // Move below the content's vertical padding.
            socket_index as f32 * (SOCKET_HEIGHT.0 + SOCKET_GAP.0) + // Move to the correct socket.
            SOCKET_HEIGHT.0 / 2.0 + // Move to the center of the socket.
            -1.0; // FIXME: Why do we need this to actually move it to the center?

        node_position + geo::Point::new(x_offset, y_offset)
    }

    fn node_on_drag_move(
        &mut self,
        event: &DragMoveEvent<DraggedNode>,
        cx: &mut ViewContext<Self>,
    ) {
        let dragged_node = event.drag(cx);
        let new_position = dragged_node.start_node_position
            + (cx.mouse_position() - dragged_node.start_mouse_position);

        self.visual_graph_state
            .set_node_position(dragged_node.node_id, new_position.into());
    }
}

impl<
        D: VisualDataType + 'static,
        V: 'static,
        N: GraphNodeKind<DataType = D, Value = V> + VisualNode + 'static,
    > Render for GraphView<D, V, N>
{
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let nodes = self
            .graph
            .read(cx)
            .nodes
            .values()
            .map(|node| self.render_node(node, cx));

        z_stack([
            div().size_full().children(nodes).into_any_element(),
            self.render_connections(cx).into_any_element(),
        ])
    }
}
