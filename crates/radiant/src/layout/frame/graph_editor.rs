use flow::{Graph, GraphDef, gpui::editor::GraphEditorView};
use gpui::{Entity, FocusHandle, Focusable, SharedString, Window, div, prelude::*};
use show::assets::Asset;

use crate::layout::{VirtualWindow, VirtualWindowDelegate};

pub struct GraphEditor<D: GraphDef> {
    asset: Entity<Asset<Graph<D>>>,
    graph_editor_view: Entity<GraphEditorView<D>>,
}

impl<D: GraphDef + 'static> GraphEditor<D> {
    pub fn new(
        asset: Entity<Asset<Graph<D>>>,
        window: &mut Window,
        cx: &mut Context<VirtualWindow<Self>>,
    ) -> Self {
        let graph = crate::utils::map_entity(
            asset.clone(),
            |source, cx| source.read(cx).data.clone(),
            |source, target, cx| source.data = target.read(cx).clone(),
            cx,
        );

        let graph_editor_view = cx.new(|cx| GraphEditorView::new(graph, window, cx));
        Self { asset, graph_editor_view }
    }
}

impl<D: GraphDef + 'static> VirtualWindowDelegate for GraphEditor<D> {
    fn title(&self, cx: &gpui::App) -> SharedString {
        let graph_name = self.asset.read(cx).label.clone();
        format!("Effect Graph Editor [{}]", graph_name).into()
    }

    fn on_close_window(&mut self, _w: &mut Window, _cx: &mut Context<VirtualWindow<Self>>) {}

    fn render_content(
        &mut self,
        _w: &mut Window,
        _cx: &mut Context<VirtualWindow<Self>>,
    ) -> impl IntoElement {
        div().size_full().child(self.graph_editor_view.clone())
    }
}

impl<D: GraphDef + 'static> Focusable for GraphEditor<D> {
    fn focus_handle(&self, cx: &gpui::App) -> FocusHandle {
        self.graph_editor_view.focus_handle(cx)
    }
}
