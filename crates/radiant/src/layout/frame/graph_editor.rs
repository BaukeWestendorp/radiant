use flow::{Graph, GraphDef, gpui::editor::GraphEditorView};
use gpui::{Entity, FocusHandle, Focusable, Window, div, prelude::*};
use show::assets::Asset;

use crate::layout::{VirtualWindow, VirtualWindowDelegate};

pub struct GraphEditor<D: GraphDef> {
    graph_editor_view: Entity<GraphEditorView<D>>,
}

impl<D: GraphDef + 'static> GraphEditor<D> {
    pub fn new(
        graph: Entity<Asset<Graph<D>>>,
        window: &mut Window,
        cx: &mut Context<VirtualWindow<Self>>,
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

impl<D: GraphDef + 'static> VirtualWindowDelegate for GraphEditor<D> {
    fn title(&self, _cx: &gpui::App) -> &str {
        "Effect Graph Editor"
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
