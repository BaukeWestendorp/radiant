use flow::{Graph, GraphDef, gpui::editor::GraphEditorView};
use gpui::{Entity, Window, div, prelude::*};
use show::assets::Asset;

pub struct GraphEditor<D: GraphDef> {
    graph_editor_view: Entity<GraphEditorView<D>>,
}

impl<D: GraphDef + 'static> GraphEditor<D> {
    pub fn new(
        graph: Entity<Asset<Graph<D>>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let graph = crate::utils::map_entity(
            graph,
            |source, cx| source.read(cx).data.clone(),
            |source, target, cx| source.data = target.read(cx).clone(),
            cx,
        );

        let graph_editor_view = cx.new(|cx| GraphEditorView::new(graph, window, cx));
        Self { graph_editor_view }
    }
}

impl<D: GraphDef + 'static> Render for GraphEditor<D> {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(self.graph_editor_view.clone())
    }
}
