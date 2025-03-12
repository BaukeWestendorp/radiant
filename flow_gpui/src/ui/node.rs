use super::graph::GraphView;
use crate::DataType;
use flow::{AnySocket, GraphDef, Input, NodeId, Output, Socket};
use gpui::*;
use prelude::FluentBuilder;
use ui::{styled_ext::StyledExt, theme::ActiveTheme};

pub(crate) const NODE_CONTENT_Y_PADDING: Pixels = px(6.0);
pub(crate) const NODE_WIDTH: Pixels = px(204.0);
pub(crate) const HEADER_HEIGHT: Pixels = px(24.0);
pub(crate) const SOCKET_HEIGHT: Pixels = px(24.0); // cx.theme().input_height;
pub(crate) const SOCKET_GAP: Pixels = px(12.0);
pub(crate) const SNAP_GRID_SIZE: Pixels = px(12.0);

pub struct NodeView<D: GraphDef + 'static>
where
    D::DataType: crate::DataType<D>,
{
    node_id: NodeId,

    graph_view: Entity<GraphView<D>>,

    inputs: Vec<Entity<InputView<D>>>,
    outputs: Vec<Entity<OutputView<D>>>,

    focus_handle: FocusHandle,
}

impl<D: GraphDef + 'static> NodeView<D>
where
    D::DataType: crate::DataType<D>,
{
    pub fn build(
        node_id: NodeId,
        graph_view: Entity<GraphView<D>>,
        graph: Entity<crate::Graph<D>>,
        cx: &mut App,
    ) -> Entity<Self> {
        cx.new(move |cx| {
            let node = graph.read(cx).node(&node_id);
            let template = graph.read(cx).template(node.template_id()).clone();

            let inputs = template
                .inputs()
                .iter()
                .cloned()
                .map(|input| InputView::build(input, node_id, graph_view.clone(), cx))
                .collect();

            let outputs = template
                .outputs()
                .iter()
                .cloned()
                .map(|output| OutputView::build(output, node_id, graph_view.clone(), cx))
                .collect();

            Self { node_id, graph_view, inputs, outputs, focus_handle: cx.focus_handle() }
        })
    }
}

impl<D: GraphDef + 'static> Render for NodeView<D>
where
    D::DataType: crate::DataType<D>,
{
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let graph = self.graph_view.read(cx).graph().read(cx);
        let template_id = graph.node(&self.node_id).template_id().clone();
        let template = graph.template(&template_id);

        let focused = self.focus_handle.is_focused(window);

        let header = {
            let label = template.label().to_string();

            div()
                .h_flex()
                .h(HEADER_HEIGHT)
                .gap_1()
                .px_1()
                .py_px()
                .border_b_1()
                .border_color(cx.theme().border_color)
                .when(focused, |e| {
                    e.bg(cx.theme().background_focused)
                        .border_color(cx.theme().border_color_focused)
                })
                .child(label)
        };

        let content = div()
            .v_flex()
            .gap(SOCKET_GAP)
            .py(NODE_CONTENT_Y_PADDING)
            .children(self.inputs.clone())
            .children(self.outputs.clone());

        div()
            .track_focus(&self.focus_handle)
            .w(NODE_WIDTH)
            .min_h(SNAP_GRID_SIZE * 8)
            .bg(cx.theme().background)
            .border_1()
            .border_color(cx.theme().border_color)
            .when(focused, |e| e.border_color(cx.theme().border_color_focused))
            .rounded(cx.theme().radius)
            .cursor_grab()
            .children([header, content])
    }
}

impl<D: GraphDef + 'static> Focusable for NodeView<D>
where
    D::DataType: crate::DataType<D>,
{
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

struct InputView<D: GraphDef + 'static>
where
    D::DataType: crate::DataType<D>,
{
    input: Input<D>,
    node_id: NodeId,

    connector: Entity<ConnectorView<D>>,
}

impl<D: GraphDef + 'static> InputView<D>
where
    D::DataType: crate::DataType<D>,
{
    pub fn build(
        input: Input<D>,
        node_id: NodeId,
        graph_view: Entity<GraphView<D>>,
        cx: &mut App,
    ) -> Entity<Self> {
        cx.new(|cx| {
            let socket = Socket::new(node_id, input.id().to_string());
            let data_type = input.data_type().clone();
            Self {
                input,
                node_id,
                connector: ConnectorView::build(
                    AnySocket::Input(socket),
                    data_type,
                    graph_view,
                    cx,
                ),
            }
        })
    }
}

impl<D: GraphDef + 'static> Render for InputView<D>
where
    D::DataType: crate::DataType<D>,
{
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let label = self.input.label().to_string();

        let id = ElementId::Name(format!("input-{}-{}", self.node_id.0, self.input.id()).into());

        div()
            .id(id)
            .h_flex()
            .pr_1()
            .h(SOCKET_HEIGHT)
            .gap_2()
            .child(self.connector.clone())
            .child(label)
    }
}

