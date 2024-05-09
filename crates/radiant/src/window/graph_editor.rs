use std::collections::HashMap;

use backstage::show::graph::{Graph, GraphNode, InputId, NodeId, OutputId, ValueType};
use gpui::{
    canvas, div, point, prelude::FluentBuilder, px, rgba, Bounds, Context, DragMoveEvent, Element,
    Global, InteractiveElement, IntoElement, Model, ParentElement, Path, Pixels, Point, Render,
    SharedString, StatefulInteractiveElement, Styled, View, ViewContext, VisualContext,
    WindowContext,
};

use crate::{showfile::Showfile, theme::THEME};

use super::{WindowDelegate, WindowView};

pub struct GraphEditorWindowDelegate {
    graph: Option<View<GraphView>>,
}

impl GraphEditorWindowDelegate {
    pub fn new(graph_id: Option<usize>, cx: &mut WindowContext) -> Self {
        let graph = match graph_id {
            Some(graph_id) => {
                if let Some(graph) = Showfile::get(cx).show.data().graph(graph_id).cloned() {
                    Some(GraphView::build(cx.new_model(|_cx| graph), cx))
                } else {
                    None
                }
            }
            None => None,
        };

        Self { graph }
    }
}

impl WindowDelegate for GraphEditorWindowDelegate {
    fn title(&mut self, _cx: &mut ViewContext<WindowView<Self>>) -> Option<SharedString> {
        Some("Graph Editor".into())
    }

    fn render_content(&mut self, _cx: &mut ViewContext<WindowView<Self>>) -> impl IntoElement {
        div().size_full().when_else(
            self.graph.is_some(),
            |this| this.child(self.graph.as_ref().unwrap().clone()),
            |this| this.child("No graph selected"),
        )
    }
}

pub struct GraphView {
    graph: Model<Graph>,
    socket_bounds: Model<HashMap<Socket, Bounds<Pixels>>>,
    nodes: Vec<View<NodeView>>,
}

impl GraphView {
    pub fn build(graph: Model<Graph>, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let socket_bounds = cx.new_model(|_cx| HashMap::new());

            let get_node_views = |graph: Model<Graph>,
                                  socket_bounds: Model<HashMap<Socket, Bounds<Pixels>>>,
                                  cx: &mut WindowContext| {
                graph
                    .read(cx)
                    .nodes()
                    .clone()
                    .into_iter()
                    .filter_map(|(node_id, node)| {
                        NodeView::build(node, node_id, graph.clone(), socket_bounds.clone(), cx)
                            .map_err(|err| {
                                log::error!("Failed to build node: {}", err);
                            })
                            .ok()
                    })
                    .collect()
            };

            cx.observe(&graph, {
                let socket_bounds = socket_bounds.clone();
                move |this: &mut Self, graph, cx| {
                    this.nodes = get_node_views(graph.clone(), socket_bounds.clone(), cx);
                    cx.notify();
                }
            })
            .detach();

            Self {
                graph: graph.clone(),
                socket_bounds: socket_bounds.clone(),
                nodes: get_node_views(graph, socket_bounds, cx),
            }
        })
    }

    fn render_connections(&self, cx: &mut WindowContext) -> impl Element {
        let socket_bounds = self.socket_bounds.read(cx).clone();
        let connections = self.graph.read(cx).connections().clone();

        canvas(
            |_, _| {},
            move |_bounds, _, cx| {
                for (input, output) in connections {
                    let input_bounds = socket_bounds
                        .get(&Socket::Input(input))
                        .cloned()
                        .unwrap_or_default();
                    let output_bounds = socket_bounds
                        .get(&Socket::Output(output))
                        .cloned()
                        .unwrap_or_default();

                    let start = output_bounds.center();
                    let end = input_bounds.center();
                    let middle = point((start.x + end.x) / 2.0, (start.y + end.y) / 2.0);

                    let mut path = Path::new(start);
                    path.curve_to(middle, point(middle.x, start.y));
                    path.curve_to(end, end);
                    path.curve_to(middle, point(middle.x, end.y));
                    path.curve_to(start, start);

                    cx.paint_path(path, gpui::red());
                }
            },
        )
        .size_full()
    }
}

impl Render for GraphView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .relative()
            .size_full()
            .bg(THEME.background)
            .children(self.nodes.clone())
            .child(self.render_connections(cx))
    }
}

pub struct NodeView {
    node: GraphNode,
    node_id: NodeId,
    graph: Model<Graph>,
    socket_bounds: Model<HashMap<Socket, Bounds<Pixels>>>,
}

