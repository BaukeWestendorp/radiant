use flow::{GraphDef, Input, NodeId, Output, Socket, SocketKind};
use gpui::*;
use prelude::FluentBuilder;
use ui::{styled_ext::StyledExt, theme::ActiveTheme};

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

            Self { node_id, graph, inputs, outputs }
        })
    }
}

impl<D: GraphDef + 'static> Render for NodeView<D> {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let graph = self.graph.read(cx);
        let template_id = graph.node(&self.node_id).template_id().clone();

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
                .child(label)
        };

        let content = div()
            .min_h_10()
            .v_flex()
            .gap(SOCKET_GAP)
            .py(NODE_CONTENT_Y_PADDING)
            .children(self.inputs.clone())
            .children(self.outputs.clone());

        div()
            .w(NODE_WIDTH)
            .min_h(SNAP_GRID_SIZE * 8)
            .bg(cx.theme().background)
            .border_1()
            .border_color(cx.theme().border_color)
            .rounded(cx.theme().radius)
            .children([header, content])
    }
}

struct InputView<D: GraphDef> {
    input: Input<D>,
    node_id: NodeId,

    graph: Entity<crate::Graph<D>>,
}

impl<D: GraphDef + 'static> InputView<D> {
    pub fn build(
        input: Input<D>,
        node_id: NodeId,
        graph: Entity<crate::Graph<D>>,
        cx: &mut App,
    ) -> Entity<Self> {
        cx.new(|_cx| Self { input, node_id, graph })
    }
}

impl<D: GraphDef + 'static> Render for InputView<D> {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let socket = Socket::new(self.node_id, self.input.id().to_string());
        let socket_kind = SocketKind::Input(socket);
        let connector = render_connector(&socket_kind, &self.graph, cx);
        let label = self.input.label().to_string();

        div().h_flex().pr_1().h(SOCKET_HEIGHT).gap_2().child(connector).child(label)
    }
}

struct OutputView<D: GraphDef> {
    output: Output,
    node_id: NodeId,

    graph: Entity<crate::Graph<D>>,
}

impl<D: GraphDef + 'static> OutputView<D> {
    pub fn build(
        output: Output,
        node_id: NodeId,
        graph: Entity<crate::Graph<D>>,
        cx: &mut App,
    ) -> Entity<Self> {
        cx.new(|_cx| Self { output, node_id, graph })
    }
}

impl<D: GraphDef + 'static> Render for OutputView<D> {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let socket = Socket::new(self.node_id, self.output.id().to_string());
        let socket_kind = SocketKind::Output(socket);
        let connector = render_connector(&socket_kind, &self.graph, cx);
        let label = self.output.label().to_string();

        div()
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

fn render_connector<D: GraphDef>(
    socket: &SocketKind,
    graph: &Entity<crate::Graph<D>>,
    cx: &App,
) -> impl IntoElement {
    let width = px(3.0);
    let height = px(13.0);
    let hover_box_size = px(35.0);

    let left = match socket {
        SocketKind::Input(_) => false,
        SocketKind::Output(_) => true,
    };

    let color = red();

    div()
        .w(width)
        .h(height)
        .bg(color)
        .rounded_r(cx.theme().radius)
        .when(left, |e| e.rounded_r_none().rounded_l(cx.theme().radius))
        .child(
            div()
                .size(hover_box_size)
                .ml(width / 2.0 - hover_box_size / 2.0)
                .mt(height / 2.0 - hover_box_size / 2.0)
                .cursor_crosshair(),
        )
}