struct OutputView<D: GraphDef + 'static>
where
    D::DataType: crate::DataType<D>,
{
    output: Output<D>,
    node_id: NodeId,

    connector: Entity<ConnectorView<D>>,
}

impl<D: GraphDef + 'static> OutputView<D>
where
    D::DataType: crate::DataType<D>,
{
    pub fn build(
        output: Output<D>,
        node_id: NodeId,
        graph_view: Entity<GraphView<D>>,
        cx: &mut App,
    ) -> Entity<Self> {
        cx.new(|cx| {
            let socket = Socket::new(node_id, output.id().to_string());
            let data_type = output.data_type().clone();
            Self {
                output,
                node_id,
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

impl<D: GraphDef + 'static> Render for OutputView<D>
where
    D::DataType: crate::DataType<D>,
{
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let label = self.output.label().to_string();

        let id = ElementId::Name(format!("output-{}-{}", self.node_id.0, self.output.id()).into());

        div()
            .id(id)
            .pl_1()
            .h_flex()
            .h(SOCKET_HEIGHT)
            .w_full()
            .flex_row_reverse()
            .gap_2()
            .child(self.connector.clone())
            .child(label)
    }
}

struct ConnectorView<D: GraphDef + 'static>
where
    D::DataType: crate::DataType<D>,
{
    any_socket: AnySocket,
    data_type: D::DataType,
    hovering: bool,

    graph_view: Entity<GraphView<D>>,
}

impl<D: GraphDef + 'static> ConnectorView<D>
where
    D::DataType: crate::DataType<D>,
{
    const HITBOX_SIZE: Pixels = px(22.0);

    pub fn build(
        any_socket: AnySocket,
        data_type: D::DataType,
        graph_view: Entity<GraphView<D>>,
        cx: &mut App,
    ) -> Entity<Self> {
        cx.new(|_cx| Self { any_socket, data_type, hovering: false, graph_view })
    }

    fn on_drag_move(
        &mut self,
        event: &DragMoveEvent<AnySocket>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if &self.any_socket != event.drag(cx) {
            return;
        }

        self.graph_view.update(cx, |graph_view, cx| {
            graph_view.drag_new_edge(&self.any_socket, Self::HITBOX_SIZE.0 / 2.0, window, cx);
        })
    }

    fn on_mouse_down(
        &mut self,
        _event: &MouseDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.graph_view
            .update(cx, |graph_view, cx| graph_view.set_new_edge_socket(&self.any_socket, cx))
    }

    fn on_mouse_up(&mut self, _event: &MouseUpEvent, _window: &mut Window, cx: &mut Context<Self>) {
        self.graph_view.update(cx, |graph_view, cx| graph_view.finish_new_edge(cx))
    }
}

impl<D: GraphDef + 'static> Render for ConnectorView<D>
where
    D::DataType: crate::DataType<D>,
{
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let width = px(5.0);
        let height = px(13.0);

        let socket = self.any_socket.socket();
        let id = ElementId::Name(format!("connector-{}-{}", socket.node_id.0, socket.id).into());

        let hitbox = div()
            .id(id)
            .size(Self::HITBOX_SIZE)
            .ml(width / 2.0 - Self::HITBOX_SIZE / 2.0)
            .mt(height / 2.0 - Self::HITBOX_SIZE / 2.0)
            .cursor_crosshair()
            .on_hover(cx.listener(|this, hovering, _, _| this.hovering = *hovering))
            .on_drag(self.any_socket.clone(), |_, _, _, cx| cx.new(|_| EmptyView))
            .on_drag_move(cx.listener(Self::on_drag_move))
            .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::on_mouse_up))
            .on_mouse_up_out(MouseButton::Left, cx.listener(Self::on_mouse_up));

        let left_side = match self.any_socket {
            AnySocket::Input(_) => false,
            AnySocket::Output(_) => true,
        };

        div()
            .w(width)
            .h(height)
            .bg(self.data_type.color())
            .rounded_r(cx.theme().radius)
            .border_1()
            .border_color(black().opacity(0.3))
            .when(left_side, |e| e.rounded_r_none().rounded_l(cx.theme().radius))
            .when(self.hovering, |e| e.bg(white()))
            .child(hitbox)
    }
}
