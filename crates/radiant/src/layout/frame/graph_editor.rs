use flow::{Graph, GraphDef, gpui::editor::GraphEditorView};
use gpui::{Entity, Focusable, Window, div, prelude::*};
use show::assets::Asset;
use ui::ActiveTheme;

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
    fn render(&mut self, w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let contains_focus = self.graph_editor_view.focus_handle(cx).contains_focused(w, cx);

        div()
            .size_full()
            .bg(cx.theme().colors.bg_primary)
            .border_1()
            .border_color(cx.theme().colors.border)
            .when(contains_focus, |e| e.border_color(cx.theme().colors.border_focused))
            .rounded(cx.theme().radius)
            .child(self.graph_editor_view.clone())
    }
}
