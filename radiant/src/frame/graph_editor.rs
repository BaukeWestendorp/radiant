use flow::{Graph, frontend::gpui::GraphEditorView};

use gpui::*;

pub struct GraphEditor<D: flow::GraphDef> {
    graph_editor_view: Entity<GraphEditorView<D>>,
}

impl<D: flow::GraphDef + 'static> GraphEditor<D> {
    pub fn build(effect_graph: Entity<Graph<D>>, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            let graph_editor_view = GraphEditorView::build(effect_graph, cx);
            Self { graph_editor_view }
        })
    }
}

impl<D: flow::GraphDef + 'static> Render for GraphEditor<D> {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<'_, Self>) -> impl IntoElement {
        div().child(self.graph_editor_view.clone())
    }
}
