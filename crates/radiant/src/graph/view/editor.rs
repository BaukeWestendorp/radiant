use crate::graph::view::graph::GraphView;
use crate::graph::Graph;
use gpui::*;
use ui::theme::ActiveTheme;
use ui::StyledExt;

actions!(graph_editor, [NewNode]);

const CONTEXT: &str = "GraphEditor";

pub fn init(cx: &mut AppContext) {
    cx.bind_keys([KeyBinding::new("space", NewNode, Some(CONTEXT))]);
}

pub struct EditorView {
    graph_view: View<GraphView>,
    focus_handle: FocusHandle,
}

impl EditorView {
    pub fn build(graph: Model<Graph>, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let graph_view = GraphView::build(graph, cx);
            let focus_handle = cx.focus_handle().clone();

            Self {
                graph_view,
                focus_handle,
            }
        })
    }

    fn new_node(&mut self, _: &NewNode, cx: &mut ViewContext<Self>) {
        dbg!("New Node");
    }

    fn render_header(&self, cx: &WindowContext) -> impl IntoElement {
        div()
            .h_flex()
            .px_2()
            .bg(cx.theme().secondary)
            .w_full()
            .h_8()
            .border_b_1()
            .border_color(cx.theme().border)
            .child("header")
    }
}

impl Render for EditorView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .key_context(CONTEXT)
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::new_node))
            .size_full()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .text_size(cx.theme().font_size)
            .font_family(cx.theme().font_family.clone())
            .child(self.render_header(cx))
            .child(self.graph_view.clone())
    }
}

impl FocusableView for EditorView {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}
