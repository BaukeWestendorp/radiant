use gpui::*;
use graph::view::editor::GraphEditorView;
use show::Show;
use ui::theme::ActiveTheme;

pub struct ProjectView {
    show: Show,

    editor_view: View<GraphEditorView>,

    focus_handle: FocusHandle,
}

impl ProjectView {
    pub fn build(show: Show, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let graph_model = cx.new_model(|_cx| show.effect_graph().clone());
            let editor_view = GraphEditorView::build(graph_model, cx);
            let focus_handle = cx.focus_handle().clone();
            Self {
                show,
                editor_view,
                focus_handle,
            }
        })
    }

    fn render_sidebar(&self, cx: &AppContext) -> impl IntoElement {
        div()
            .max_w_40()
            .min_w_40()
            .h_full()
            .bg(cx.theme().secondary)
            .border_r_1()
            .border_color(cx.theme().border)
    }
}

impl Render for ProjectView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .size_full()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .text_size(cx.theme().font_size)
            .font_family(cx.theme().font_family.clone())
            .child(self.render_sidebar(cx))
            .child(self.editor_view.clone())
    }
}

impl FocusableView for ProjectView {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}
