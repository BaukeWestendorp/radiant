use std::collections::HashMap;

use backstage::show::graph::{DataType, Graph, InputId, Node, NodeId, OutputId};
use gpui::{
    canvas, div, point, prelude::FluentBuilder, px, rgba, Bounds, Context, Element, Global,
    IntoElement, Model, ParentElement, Path, Pixels, Render, SharedString, Styled, View,
    ViewContext, VisualContext, WindowContext,
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

            let nodes = graph
                .read(cx)
                .nodes()
                .cloned()
                .collect::<Vec<_>>()
                .into_iter()
                .filter_map(|node| {
                    NodeView::build(node.id(), graph.clone(), socket_bounds.clone(), cx)
                        .map_err(|err| {
                            log::error!("Failed to build node: {}", err);
                        })
                        .ok()
                })
                .collect();

            Self {
                graph,
                socket_bounds,
                nodes,
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
    node: Node,
    graph: Model<Graph>,
    socket_bounds: Model<HashMap<Socket, Bounds<Pixels>>>,
}

impl NodeView {
    pub fn build(
        node_id: NodeId,
        graph: Model<Graph>,
        socket_bounds: Model<HashMap<Socket, Bounds<Pixels>>>,
        cx: &mut WindowContext,
    ) -> anyhow::Result<View<Self>> {
        let node = graph.read(cx).get_node(node_id).cloned().map_or_else(
            || {
                Err(anyhow::anyhow!(
                    "Failed to build node: Node with provided id not found."
                ))
            },
            Ok,
        )?;
        Ok(cx.new_view(|_cx| Self {
            node,
            graph,
            socket_bounds,
        }))
    }

    fn render_header(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let label = self.node.node_type().label();

        div()
            .px_2()
            .w_full()
            .h(cx.line_height())
            .bg(THEME.fill_secondary)
            .border_b()
            .border_color(THEME.border)
            .flex()
            .justify_between()
            .child(label)
    }

    fn render_content(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let inputs = self
            .node
            .inputs()
            .iter()
            .filter_map(|(label, input_id)| {
                let Some(input) = self.graph.read(cx).get_input(*input_id).cloned() else {
                    log::error!("Failed to get input: Input with provided id not found.");
                    return None;
                };

                Some(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(self.render_socket(
                            Socket::Input(*input_id),
                            input.data_type(),
                            self.socket_bounds.clone(),
                        ))
                        .child(label.clone()),
                )
            })
            .collect::<Vec<_>>();

        let outputs = self.node.outputs().iter().filter_map(|(label, output_id)| {
            let Some(output) = self.graph.read(cx).get_output(*output_id).cloned() else {
                log::error!("Failed to get output: Output with provided id not found.");
                return None;
            };

            Some(
                div()
                    .flex()
                    .justify_end()
                    .items_center()
                    .gap_2()
                    .child(label.clone())
                    .child(self.render_socket(
                        Socket::Output(*output_id),
                        output.data_type(),
                        self.socket_bounds.clone(),
                    )),
            )
        });

        div().children(inputs).children(outputs)
    }

    fn render_socket(
        &self,
        socket: Socket,
        data_type: &DataType,
        socket_bounds: Model<HashMap<Socket, Bounds<Pixels>>>,
    ) -> impl IntoElement {
        div()
            .size_3()
            .border()
            .border_color(rgba(0x00000080))
            .bg(gpui::rgb(data_type.hex_color()))
            .rounded_full()
            .child({
                canvas(
                    |_, _| {},
                    move |bounds, _, cx| {
                        dbg!(&bounds);
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
            .child(self.render_header(cx))
            .child(self.render_content(cx))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Socket {
    Input(InputId),
    Output(OutputId),
}
