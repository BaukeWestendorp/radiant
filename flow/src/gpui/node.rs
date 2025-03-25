use crate::{
    AnySocket, Control, DataType, GraphDef, Input, InputSocket, NodeId, Output, OutputSocket,
};

use super::{ControlEvent, ControlView, graph::GraphView};
use gpui::*;
use prelude::FluentBuilder;
use ui::{styled_ext::StyledExt, theme::ActiveTheme};

pub struct NodeMeasurements {
    pub content_padding_y: Pixels,
    pub width: Pixels,
    pub min_height: Pixels,
    pub header_height: Pixels,
    pub socket_height: Pixels,
    pub socket_gap: Pixels,
    pub snap_size: Pixels,
    pub connector_width: Pixels,
    pub connector_height: Pixels,
}

impl NodeMeasurements {
    pub fn new(window: &Window) -> Self {
        let rem = window.rem_size();
        let line_height = window.line_height();

        Self {
            content_padding_y: rem,
            width: rem * 16.0,
            min_height: rem * 8.0,
            header_height: line_height,
            socket_height: line_height,
            socket_gap: rem / 2.0,
            snap_size: rem,
            connector_width: rem / 4.0,
            connector_height: rem,
        }
    }
}

pub struct NodeView<D: GraphDef + 'static> {
    node_id: NodeId,

    graph_view: Entity<GraphView<D>>,

    inputs: Vec<Entity<InputView<D>>>,
    outputs: Vec<Entity<OutputView<D>>>,
    controls: Vec<Entity<ControlView>>,

    focus_handle: FocusHandle,
}

impl<D: GraphDef + 'static> NodeView<D> {
    pub fn build(
        node_id: NodeId,
        graph_view: Entity<GraphView<D>>,
        graph: Entity<crate::Graph<D>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<Self> {
        cx.new(move |cx| {
            let node = graph.read(cx).node(&node_id);
            let template = graph.read(cx).template(node.template_id()).clone();

            let inputs = template
                .inputs()
                .iter()
                .cloned()
                .map(|input| {
                    let socket = InputSocket::new(node_id, input.id().to_owned());
                    let value = graph.read(cx).input_value(&socket).unwrap();
                    InputView::build(input, node_id, value.clone(), graph_view.clone(), window, cx)
                })
                .collect();

            let outputs = template
                .outputs()
                .iter()
                .cloned()
                .map(|output| OutputView::build(output, node_id, graph_view.clone(), cx))
                .collect();

            let controls = template
                .controls()
                .iter()
                .cloned()
                .map(|control| {
                    let value = graph.read(cx).control_value(&node_id, control.id()).unwrap();
                    let id = ElementId::Name(
                        format!("node-control-{:?}-{}", node_id, control.id()).into(),
                    );

                    let control_view = control.control().build_view(value.clone(), id, window, cx);

                    cx.subscribe(&control_view, {
                        let graph = graph.clone();
                        move |_control_view: &mut Self, _, event: &ControlEvent<D>, cx| match event
                        {
                            ControlEvent::Change(value) => {
                                let value = value.clone();
                                graph.update(cx, |graph, cx| {
                                    graph.set_control_value(
                                        &node_id,
                                        control.id().to_owned(),
                                        value,
                                    );
                                    cx.notify();
                                })
                            }
                        }
                    })
                    .detach();

                    control_view
                })
                .collect();

            Self { node_id, graph_view, inputs, outputs, controls, focus_handle: cx.focus_handle() }
        })
    }
}

impl<D: GraphDef + 'static> Render for NodeView<D> {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let graph = self.graph_view.read(cx).graph().read(cx);
        let template_id = graph.node(&self.node_id).template_id().clone();
        let template = graph.template(&template_id);

        let focused = self.focus_handle.is_focused(window);

        let NodeMeasurements {
            content_padding_y,
            width,
            min_height,
            header_height,
            socket_gap,
            ..
        } = NodeMeasurements::new(window);

        let header = {
            let label = template.label().to_string();

            div()
                .h_flex()
                .h(header_height)
                .gap_1()
                .px_1()
                .py_px()
                .border_b_1()
                .border_color(cx.theme().border)
                .when(focused, |e| {
                    e.bg(cx.theme().element_background_focused)
                        .border_color(cx.theme().border_focused)
                })
                .child(label)
        };

        let content = div()
            .v_flex()
            .gap(socket_gap)
            .py(content_padding_y)
            .children(self.inputs.clone())
            .children(self.outputs.clone())
            .children(self.controls.iter().map(|c| c.read(cx).view.clone()));

        div()
            .track_focus(&self.focus_handle)
            .w(width)
            .min_h(min_height)
            .bg(cx.theme().background)
            .border_1()
            .border_color(cx.theme().border)
            .when(focused, |e| e.border_color(cx.theme().border_focused))
            .rounded(cx.theme().radius)
            .cursor_grab()
            .children([header, content])
    }
}

impl<D: GraphDef + 'static> Focusable for NodeView<D> {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

struct InputView<D: GraphDef + 'static> {
    input: Input<D>,
    node_id: NodeId,
    graph_view: Entity<GraphView<D>>,

    id: ElementId,
    connector: Entity<ConnectorView<D>>,
    control: Entity<ControlView>,
}

