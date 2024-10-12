use gpui::*;

use crate::{traits, DataType, Graph, NodeKind, Value};

use super::graph::GraphView;

pub struct EditorView<D, V, N>
where
    D: DataType<Value = V>,
    N: NodeKind<DataType = D, Value = V>,
{
    graph_view: Option<View<GraphView<D, V, N>>>,
}

impl<D, V, N> EditorView<D, V, N>
where
    D: DataType<Value = V> + 'static,
    V: traits::Value + 'static,
    N: NodeKind<DataType = D, Value = V> + 'static,
{
    pub fn new(cx: &mut WindowContext, graph_model: Option<Model<Graph<D, V, N>>>) -> Self {
        Self {
            graph_view: graph_model.map(|graph_model| GraphView::build(graph_model, cx)),
        }
    }
}

impl<D, V, N> Render for EditorView<D, V, N>
where
    D: DataType<Value = V> + 'static,
    V: Value + 'static,
    N: NodeKind<DataType = D, Value = V> + 'static,
{
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .text_color(white())
            .text_xs()
            .font_family("IBM Plex Mono")
            .children(self.graph_view.clone())
    }
}
