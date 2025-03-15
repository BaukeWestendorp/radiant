use flow_gpui::{Graph, editor::GraphEditorView, flow::GraphDef};
use gpui::*;

pub struct GraphEditor<D: GraphDef> {
    graph_editor_view: Entity<GraphEditorView<D>>,
}
impl<D: GraphDef + 'static> GraphEditor<D>
where
    D::DataType: flow_gpui::DataType<D>,
{
    pub fn build(graph: Entity<Graph<D>>, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            let graph_editor_view = GraphEditorView::build(graph, cx);
            Self { graph_editor_view }
        })
    }
}

impl<D: GraphDef + 'static> Render for GraphEditor<D> {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(self.graph_editor_view.clone())
    }
}
