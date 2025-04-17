use flow::{Graph, GraphDef, gpui::editor::GraphEditorView};
use gpui::*;
use show::assets::AssetId;

pub struct GraphEditor<D: GraphDef> {
    graph_editor_view: Entity<GraphEditorView<D>>,
}

impl<D: GraphDef + 'static> GraphEditor<D> {
    pub fn new<Id: AssetId + 'static>(
        graph: Entity<show::assets::Asset<Graph<D>, Id>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let graph_model = cx.new(|cx| graph.read(cx).data.clone());
        cx.observe(&graph, {
            let graph_model = graph_model.clone();
            move |_, graph, cx| {
                graph_model.update(cx, |gm, cx| *gm = graph.read(cx).data.clone());
            }
        })
        .detach();

        cx.observe(&graph_model, {
            let graph = graph.clone();
            move |_, graph_model, cx| {
                graph.update(cx, |g, cx| {
                    g.data = graph_model.read(cx).clone();
                });
            }
        })
        .detach();

        let graph_editor_view = cx.new(|cx| GraphEditorView::new(graph_model, window, cx));
        Self { graph_editor_view }
    }
}

impl<D: GraphDef + 'static> Render for GraphEditor<D> {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(self.graph_editor_view.clone())
    }
}
