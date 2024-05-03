use gpui::{div, IntoElement, ParentElement, Render, Styled, View, VisualContext, WindowContext};

use crate::{Graph, NodeType, Value};

impl<N, D, V> Graph<N, D, V>
where
    N: NodeType<DataType = D, Value = V> + 'static,
    D: 'static,
    V: Value<DataType = D> + 'static,
{
    pub fn build_view(graph: Self, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| graph)
    }
}

impl<N, D, V> Render for Graph<N, D, V>
where
    N: NodeType + 'static,
    D: 'static,
    V: Value + 'static,
{
    fn render(&mut self, _cx: &mut gpui::ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .text_color(gpui::white())
            .child(self.connections.len().to_string())
    }
}
