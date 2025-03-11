use flow::{AnySocket, GraphDef, Input, NodeId, Output, Socket};
use gpui::*;
use prelude::FluentBuilder;
use ui::{styled_ext::StyledExt, theme::ActiveTheme};

use crate::DataType;

pub(crate) const NODE_CONTENT_Y_PADDING: Pixels = px(6.0);
pub(crate) const NODE_WIDTH: Pixels = px(204.0);
pub(crate) const HEADER_HEIGHT: Pixels = px(24.0);
pub(crate) const SOCKET_HEIGHT: Pixels = px(24.0); // cx.theme().input_height;
pub(crate) const SOCKET_GAP: Pixels = px(12.0);
pub(crate) const SNAP_GRID_SIZE: Pixels = px(12.0);

pub struct NodeView<D: GraphDef> {
    node_id: NodeId,

    graph: Entity<crate::Graph<D>>,

    inputs: Vec<Entity<InputView<D>>>,
    outputs: Vec<Entity<OutputView<D>>>,

    focus_handle: FocusHandle,
}

impl<D: GraphDef + 'static> NodeView<D> {
    pub fn build(node_id: NodeId, graph: Entity<crate::Graph<D>>, cx: &mut App) -> Entity<Self> {
        cx.new(move |cx| {
            let node = graph.read(cx).node(&node_id);
            let template = graph.read(cx).template(node.template_id()).clone();

            let inputs = template
                .inputs()
                .iter()
                .cloned()
                .map(|input| InputView::build(input, node_id, graph.clone(), cx))
                .collect();

            let outputs = template
                .outputs()
                .iter()
                .cloned()
                .map(|output| OutputView::build(output, node_id, graph.clone(), cx))
                .collect();

            Self { node_id, graph, inputs, outputs, focus_handle: cx.focus_handle() }
        })
    }
}

impl<D: GraphDef + 'static> Render for NodeView<D>
where
    D::DataType: crate::DataType<D>,
{
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let graph = self.graph.read(cx);
        let template_id = graph.node(&self.node_id).template_id().clone();

        let focused = self.focus_handle.is_focused(window);

        let header = {
            let label = graph.template(&template_id).label().to_string();

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
            .children([header, content])
    }
}

impl<D: GraphDef + 'static> Focusable for NodeView<D> {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

struct InputView<D: GraphDef> {
    input: Input<D>,
    node_id: NodeId,

    hovering: bool,

    graph: Entity<crate::Graph<D>>,
}

impl<D: GraphDef + 'static> InputView<D> {
    pub fn build(
        input: Input<D>,
        node_id: NodeId,
        graph: Entity<crate::Graph<D>>,
        cx: &mut App,
    ) -> Entity<Self> {
        cx.new(|_cx| Self { input, node_id, hovering: false, graph })
    }
}

impl<D: GraphDef + 'static> Render for InputView<D>
where
    D::DataType: crate::DataType<D>,
{
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let socket = Socket::new(self.node_id, self.input.id().to_string());
        let any_socket = AnySocket::Input(socket);
        let connector = render_connector(&any_socket, self.hovering, &self.graph, cx);
        let label = self.input.label().to_string();

        let id = ElementId::Name(format!("input-{}-{}", self.node_id.0, self.input.id()).into());

        div()
            .id(id)
            .on_drag_move::<()>(|_, _, _| {}) // FIXME: For some reason this on_drag_move is required to make on_hover work...
            .on_hover(cx.listener(|this, hovering, _, cx| {
                this.hovering = *hovering;
                cx.notify();
            }))
            .h_flex()
            .pr_1()
            .h(SOCKET_HEIGHT)
            .gap_2()
            .child(connector)
            .child(label)
    }
}

struct OutputView<D: GraphDef> {
    output: Output<D>,
    node_id: NodeId,

    hovering: bool,

    graph: Entity<crate::Graph<D>>,
}

impl<D: GraphDef + 'static> OutputView<D> {
    pub fn build(
        output: Output<D>,
        node_id: NodeId,
        graph: Entity<crate::Graph<D>>,
        cx: &mut App,
    ) -> Entity<Self> {
        cx.new(|_cx| Self { output, node_id, hovering: false, graph })
    }
}

impl<D: GraphDef + 'static> Render for OutputView<D>
where
    D::DataType: crate::DataType<D>,
{
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let socket = Socket::new(self.node_id, self.output.id().to_string());
        let any_socket = AnySocket::Output(socket);
        let connector = render_connector(&any_socket, self.hovering, &self.graph, cx);
        let label = self.output.label().to_string();

        let id = ElementId::Name(format!("output-{}-{}", self.node_id.0, self.output.id()).into());

        div()
            .id(id)
            .on_drag_move::<()>(|_, _, _| {}) // FIXME: For some reason this on_drag_move is required to make on_hover work...
            .on_hover(cx.listener(|this, hovering, _, cx| {
                this.hovering = *hovering;
                cx.notify();
            }))
            .pl_1()
            .h_flex()
            .h(SOCKET_HEIGHT)
            .w_full()
            .flex_row_reverse()
            .gap_2()
            .child(connector)
            .child(label)
    }
}

fn render_connector<D: GraphDef + 'static>(
    any_socket: &AnySocket,
    hovering: bool,
    graph: &Entity<crate::Graph<D>>,
    cx: &App,
) -> impl IntoElement
where
    D::DataType: crate::DataType<D>,
{
    let width = px(5.0);
    let height = px(13.0);
    let hover_box_size = px(22.0);

    let left = match any_socket {
        AnySocket::Input(_) => false,
        AnySocket::Output(_) => true,
    };

    let socket = any_socket.socket();
    let template_id = graph.read(cx).node(&socket.node_id).template_id();
    let template = graph.read(cx).template(template_id);
    let color = match any_socket {
        AnySocket::Input(_) => template.input(&socket.id).data_type().color(),
        AnySocket::Output(_) => template.output(&socket.id).data_type().color(),
    };

    div()
        .w(width)
        .h(height)
        .bg(color)
        .rounded_r(cx.theme().radius)
        .border_1()
        .border_color(black().opacity(0.3))
        .when(left, |e| e.rounded_r_none().rounded_l(cx.theme().radius))
        .when(hovering, |e| e.bg(white()))
        .child(
            div()
                .size(hover_box_size)
                .ml(width / 2.0 - hover_box_size / 2.0)
                .mt(height / 2.0 - hover_box_size / 2.0)
                .cursor_crosshair(),
        )
}
