use anyhow::bail;
use flow_gpui::editor::GraphEditorView;
use gpui::*;
use show::effect_graph::{EffectGraphDefinition, EffectGraphProcessingContext};
use show::fixture::FixtureId;
use show::{FixtureGroup, Show};
use ui::theme::ActiveTheme;

use crate::io::{IoManager, IoManagerEvent};

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
        titlebar: Some(TitlebarOptions {
            title: Some("Radiant".into()),
            ..Default::default()
        }),
        window_min_size: Some(size(px(600.0), px(400.0))),
        ..Default::default()
    };

    cx.open_window(options, |cx| ShowView::build(show, cx))
}

pub struct ShowView {
    io_manager: Model<IoManager>,
    show: Model<Show>,
    editor_view: View<GraphEditorView<EffectGraphDefinition>>,
    focus_handle: FocusHandle,
}

impl ShowView {
    pub fn build(show: Show, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let effect_graph = show.effect_graph().clone();
            let effect_graph_model = cx.new_model(|_cx| effect_graph);

            cx.observe(&effect_graph_model, |this: &mut Self, model, cx| {
                this.show.update(cx, |show, cx| {
                    *show.effect_graph_mut() = model.read(cx).clone();
                });
                cx.set_window_edited(true);
                cx.notify();
            })
            .detach();

            let io_manager = cx.new_model(|cx| {
                let io_manager = IoManager::new(show.dmx_protocols()).unwrap();
                io_manager.start_emitting(cx);
                io_manager
            });

            cx.subscribe(&io_manager, Self::handle_io_manager_event)
                .detach();

            let editor_view = GraphEditorView::build(effect_graph_model, cx);
            let focus_handle = cx.focus_handle().clone();

            Self {
                io_manager,
                show: cx.new_model(|_cx| show),
                editor_view,
                focus_handle,
            }
        })
    }

    fn handle_io_manager_event(
        &mut self,
        io_manager: Model<IoManager>,
        event: &IoManagerEvent,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            IoManagerEvent::OutputRequested => io_manager.update(cx, |io_manager, cx| {
                let mut context = EffectGraphProcessingContext::default();
                context.set_group(FixtureGroup::new(vec![
                    FixtureId(0),
                    FixtureId(1),
                    FixtureId(2),
                    FixtureId(3),
                    FixtureId(4),
                    FixtureId(5),
                ]));
                context
                    .process_frame(self.show.read(cx).effect_graph())
                    .unwrap();
                io_manager.set_dmx_output(context.dmx_output);
            }),
        }
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
            move |_, mut cx| async move {
                let path = match path.await? {
                    Ok(path) => path,
                    Err(err) => bail!("Failed to get save-path: {}", err),
                };

                let Some(path) = path else {
                    bail!("Failed to get save-path: Dialog cancelled.")
                };

                // FIXME: GPUI adds an extra extension sometimes for some reason.

                cx.update(|cx| -> anyhow::Result<()> {
                    show.read(cx).save_to_file(&path)?;

                    cx.set_window_edited(false);
                    cx.add_recent_document(&path);

                    Ok(())
                })??;

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
