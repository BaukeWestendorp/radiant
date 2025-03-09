use flow_gpui::{Graph, editor::GraphEditorView, flow::GraphDef};
use gpui::*;

pub struct GraphEditor<D: GraphDef> {
    graph_editor_view: Entity<GraphEditorView<D>>,
}

impl<D: GraphDef + 'static> GraphEditor<D> {
    pub fn build(effect_graph: Entity<Graph<D>>, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            let graph_editor_view = GraphEditorView::build(effect_graph, cx);
            Self { graph_editor_view }
        })
    }
}

impl<D: GraphDef + 'static> Render for GraphEditor<D> {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(self.graph_editor_view.clone())
    }
}
