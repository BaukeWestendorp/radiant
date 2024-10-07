use gpui::*;

use crate::{InputId, OutputId};

pub(crate) const NODE_CONTENT_Y_PADDING: Pixels = px(6.0);
pub(crate) const NODE_WIDTH: Pixels = px(200.0);
pub(crate) const HEADER_HEIGHT: Pixels = px(24.0);
pub(crate) const SOCKET_HEIGHT: Pixels = px(12.0);
pub(crate) const SOCKET_GAP: Pixels = px(12.0);

// pub struct NodeView<D, V, N>
// where
//     D: VisualDataType<Value = V>,
//     N: GraphNodeKind<DataType = D, Value = V> + VisualNode,
// {
//     graph: Model<Graph<D, V, N>>,
//     node: Node<D, V, N>,
//     position: Point<Pixels>,
// }

// impl<D, V, N> NodeView<D, V, N>
// where
//     D: VisualDataType<Value = V>,
//     N: GraphNodeKind<DataType = D, Value = V> + VisualNode,
// {
//     fn render_socket(&self, data_type: &D, left: bool) -> impl IntoElement {
//         div()
//             .w_1()
//             .h(SOCKET_HEIGHT)
//             .bg(data_type.color())
//             .rounded_r_sm()
//             .when(left, |e| e.rounded_r_none().rounded_l_sm())
//     }

//     fn node_on_drag_move(
//         &mut self,
//         event: &DragMoveEvent<DraggedNode>,
//         cx: &mut ViewContext<Self>,
//     ) {
//         let dragged_node = event.drag(cx);
//         let new_position = dragged_node.start_node_position
//             + (cx.mouse_position() - dragged_node.start_mouse_position);

//         self.position = new_position.into();
//     }
// }

// impl<D, V, N> Render for NodeView<D, V, N>
// where
//     D: VisualDataType<Value = V> + 'static,
//     V: 'static,
//     N: GraphNodeKind<DataType = D, Value = V> + VisualNode + 'static,
// {
//     fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
//         let header = {
//             let label = self.node.kind.label().to_owned();

//             div()
//                 .h_flex()
//                 .gap_1()
//                 .px_1()
//                 .py_px()
//                 .border_b_1()
//                 .border_color(rgb(0x404040))
//                 .child(label)
//         };

//         let content = {
//             let inputs = self
//                 .node
//                 .inputs
//                 .clone()
//                 .into_iter()
//                 .map(|(label, input_id)| {
//                     let input = self.graph.read(cx).input(input_id);

//                     div()
//                         .h_flex()
//                         .h(SOCKET_HEIGHT)
//                         .gap_2()
//                         .child(self.render_socket(&input.data_type, false))
//                         .child(label.clone())
//                 })
//                 .collect::<Vec<_>>();

//             let outputs = self
//                 .node
//                 .outputs
//                 .clone()
//                 .into_iter()
//                 .map(|(label, output_id)| {
//                     let output = self.graph.read(cx).output(output_id);

//                     div()
//                         .h_flex()
//                         .h(SOCKET_HEIGHT)
//                         .w_full()
//                         .flex_row_reverse()
//                         .gap_2()
//                         .child(self.render_socket(&output.data_type, true))
//                         .child(label.clone())
//                 });

//             div()
//                 .min_h_10()
//                 .v_flex()
//                 .gap(SOCKET_GAP)
//                 .py(NODE_CONTENT_Y_PADDING)
//                 .children(inputs)
//                 .children(outputs)
//         };

//         div()
//             // FIXME: This way of creating an id feels hacky.
//             .id(ElementId::Name(format!("node-{:?}", self.node.id).into()))
//             .absolute()
//             .left(self.position.x)
//             .top(self.position.y)
//             .w(NODE_WIDTH)
//             .bg(rgb(0x181818))
//             .border_1()
//             .border_color(rgb(0x404040))
//             .rounded_md()
//             .hover(|v| v) // FIXME: This is a hack to prevent a weird movement issue when dragging for some reason?
//             .child(header)
//             .child(content)
//             .on_drag(
//                 DraggedNode {
//                     start_node_position: self.position.into(),
//                     start_mouse_position: cx.mouse_position(),
//                 },
//                 |_, cx| cx.new_view(|_cx| EmptyView),
//             )
//             .on_drag_move(cx.listener(Self::node_on_drag_move))
//     }
// }

pub enum Socket {
    Input(InputId),
    Output(OutputId),
}

// struct DraggedNode {
//     pub start_node_position: Point<Pixels>,
//     pub start_mouse_position: Point<Pixels>,
// }
