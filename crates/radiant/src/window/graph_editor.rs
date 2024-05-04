use backstage::show::graph::{DataType, Graph, Node, NodeId};
use gpui::{
    div, prelude::FluentBuilder, px, rgba, Context, Global, IntoElement, Model, ParentElement,
    Render, SharedString, Styled, View, ViewContext, VisualContext, WindowContext,
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
    nodes: Vec<View<NodeView>>,
}

impl GraphView {
    pub fn build(graph: Model<Graph>, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let nodes = graph
                .read(cx)
                .nodes()
                .cloned()
                .collect::<Vec<_>>()
                .into_iter()
                .filter_map(|node| {
                    NodeView::build(node.id(), graph.clone(), cx)
                        .map_err(|err| {
                            log::error!("Failed to build node: {}", err);
                        })
                        .ok()
                })
                .collect();

            Self { nodes }
        })
    }
}

impl Render for GraphView {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .relative()
            .size_full()
            .bg(THEME.background)
            .children(self.nodes.clone())
    }
}

pub struct NodeView {
    node: Node,
    graph: Model<Graph>,
}

impl NodeView {
    pub fn build(
        node_id: NodeId,
        graph: Model<Graph>,
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
        Ok(cx.new_view(|_cx| Self { node, graph }))
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

    fn render_content(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let inputs = self.node.inputs().iter().filter_map(|(label, input_id)| {
            let Some(input) = self.graph.read(cx).get_input(*input_id) else {
                log::error!("Failed to get input: Input with provided id not found.");
                return None;
            };

            Some(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(self.render_socket(input.data_type()))
                    .child(label.clone()),
            )
        });

        let outputs = self.node.outputs().iter().filter_map(|(label, output_id)| {
            let Some(output) = self.graph.read(cx).get_output(*output_id) else {
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
                    .child(self.render_socket(output.data_type())),
            )
        });

        div().children(inputs).children(outputs)
    }

    fn render_socket(&self, data_type: &DataType) -> impl IntoElement {
        div()
            .size_3()
            .border()
            .border_color(rgba(0x00000080))
            .bg(gpui::rgb(data_type.hex_color()))
            .rounded_full()
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
