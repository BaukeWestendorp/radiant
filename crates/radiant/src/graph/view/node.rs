use gpui::prelude::FluentBuilder;
use gpui::*;

use ui::StyledExt;

use crate::graph::{
    node::{Input, Node, Output, OutputValue},
    DataType, Graph, InputId, NodeId, OutputId, Value,
};

pub(crate) const NODE_CONTENT_Y_PADDING: Pixels = px(6.0);
pub(crate) const NODE_WIDTH: Pixels = px(200.0);
pub(crate) const HEADER_HEIGHT: Pixels = px(24.0);
pub(crate) const SOCKET_HEIGHT: Pixels = px(12.0);
pub(crate) const SOCKET_GAP: Pixels = px(12.0);

pub struct NodeView {
    node_id: NodeId,
    graph: Model<Graph>,
    inputs: Vec<View<InputView>>,
    outputs: Vec<View<OutputView>>,
}

impl NodeView {
    pub fn build(node_id: NodeId, graph: Model<Graph>, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| Self {
            node_id,
            inputs: Self::build_inputs(node_id, &graph, cx),
            outputs: Self::build_outputs(node_id, &graph, cx),
            graph,
        })
    }

    fn node<'cx>(&'cx self, cx: &'cx AppContext) -> &Node {
        self.graph.read(cx).node(self.node_id)
    }

    fn build_inputs(
        node_id: NodeId,
        graph: &Model<Graph>,
        cx: &mut WindowContext,
    ) -> Vec<View<InputView>> {
        let inputs = {
            let graph = graph.read(cx);
            graph.node(node_id).inputs.clone()
        };

        inputs
            .into_iter()
            .map({
                let graph = graph.clone();
                move |(label, input_id)| {
                    let input = graph.read(cx).input(input_id).clone();
                    InputView::build(input, label, cx)
                }
            })
            .collect()
    }

    fn build_outputs(
        node_id: NodeId,
        graph: &Model<Graph>,
        cx: &mut WindowContext,
    ) -> Vec<View<OutputView>> {
        let outputs = {
            let graph = graph.read(cx);
            graph.node(node_id).outputs.clone()
        };

        outputs
            .into_iter()
            .map({
                let graph = graph.clone();
                move |(label, output_id)| {
                    let output = graph.read(cx).output(output_id).clone();

                    let output_view = OutputView::build(output.clone(), label, cx);
                    cx.subscribe(&output_view, {
                        let graph = graph.clone();
                        move |view, event, cx| {
                            let output_id = view.read(cx).output.id;
                            let ControlEvent::ChangeValue(value) = event;
                            graph.update(cx, move |graph, cx| {
                                graph.output_mut(output_id).value =
                                    OutputValue::Constant(value.clone());
                                cx.notify();
                            });
                        }
                    })
                    .detach();
                    output_view
                }
            })
            .collect()
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
}

impl Render for NodeView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let node = self.node(cx);

        let header = {
            let label = node.kind.label().to_owned();

            div()
                .h_flex()
                .h(HEADER_HEIGHT)
                .gap_1()
                .px_1()
                .py_px()
                .border_b_1()
                .border_color(rgb(0x404040))
                .child(label)
        };

        let content = {
            div()
                .min_h_10()
                .v_flex()
                .gap(SOCKET_GAP)
                .py(NODE_CONTENT_Y_PADDING)
                .children(self.inputs.clone())
                .children(self.outputs.clone())
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

struct DraggedNode {
    pub node_id: NodeId,

    pub start_node_position: Point<Pixels>,
    pub start_mouse_position: Point<Pixels>,
}

pub struct InputView {
    input: Input,
    label: String,
}

impl InputView {
    pub fn build(input: Input, label: String, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| Self { input, label })
    }
}

impl Render for InputView {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .h_flex()
            .h(SOCKET_HEIGHT)
            .gap_2()
            .child(render_socket(&self.input.data_type, false))
            .child(self.label.clone())
    }
}

impl EventEmitter<ControlEvent> for InputView {}

pub struct OutputView {
    output: Output,
    label: String,
    control_view: AnyView,
}

impl OutputView {
    pub fn build(output: Output, label: String, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let control_view = output.data_type.control(cx);

            Self {
                output,
                label,
                control_view,
            }
        })
    }
}

impl Render for OutputView {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .h_flex()
            .h(SOCKET_HEIGHT)
            .w_full()
            .flex_row_reverse()
            .gap_2()
            .child(render_socket(&self.output.data_type, true))
            .child(self.label.clone())
            .child(self.control_view.clone())
    }
}

fn render_socket(data_type: &DataType, left: bool) -> impl IntoElement {
    div()
        .w_1()
        .h(SOCKET_HEIGHT)
        .bg(data_type.color())
        .rounded_r_sm()
        .when(left, |e| e.rounded_r_none().rounded_l_sm())
}

impl EventEmitter<ControlEvent> for OutputView {}

#[derive(Debug, Clone)]
pub enum ControlEvent {
    ChangeValue(Value),
}

pub enum Socket {
    Input(InputId),
    Output(OutputId),
}
