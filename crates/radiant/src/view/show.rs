use anyhow::bail;
use flow_gpui::editor::GraphEditorView;
use gpui::*;
use show::effect_graph::EffectGraphDefinition;
use show::Show;
use ui::theme::ActiveTheme;

actions!(show, [Save]);

const CONTEXT: &str = "Show";

pub fn init(cx: &mut AppContext) {
    cx.bind_keys([KeyBinding::new("cmd-s", Save, Some(CONTEXT))]);
}

pub fn open_show_window(show: Show, cx: &mut AppContext) -> anyhow::Result<WindowHandle<ShowView>> {
    let options = WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
            None,
            size(px(1200.0), px(800.0)),
            cx,
        ))),
        ..Default::default()
    };

    cx.open_window(options, |cx| ShowView::build(show, cx))
}

pub struct ShowView {
    show: Show,
    editor_view: View<GraphEditorView<EffectGraphDefinition>>,
    focus_handle: FocusHandle,
}

impl ShowView {
    pub fn build(show: Show, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let effect_graph = show.effect_graph().clone();
            let effect_graph_model = cx.new_model(|_cx| effect_graph);

            cx.observe(&effect_graph_model, |this: &mut Self, model, cx| {
                *this.show.effect_graph_mut() = model.read(cx).clone();
                cx.notify();
            })
            .detach();

            let editor_view = GraphEditorView::build(effect_graph_model, cx);
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

    fn handle_save(&mut self, _: &Save, cx: &mut ViewContext<Self>) {
        let initial_directory = dirs::desktop_dir().unwrap_or_else(|| dirs::home_dir().unwrap());

        let path = cx.prompt_for_new_path(initial_directory.as_path());
        cx.spawn({
            let show = self.show.clone();
            move |_, _cx| async move {
                let path = match path.await? {
                    Ok(path) => path,
                    Err(err) => bail!("Failed to get save-path: {}", err),
                };

                let Some(path) = path else {
                    bail!("Failed to get save-path: Dialog cancelled.")
                };

                // FIXME: GPUI adds an extra extension for some reason.

                show.save_to_file(&path)?;

                Ok(())
            }
        })
        .detach_and_log_err(cx);
    }
}

impl Render for ShowView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .key_context(CONTEXT)
            .flex()
            .size_full()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .text_size(cx.theme().font_size)
            .font_family(cx.theme().font_family.clone())
            .child(self.render_sidebar(cx))
            .child(self.editor_view.clone())
            .on_action(cx.listener(Self::handle_save))
    }
}

impl FocusableView for ShowView {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}
