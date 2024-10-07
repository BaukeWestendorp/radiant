use flow::{Graph, GraphNodeKind};
use gpui::*;

use crate::graph::{GraphView, VisualDataType, VisualGraphState, VisualNode};

pub struct Editor<D: VisualDataType, V, N: GraphNodeKind<DataType = D, Value = V> + VisualNode> {
    graph_model: Option<Model<Graph<D, V, N>>>,
    graph_view: Option<View<GraphView<D, V, N>>>,
}

impl<
        D: VisualDataType + 'static,
        V: 'static,
        N: GraphNodeKind<DataType = D, Value = V> + VisualNode + 'static,
    > Editor<D, V, N>
{
    pub fn new(
        cx: &mut WindowContext,
        graph_model: Option<Model<Graph<D, V, N>>>,
        visual_graph_state: VisualGraphState,
    ) -> Self {
        Self {
            graph_model: graph_model.clone(),
            graph_view: graph_model
                .map(|graph_model| GraphView::build(graph_model, visual_graph_state, cx)),
        }
    }
}

impl<
        D: VisualDataType + 'static,
        V: 'static,
        N: GraphNodeKind<DataType = D, Value = V> + VisualNode + 'static,
    > Render for Editor<D, V, N>
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
