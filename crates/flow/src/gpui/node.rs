use super::{GraphEvent, VisualControl, VisualDataType, VisualNodeData, VisualNodeKind};
use crate::{
    Graph, GraphDefinition, InputId, InputParameterKind, Node, NodeId, OutputId,
    OutputParameterKind, Parameter,
};

use gpui::prelude::FluentBuilder;
use gpui::*;
use ui::{ActiveTheme, StyledExt};

pub(crate) const NODE_CONTENT_Y_PADDING: Pixels = px(6.0);
pub(crate) const NODE_WIDTH: Pixels = px(204.0);
pub(crate) const HEADER_HEIGHT: Pixels = px(24.0);
pub(crate) const SOCKET_HEIGHT: Pixels = px(24.0); // cx.theme().input_height;
pub(crate) const SOCKET_GAP: Pixels = px(12.0);
pub(crate) const SNAP_GRID_SIZE: f32 = 12.0;

actions!(graph_editor, [RemoveNode]);

const KEY_CONTEXT: &str = "GraphNode";

pub(crate) fn init(cx: &mut AppContext) {
    cx.bind_keys([
        KeyBinding::new("backspace", RemoveNode, Some(KEY_CONTEXT)),
        KeyBinding::new("delete", RemoveNode, Some(KEY_CONTEXT)),
    ]);
}

pub struct NodeView<Def: GraphDefinition> {
    node_id: NodeId,
    graph: Model<Graph<Def>>,
    inputs: Vec<View<InputView<Def>>>,
    outputs: Vec<View<OutputView<Def>>>,
    focus_handle: FocusHandle,
    prev_mouse_pos: Option<Point<Pixels>>,
}

