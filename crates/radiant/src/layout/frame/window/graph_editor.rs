use crate::show::asset::Asset;
use flow::{Graph, GraphDef, gpui::editor::GraphEditorView};
use gpui::{Entity, Window, div, prelude::*};

pub struct GraphEditorFrame<D: GraphDef> {
    pub asset: Option<Entity<Asset<Graph<D>>>>,
    graph_editor_view: Option<Entity<GraphEditorView<D>>>,
}

impl<D: GraphDef + 'static> GraphEditorFrame<D> {
    pub fn new(
        asset: Option<Entity<Asset<Graph<D>>>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let graph = asset.as_ref().map(|asset| {
            crate::utils::map_entity(
                asset.clone(),
                |source, cx| source.read(cx).data.clone(),
                |source, target, cx| source.data = target.read(cx).clone(),
                cx,
            )
        });

        let graph_editor_view =
            graph.map(|graph| cx.new(|cx| GraphEditorView::new(graph, window, cx)));
        Self { asset, graph_editor_view }
    }
}

impl<D: GraphDef + 'static> Render for GraphEditorFrame<D> {
    fn render(&mut self, _w: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(match &self.graph_editor_view {
            Some(editor) => editor.clone().into_any_element(),
            None => div()
                .size_full()
                .flex()
                .justify_center()
                .items_center()
                .child("No Graph Selected")
                .into_any_element(),
        })
    }
}