impl<D: GraphDef + 'static> InputView<D> {
    pub fn build(
        input: Input<D>,
        node_id: NodeId,
        value: D::Value,
        graph_view: Entity<GraphView<D>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<Self> {
        cx.new(|cx| {
            let socket = InputSocket::new(node_id.clone(), input.id().to_string());
            let data_type = input.data_type().clone();
            let id = ElementId::Name(format!("input-{}-{}", node_id.0, input.id()).into());
            let control = input.control().build_view(value, id.clone(), window, cx);

            cx.subscribe(&control, {
                let socket = socket.clone();
                move |input_view: &mut Self, _, event: &ControlEvent<D>, cx| match event {
                    ControlEvent::Change(value) => {
                        let value = value.clone();
                        let socket = socket.clone();
                        input_view.graph_view.update(cx, move |graph_view, cx| {
                            graph_view.graph().update(cx, |graph, cx| {
                                graph.set_input_value(socket, value);
                                cx.notify();
                            })
                        });
                    }
                }
            })
            .detach();

            Self {
                input,
                node_id,
                graph_view: graph_view.clone(),
                id,
                connector: ConnectorView::build(
                    AnySocket::Input(socket),
                    data_type,
                    graph_view,
                    cx,
                ),
                control,
            }
        })
    }
}

impl<D: GraphDef + 'static> Render for InputView<D> {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let label = self.input.label().to_string();

        let graph = self.graph_view.read(cx).graph().read(cx);
        let socket = InputSocket::new(self.node_id, self.input.id().to_string());
        let has_connection = graph.edge_source(&socket).is_some();

        let NodeMeasurements { socket_height, .. } = NodeMeasurements::new(window);

        div()
            .id(self.id.clone())
            .h_flex()
            .pr_1()
            .h(socket_height)
            .gap_2()
            .child(self.connector.clone())
            .child(label)
            .when(!has_connection, |e| e.child(self.control.read(cx).view.clone()))
    }
}

struct OutputView<D: GraphDef + 'static> {
    output: Output<D>,

    id: ElementId,
    connector: Entity<ConnectorView<D>>,
}

impl<D: GraphDef + 'static> OutputView<D> {
    pub fn build(
        output: Output<D>,
        node_id: NodeId,
        graph_view: Entity<GraphView<D>>,
        cx: &mut App,
    ) -> Entity<Self> {
        cx.new(|cx| {
            let socket = OutputSocket::new(node_id, output.id().to_string());
            let data_type = output.data_type().clone();
            let id = ElementId::Name(format!("output-{}-{}", node_id.0, output.id()).into());
            Self {
                output,
                id,
                connector: ConnectorView::build(
                    AnySocket::Output(socket),
                    data_type,
                    graph_view,
                    cx,
                ),
            }
        })
    }
}

impl<D: GraphDef + 'static> Render for OutputView<D> {
    fn render(&mut self, window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let label = self.output.label().to_string();

        let NodeMeasurements { socket_height, .. } = NodeMeasurements::new(window);

        div()
            .id(self.id.clone())
            .pl_1()
            .h_flex()
            .h(socket_height)
            .w_full()
            .flex_row_reverse()
            .gap_2()
            .child(self.connector.clone())
            .child(label)
    }
}

struct ConnectorView<D: GraphDef + 'static> {
    socket: AnySocket,
    data_type: D::DataType,
    hovering: bool,

    graph_view: Entity<GraphView<D>>,
}

impl<D: GraphDef + 'static> ConnectorView<D> {
    const HITBOX_SIZE: Pixels = px(22.0);

    pub fn build(
        socket: AnySocket,
        data_type: D::DataType,
        graph_view: Entity<GraphView<D>>,
        cx: &mut App,
    ) -> Entity<Self> {
        cx.new(|_cx| Self { socket, data_type, hovering: false, graph_view })
    }
}

impl<D: GraphDef> ConnectorView<D> {
    fn handle_drag_move(
        &mut self,
        event: &DragMoveEvent<AnySocket>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if &self.socket != event.drag(cx) {
            return;
        }

        self.graph_view.update(cx, |graph_view, cx| {
            graph_view.drag_new_edge(&self.socket, Self::HITBOX_SIZE.0 / 2.0, window, cx);
        })
    }

    fn handle_mouse_down(
        &mut self,
        _event: &MouseDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.graph_view
            .update(cx, |graph_view, cx| graph_view.set_new_edge_socket(&self.socket, cx))
    }
}

impl<D: GraphDef + 'static> Render for ConnectorView<D> {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let NodeMeasurements { connector_width, connector_height, .. } =
            NodeMeasurements::new(window);

        let (node_id, socket_name) = match self.socket.clone() {
            AnySocket::Input(socket) => (socket.node_id, socket.id),
            AnySocket::Output(socket) => (socket.node_id, socket.id),
        };

        let id = ElementId::Name(format!("connector-{}-{}", node_id.0, socket_name).into());

        let hitbox = div()
            .id(id)
            .size(Self::HITBOX_SIZE)
            .ml(connector_width / 2.0 - Self::HITBOX_SIZE / 2.0)
            .mt(connector_height / 2.0 - Self::HITBOX_SIZE / 2.0)
            .cursor_crosshair()
            .on_hover(cx.listener(|this, hovering, _, _| this.hovering = *hovering))
            .on_drag(self.socket.clone(), |_, _, _, cx| cx.new(|_| EmptyView))
            .on_drag_move(cx.listener(Self::handle_drag_move))
            .on_mouse_down(MouseButton::Left, cx.listener(Self::handle_mouse_down));

        let left_side = match self.socket {
            AnySocket::Input(_) => false,
            AnySocket::Output(_) => true,
        };

        div()
            .w(connector_width)
            .h(connector_height)
            .bg(self.data_type.color())
            .rounded_r(cx.theme().radius)
            .border_1()
            .border_color(black().opacity(0.3))
            .when(left_side, |e| e.rounded_r_none().rounded_l(cx.theme().radius))
            .when(self.hovering, |e| e.bg(white()))
            .child(hitbox)
    }
}
