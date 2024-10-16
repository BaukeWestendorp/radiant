use gpui::prelude::FluentBuilder;
use gpui::*;

use crate::node::{Input, InputValue, Node, Output, OutputValue, Socket};
use crate::{Graph, NodeId, Value};
use ui::{theme::ActiveTheme, StyledExt};

pub(crate) const NODE_CONTENT_Y_PADDING: Pixels = px(6.0);
pub(crate) const NODE_WIDTH: Pixels = px(200.0);
pub(crate) const HEADER_HEIGHT: Pixels = px(24.0);
pub(crate) const SOCKET_HEIGHT: Pixels = px(22.0); // cx.theme().input_height;
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
        cx: &mut ViewContext<Self>,
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
                    let input_view = InputView::build(input, label, graph.clone(), cx);

                    cx.subscribe(&input_view, {
                        let graph = graph.clone();
                        move |_this, input_view, event, cx| {
                            let input_id = input_view.read(cx).input.id;
                            let ControlEvent::Change(new_value) = event;
                            graph.update(cx, move |graph, cx| {
                                let InputValue::Constant { value, .. } =
                                    &mut graph.input_mut(input_id).value;
                                *value = new_value.clone();

                                cx.notify();
                            });
                        }
                    })
                    .detach();

                    // Propagate events from the input view to the graph
                    cx.subscribe(&input_view, |_this, _, event: &SocketEvent, cx| {
                        cx.emit(event.clone())
                    })
                    .detach();

                    input_view
                }
            })
            .collect()
    }

    fn build_outputs(
        node_id: NodeId,
        graph: &Model<Graph>,
        cx: &mut ViewContext<Self>,
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

                    let output_view = OutputView::build(output.clone(), label, graph.clone(), cx);

                    cx.subscribe(&output_view, {
                        let graph = graph.clone();
                        move |_this, output_view, event, cx| {
                            let output_id = output_view.read(cx).output.id;
                            let ControlEvent::Change(new_value) = event;
                            graph.update(cx, move |graph, cx| {
                                if let OutputValue::Constant { value, .. } =
                                    &mut graph.output_mut(output_id).value
                                {
                                    *value = new_value.clone();
                                }

                                cx.notify();
                            });
                        }
                    })
                    .detach();

                    // Propagate events from the input view to the graph
                    cx.subscribe(&output_view, |_this, _, event: &SocketEvent, cx| {
                        cx.emit(event.clone())
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
                .border_color(cx.theme().border)
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
            .hover(|e| e) // FIXME: This is a hack to make the node a little bit less spacy when dragging for some reason...
            .absolute()
            .left(position.x)
            .top(position.y)
            .w(NODE_WIDTH)
            .bg(cx.theme().secondary)
            .border_1()
            .border_color(cx.theme().border)
            .rounded(cx.theme().radius)
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

impl EventEmitter<SocketEvent> for NodeView {}

struct DraggedNode {
    pub node_id: NodeId,

    pub start_node_position: Point<Pixels>,
    pub start_mouse_position: Point<Pixels>,
}

#[derive(Debug, Clone)]
pub enum ControlEvent {
    Change(Value),
}

#[derive(Debug, Clone)]
pub enum SocketEvent {
    StartNewConnection(Socket),
    EndNewConnection,
}

pub struct InputView {
    input: Input,
    label: String,
    graph: Model<Graph>,
    control_view: AnyView,
    hovering: bool,
}

impl InputView {
    pub fn build(
        input: Input,
        label: String,
        graph: Model<Graph>,
        cx: &mut WindowContext,
    ) -> View<Self> {
        cx.new_view(|cx| {
            let control_id: SharedString = format!("input-{:?}", input.id).into();
            let InputValue::Constant { value, control } = input.value.clone();
            let control_view = control.view(ElementId::Name(control_id), value, cx);
            Self {
                input,
                label,
                graph,
                control_view,
                hovering: false,
            }
        })
    }

    fn has_connection(&self, cx: &AppContext) -> bool {
        self.graph
            .read(cx)
            .connection_source(self.input.id)
            .is_some()
    }
}

impl Render for InputView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .h_flex()
            .pr_1()
            .h(SOCKET_HEIGHT)
            .gap_2()
            .child(render_connector(
                &Socket::Input(self.input.id),
                &self.graph,
                self.hovering,
                cx,
            ))
            .child(self.label.clone())
            .when(!self.has_connection(cx), |e| {
                e.child(self.control_view.clone())
            })
    }
}

impl EventEmitter<ControlEvent> for InputView {}

impl EventEmitter<SocketEvent> for InputView {}

impl Hovering for InputView {
    fn set_hovering(&mut self, hovering: bool) {
        self.hovering = hovering;
    }
}

pub struct OutputView {
    output: Output,
    label: String,
    control_view: Option<AnyView>,
    graph: Model<Graph>,
    hovering: bool,
}

impl OutputView {
    pub fn build(
        output: Output,
        label: String,
        graph: Model<Graph>,
        cx: &mut WindowContext,
    ) -> View<Self> {
        cx.new_view(|cx| {
            let control_id: SharedString = format!("output-{:?}", output.id).into();
            let control_view = match &output.value {
                OutputValue::Computed => None,
                OutputValue::Constant { value, control } => {
                    Some(control.view(ElementId::Name(control_id), value.clone(), cx))
                }
            };

            Self {
                output,
                label,
                control_view,
                graph,
                hovering: false,
            }
        })
    }
}

impl Render for OutputView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .pl_1()
            .h_flex()
            .h(SOCKET_HEIGHT)
            .w_full()
            .flex_row_reverse()
            .gap_2()
            .child(render_connector(
                &Socket::Output(self.output.id),
                &self.graph,
                self.hovering,
                cx,
            ))
            .child(self.label.clone())
            .children(self.control_view.clone())
    }
}