impl<Def: GraphDefinition + 'static> NodeView<Def>
where
    Def::NodeKind: VisualNodeKind,
    Def::NodeData: VisualNodeData,
    Def::DataType: VisualDataType,
    Def::Control: VisualControl<Def>,
{
    pub fn build(node_id: NodeId, graph: Model<Graph<Def>>, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| Self {
            node_id,
            inputs: Self::build_inputs(node_id, &graph, cx),
            outputs: Self::build_outputs(node_id, &graph, cx),
            graph,
            focus_handle: cx.focus_handle().clone(),
            prev_mouse_pos: None,
        })
    }

    pub fn node_id(&self) -> NodeId {
        self.node_id
    }

    fn node<'cx>(&'cx self, cx: &'cx AppContext) -> &Node<Def> {
        self.graph.read(cx).node(self.node_id)
    }

    fn build_inputs(
        node_id: NodeId,
        graph: &Model<Graph<Def>>,
        cx: &mut ViewContext<Self>,
    ) -> Vec<View<InputView<Def>>> {
        let inputs = {
            let graph = graph.read(cx);
            graph.node(node_id).inputs().to_vec()
        };

        inputs
            .into_iter()
            .map({
                let graph = graph.clone();
                move |param| {
                    let input_view =
                        InputView::build(param.id, param.label.clone(), graph.clone(), cx);

                    cx.subscribe(&input_view, {
                        let graph = graph.clone();
                        move |_this, output_view, event, cx| {
                            let input_id = output_view.read(cx).input_id;
                            let ControlEvent::Change(new_value) = event;
                            graph.update(cx, move |_graph, cx| {
                                cx.emit(GraphEvent::ValueChanged {
                                    parameter: Parameter::Input(input_id),
                                    value: new_value.clone(),
                                });
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
        graph: &Model<Graph<Def>>,
        cx: &mut ViewContext<Self>,
    ) -> Vec<View<OutputView<Def>>> {
        let outputs = {
            let graph = graph.read(cx);
            graph.node(node_id).outputs().to_vec()
        };

        outputs
            .into_iter()
            .map({
                let graph = graph.clone();
                move |param| {
                    let output_view =
                        OutputView::build(param.id, param.label.clone(), graph.clone(), cx);

                    cx.subscribe(&output_view, {
                        let graph = graph.clone();
                        move |_this, output_view, event, cx| {
                            let output_id = output_view.read(cx).output_id;
                            let ControlEvent::Change(new_value) = event;
                            graph.update(cx, move |_graph, cx| {
                                cx.emit(GraphEvent::ValueChanged {
                                    parameter: Parameter::Output(output_id),
                                    value: new_value.clone(),
                                });
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

    fn handle_drag_move(&mut self, event: &DragMoveEvent<NodeDrag>, cx: &mut ViewContext<Self>) {
        let node_drag = event.drag(cx);
        if self.node_id != node_drag.node_id {
            return;
        }

        let diff = self
            .prev_mouse_pos
            .map_or(Point::default(), |prev| cx.mouse_position() - prev);

        self.graph.update(cx, |graph, cx| {
            let node_data = &mut graph.node_mut(self.node_id).data;
            let new_position = *node_data.position() + diff.into();
            node_data.set_position(new_position);
            cx.notify();
        });

        cx.emit(GraphEvent::NodeMoved {
            node_id: self.node_id,
        });

        self.prev_mouse_pos = Some(cx.mouse_position());
    }

    fn handle_mouse_up(&mut self, _: &MouseUpEvent, cx: &mut ViewContext<Self>) {
        if self.prev_mouse_pos.is_none() {
            // The node did not move.
            return;
        }

        self.prev_mouse_pos = None;
        cx.emit(GraphEvent::NodeMoveEnded {
            node_id: self.node_id,
        });
    }
}

impl<Def: GraphDefinition + 'static> Render for NodeView<Def>
where
    Def::NodeData: VisualNodeData,
    Def::NodeKind: VisualNodeKind,
    Def::DataType: VisualDataType,
    Def::Control: VisualControl<Def>,
{
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let node = self.node(cx);
        let focused = self.focus_handle.is_focused(cx);

        let header = {
            let label = node.kind().label().to_owned();

            div()
                .h_flex()
                .h(HEADER_HEIGHT)
                .gap_1()
                .px_1()
                .py_px()
                .border_b_1()
                .border_color(cx.theme().border)
                .when(focused, |e| e.border_color(cx.theme().border_selected))
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

        let position = node.data.snapped_position(SNAP_GRID_SIZE);

        div()
            .key_context(KEY_CONTEXT)
            .id(ElementId::Name(format!("node-{:?}", self.node_id).into()))
            .track_focus(&self.focus_handle)
            .absolute()
            .left(px(position.x))
            .top(px(position.y))
            .w(NODE_WIDTH)
            .bg(cx.theme().element_background)
            .border_1()
            .border_color(cx.theme().border)
            .rounded(cx.theme().radius)
            .when(focused, |e| {
                e.bg(cx.theme().element_selected)
                    .border_color(cx.theme().border_selected)
                    .shadow_md()
            })
            .shadow_sm()
            .child(header)
            .child(content)
            .on_action::<RemoveNode>(cx.listener(|this, _, cx| {
                cx.emit(GraphEvent::RemoveNode {
                    node_id: this.node_id,
                });
            }))
            .on_drag(
                NodeDrag {
                    node_id: self.node_id,
                },
                |_, _point, cx| cx.new_view(|_cx| EmptyView),
            )
            .on_drag_move(cx.listener(Self::handle_drag_move))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::handle_mouse_up))
            .on_mouse_up_out(MouseButton::Left, cx.listener(Self::handle_mouse_up))
    }
}

impl<Def: GraphDefinition + 'static> FocusableView for NodeView<Def>
where
    Def::NodeKind: VisualNodeKind,
    Def::NodeData: VisualNodeData,
    Def::DataType: VisualDataType,
    Def::Control: VisualControl<Def>,
{
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl<Def: GraphDefinition + 'static> EventEmitter<SocketEvent> for NodeView<Def> {}

impl<Def: GraphDefinition + 'static> EventEmitter<GraphEvent<Def>> for NodeView<Def> {}

#[derive(Debug, Clone, Copy)]
pub enum NodeEvent {
    Delete { node_id: NodeId },
}

#[derive(Debug, Clone)]
pub enum SocketEvent {
    StartNewEdge(Parameter),
    EndNewEdge,
}

struct NodeDrag {
    pub node_id: NodeId,
}

pub struct InputView<Def: GraphDefinition> {
    input_id: InputId,
    label: String,
    graph: Model<Graph<Def>>,
    control_view: Option<AnyView>,
    hovering: bool,
}

impl<Def: GraphDefinition + 'static> InputView<Def>
where
    Def::DataType: VisualDataType,
    Def::Control: VisualControl<Def>,
{
    pub fn build(
        input_id: InputId,
        label: String,
        graph: Model<Graph<Def>>,
        cx: &mut WindowContext,
    ) -> View<Self> {
        cx.new_view(|cx| {
            let control_id: SharedString = format!("input-{:?}", input_id).into();
            let kind = graph.read(cx).input(input_id).kind.clone();
            match &kind {
                InputParameterKind::EdgeOrConstant { value, control } => {
                    let control_view = control.view(ElementId::Name(control_id), value.clone(), cx);

                    Self {
                        input_id,
                        label,
                        graph,
                        control_view: Some(control_view),
                        hovering: false,
                    }
                }
            }
        })
    }

    fn has_edge(&self, cx: &AppContext) -> bool {
        self.graph.read(cx).edge_source(self.input_id).is_some()
    }
}

impl<Def: GraphDefinition + 'static> Render for InputView<Def>
where
    Def::DataType: VisualDataType,
    Def::Control: VisualControl<Def>,
{
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .h_flex()
            .pr_1()
            .h(SOCKET_HEIGHT)
            .gap_2()
            .child(render_connector(
                &Parameter::Input(self.input_id),
                &self.graph,
                self.hovering,
                cx,
            ))
            .child(self.label.clone())
            .when(!self.has_edge(cx), |e| {
                e.children(self.control_view.clone())
            })
    }
}

impl<Def: GraphDefinition + 'static> EventEmitter<ControlEvent<Def>> for InputView<Def> {}

impl<Def: GraphDefinition + 'static> EventEmitter<SocketEvent> for InputView<Def> {}

impl<Def: GraphDefinition> Hovering for InputView<Def> {
    fn set_hovering(&mut self, hovering: bool) {
        self.hovering = hovering;
    }
}

pub struct OutputView<Def: GraphDefinition> {
    output_id: OutputId,
    label: String,
    control_view: Option<AnyView>,
    graph: Model<Graph<Def>>,
    hovering: bool,
}

impl<Def: GraphDefinition + 'static> OutputView<Def>
where
    Def::DataType: VisualDataType,
    Def::Control: VisualControl<Def>,
{
    pub fn build(
        output_id: OutputId,
        label: String,
        graph: Model<Graph<Def>>,
        cx: &mut WindowContext,
    ) -> View<Self> {
        cx.new_view(|cx| {
            let control_id: SharedString = format!("output-{:?}", output_id).into();
            let kind = graph.read(cx).output(output_id).kind.clone();
            let control_view = match &kind {
                OutputParameterKind::Computed => None,
                OutputParameterKind::Constant { value, control } => {
                    Some(control.view(ElementId::Name(control_id), value.clone(), cx))
                }
            };

            Self {
                output_id,
                label,
                control_view,
                graph,
                hovering: false,
            }
        })
    }
}

impl<Def: GraphDefinition + 'static> Render for OutputView<Def>
where
    Def::DataType: VisualDataType,
{
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .pl_1()
            .h_flex()
            .h(SOCKET_HEIGHT)
            .w_full()
            .flex_row_reverse()
            .gap_2()
            .child(render_connector(
                &Parameter::Output(self.output_id),
                &self.graph,
                self.hovering,
                cx,
            ))
            .child(self.label.clone())
            .children(self.control_view.clone())
    }
}

impl<Def: GraphDefinition + 'static> EventEmitter<SocketEvent> for OutputView<Def> {}

impl<Def: GraphDefinition + 'static> EventEmitter<ControlEvent<Def>> for OutputView<Def> {}

impl<Def: GraphDefinition> Hovering for OutputView<Def> {
    fn set_hovering(&mut self, hovering: bool) {
        self.hovering = hovering;
    }
}

pub enum ControlEvent<Def: GraphDefinition> {
    Change(Def::Value),
}

fn render_connector<Def, View>(
    parameter: &Parameter,
    graph: &Model<Graph<Def>>,
    hovering: bool,
    cx: &ViewContext<View>,
) -> impl IntoElement
where
    Def: GraphDefinition + 'static,
    Def::DataType: VisualDataType,
    View: EventEmitter<SocketEvent> + Hovering,
{
    let width = px(3.0);
    let height = px(13.0);
    let hover_box_size = px(35.0);

    let left = match parameter {
        Parameter::Input(_) => false,
        Parameter::Output(_) => true,
    };

    let data_type = match parameter {
        Parameter::Input(input_id) => &graph.read(cx).input(*input_id).data_type(),
        Parameter::Output(output_id) => &graph.read(cx).output(*output_id).data_type(),
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
                .id(ElementId::Name(format!("socket-{:?}", parameter).into()))
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
                        parameter: *parameter,
                    },
                    |_, _point, cx| cx.new_view(|_cx| EmptyView),
                )
                .on_drag_move(cx.listener({
                    let parameter = *parameter;
                    move |_view, event: &DragMoveEvent<SocketDrag>, cx| {
                        let drag = event.drag(cx);
                        if drag.parameter != parameter {
                            return;
                        }
                        cx.emit(SocketEvent::StartNewEdge(parameter));
                    }
                }))
                .on_mouse_up(
                    MouseButton::Left,
                    cx.listener(|_, _, cx| cx.emit(SocketEvent::EndNewEdge)),
                )
                .on_mouse_up_out(
                    MouseButton::Left,
                    cx.listener(|_, _, cx| cx.emit(SocketEvent::EndNewEdge)),
                ),
        )
}

struct SocketDrag {
    parameter: Parameter,
}

trait Hovering {
    fn set_hovering(&mut self, hovering: bool);
}
