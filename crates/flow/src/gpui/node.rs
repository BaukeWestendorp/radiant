use std::collections::HashMap;

use gpui::prelude::FluentBuilder;
use gpui::*;
use ui::StyledExt;

use crate::{DataType, Graph, InputId, Node, NodeId, NodeKind, OutputId, Value, WidgetEvent};

pub(crate) const NODE_CONTENT_Y_PADDING: Pixels = px(6.0);
pub(crate) const NODE_WIDTH: Pixels = px(200.0);
pub(crate) const HEADER_HEIGHT: Pixels = px(24.0);
pub(crate) const SOCKET_HEIGHT: Pixels = px(12.0);
pub(crate) const SOCKET_GAP: Pixels = px(12.0);

pub struct NodeView<D, V, N>
where
    D: DataType<Value = V>,
    N: NodeKind<DataType = D, Value = V>,
{
    graph: Model<Graph<D, V, N>>,
    output_widgets: HashMap<OutputId, AnyView>,
    node_id: NodeId,
}

impl<D, V, N> NodeView<D, V, N>
where
    D: DataType<Value = V> + 'static,
    V: Value + 'static,
    N: NodeKind<DataType = D, Value = V> + 'static,
{
    pub fn build(
        node_id: NodeId,
        graph: Model<Graph<D, V, N>>,
        cx: &mut WindowContext,
    ) -> View<Self> {
        cx.new_view(|cx| {
            let output_widgets = Self::create_output_widget_views(graph, cx);

            Self {
                graph,
                output_widgets,
                node_id,
            }
        })
    }

    fn node<'a>(&'a self, cx: &'a AppContext) -> &Node<D, V, N> {
        self.graph.read(cx).node(self.node_id)
    }

    fn render_socket(&self, data_type: &D, left: bool) -> impl IntoElement {
        div()
            .w_1()
            .h(SOCKET_HEIGHT)
            .bg(data_type.color())
            .rounded_r_sm()
            .when(left, |e| e.rounded_r_none().rounded_l_sm())
    }

    fn node_on_drag_move(
        &mut self,
        event: &DragMoveEvent<DraggedNode>,
        cx: &mut ViewContext<Self>,
    ) {
        let dragged_node = event.drag(cx);

        if self.node_id != dragged_node.node_id {
            return;
        }

        let new_position = dragged_node.start_node_position
            + (cx.mouse_position() - dragged_node.start_mouse_position);

        self.graph.update(cx, |graph, cx| {
            graph.node_mut(self.node_id).position = new_position;
            cx.notify();
        });
    }

    fn create_output_widget_views(
        graph: &Model<Graph<D, V, N>>,
        cx: &mut ViewContext<Self>,
    ) -> HashMap<OutputId, AnyView> {
        let constant_outputs = graph
            .read(cx)
            .outputs
            .clone()
            .iter()
            .filter(|(_id, o)| matches!(o.value, crate::OutputValue::Constant(_)));

        let mut output_widgets = HashMap::new();
        for (id, o) in constant_outputs {
            output_widgets.insert(id, o.data_type.widget(cx));
        }
        output_widgets
    }
}

impl<D, V, N> Render for NodeView<D, V, N>
where
    D: DataType<Value = V> + 'static,
    V: Value + 'static,
    N: NodeKind<DataType = D, Value = V> + 'static,
{
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let node = self.node(cx);

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
            let inputs = node
                .inputs
                .clone()
                .into_iter()
                .map(|(label, input_id)| {
                    let input = self.graph.read(cx).input(input_id);

                    div()
                        .h_flex()
                        .h(SOCKET_HEIGHT)
                        .gap_2()
                        .child(self.render_socket(&input.data_type, false))
                        .child(label.clone())
                })
                .collect::<Vec<_>>();

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

        let position = node.position;
        div()
            // FIXME: This way of creating an id feels hacky.
            .id(ElementId::Name(format!("node-{:?}", self.node_id).into()))
            .absolute()
            .left(position.x)
            .top(position.y)
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
                    node_id: self.node_id,
                    start_node_position: position,
                    start_mouse_position: cx.mouse_position(),
                },
                |_, cx| cx.new_view(|_cx| EmptyView),
            )
            .on_drag_move(cx.listener(Self::node_on_drag_move))
    }
}

impl<D, V, N> EventEmitter<WidgetEvent<V>> for NodeView<D, V, N>
where
    D: DataType<Value = V> + 'static,
    V: Value + 'static,
    N: NodeKind<DataType = D, Value = V> + 'static,
{
}

pub enum Socket {
    Input(InputId),
    Output(OutputId),
}

struct DraggedNode {
    pub node_id: NodeId,

    pub start_node_position: Point<Pixels>,
    pub start_mouse_position: Point<Pixels>,
}