impl EventEmitter<SocketEvent> for OutputView {}

impl EventEmitter<ControlEvent> for OutputView {}

impl Hovering for OutputView {
    fn set_hovering(&mut self, hovering: bool) {
        self.hovering = hovering;
    }
}

fn render_connector<V: EventEmitter<SocketEvent> + Hovering>(
    socket: &Socket,
    graph: &Model<Graph>,
    hovering: bool,
    cx: &ViewContext<V>,
) -> impl IntoElement {
    let width = px(3.0);
    let height = px(13.0);
    let hover_box_size = px(35.0);

    let left = match socket {
        Socket::Input(_) => false,
        Socket::Output(_) => true,
    };

    let data_type = match socket {
        Socket::Input(input_id) => &graph.read(cx).input(*input_id).data_type,
        Socket::Output(output_id) => &graph.read(cx).output(*output_id).data_type,
    };

    div()
        .w(width)
        .h(height)
        .bg(data_type.color())
        .when(hovering, |e| e.bg(white()))
        .rounded_r(cx.theme().radius)
        .when(left, |e| e.rounded_r_none().rounded_l(cx.theme().radius))
        .child(
            div()
                .id(ElementId::Name(format!("socket-{:?}", socket).into()))
                .size(hover_box_size)
                .ml(width / 2.0 - hover_box_size / 2.0)
                .mt(height / 2.0 - hover_box_size / 2.0)
                .cursor_grab()
                .on_hover(cx.listener(|this, hovering, cx| {
                    this.set_hovering(*hovering);
                    cx.notify();
                }))
                .on_drag(
                    SocketDrag {
                        socket: socket.clone(),
                    },
                    |_, cx| cx.new_view(|_cx| EmptyView),
                )
                .on_drag_move(cx.listener({
                    let socket = *socket;
                    move |_view, event: &DragMoveEvent<SocketDrag>, cx| {
                        let drag = event.drag(cx);
                        if drag.socket != socket {
                            return;
                        }
                        cx.emit(SocketEvent::StartNewConnection(socket.clone()));
                    }
                }))
                // FIXME: Is there a way to do this in a single listener?
                .on_mouse_up(
                    MouseButton::Left,
                    cx.listener(|_, _, cx| cx.emit(SocketEvent::EndNewConnection)),
                )
                .on_mouse_up_out(
                    MouseButton::Left,
                    cx.listener(|_, _, cx| cx.emit(SocketEvent::EndNewConnection)),
                ),
        )
}

struct SocketDrag {
    socket: Socket,
}

trait Hovering {
    fn set_hovering(&mut self, hovering: bool);
}