impl NodeView {
    const SOCKET_SIZE: Pixels = px(12.0);

    pub fn build(
        node: GraphNode,
        node_id: NodeId,
        graph: Model<Graph>,
        socket_bounds: Model<HashMap<Socket, Bounds<Pixels>>>,
        cx: &mut WindowContext,
    ) -> anyhow::Result<View<Self>> {
        Ok(cx.new_view(|_cx| Self {
            node,
            node_id,
            graph,
            socket_bounds,
        }))
    }

    fn render_header(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let label = self.node.kind().label().to_string();

        div()
            .px_2()
            .w_full()
            .h(cx.line_height())
            .bg(THEME.fill_secondary)
            .border_b()
            .border_color(THEME.border)
            .flex()
            .items_center()
            .justify_between()
            .child(label)
    }

    fn render_content(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let inputs = self
            .node
            .inputs()
            .into_iter()
            .filter_map(|input_id| {
                let Some(input) = self.graph.read(cx).input(input_id).cloned() else {
                    log::error!("Failed to get input: Input with provided id not found.");
                    return None;
                };

                Some(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .ml(-Self::SOCKET_SIZE / 2.0)
                        .child(self.render_socket(
                            Socket::Input(input_id),
                            input.value_types(),
                            self.socket_bounds.clone(),
                        ))
                        .child(input.label().to_string()),
                )
            })
            .collect::<Vec<_>>();

        let outputs = self.node.outputs().into_iter().filter_map(|output_id| {
            let Some(output) = self.graph.read(cx).output(output_id).cloned() else {
                log::error!("Failed to get output: Output with provided id not found.");
                return None;
            };

            Some(
                div()
                    .flex()
                    .justify_end()
                    .items_center()
                    .gap_2()
                    .mr(-Self::SOCKET_SIZE / 2.0)
                    .child(output.label().to_string())
                    .child(self.render_socket(
                        Socket::Output(output_id),
                        output.value_types(),
                        self.socket_bounds.clone(),
                    )),
            )
        });

        div()
            .size_full()
            .overflow_hidden()
            .children(inputs)
            .children(outputs)
    }

    fn render_socket(
        &self,
        socket: Socket,
        value_types: &[ValueType],
        socket_bounds: Model<HashMap<Socket, Bounds<Pixels>>>,
    ) -> impl IntoElement {
        let color = match value_types.len() {
            0 => rgba(0xffffff40),
            1 => gpui::rgb(value_types[0].hex_color()),
            _ => rgba(0xffffffa0),
        };
        div()
            .size(Self::SOCKET_SIZE)
            .border()
            .border_color(rgba(0x00000080))
            .bg(color)
            .rounded_full()
            .child({
                canvas(
                    |_, _| {},
                    move |bounds, _, cx| {
                        socket_bounds.update(cx, |socket_bounds, _cx| {
                            socket_bounds.insert(socket, bounds)
                        });
                    },
                )
                .size_full()
            })
    }
}

impl Render for NodeView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .id(SharedString::from(format!("{:?}", self.node_id)))
            .absolute()
            .left(px(self.node.x()))
            .top(px(self.node.y()))
            .w_40()
            .h_32()
            .bg(THEME.fill)
            .rounded_md()
            .border()
            .border_color(THEME.border)
            .flex()
            .flex_col()
            .text_sm()
            .child(self.render_header(cx))
            .child(self.render_content(cx))
            .on_drag(
                DraggedNode::new(
                    self.node_id,
                    point(
                        px(self.node.x() - cx.mouse_position().x.0),
                        px(self.node.y() - cx.mouse_position().y.0),
                    ),
                ),
                |dragged_node, cx| cx.new_view(|_cx| dragged_node.clone()),
            )
            .on_drag_move(cx.listener(|this, event: &DragMoveEvent<DraggedNode>, cx| {
                if event.drag(cx).id == this.node_id {
                    this.graph.update(cx, |graph, cx| {
                        if let Some(node) = graph.node_mut(this.node_id) {
                            node.set_position(
                                event.event.position.x.0 + event.drag(cx).grab_offset.x.0,
                                event.event.position.y.0 + event.drag(cx).grab_offset.y.0,
                            );
                            cx.notify();
                        }
                    });
                }
            }))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Socket {
    Input(InputId),
    Output(OutputId),
}

#[derive(Debug, Clone, Render)]
pub struct DraggedNode {
    pub id: NodeId,
    pub grab_offset: Point<Pixels>,
}

impl DraggedNode {
    pub fn new(id: NodeId, grab_offset: Point<Pixels>) -> Self {
        Self { id, grab_offset }
    }
}
