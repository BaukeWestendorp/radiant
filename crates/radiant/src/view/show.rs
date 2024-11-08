use anyhow::bail;
use flow_gpui::editor::GraphEditorView;
use gpui::*;
use show::effect_graph::{EffectGraphDefinition, EffectGraphProcessingContext};
use show::fixture::FixtureId;
use show::{FixtureGroup, Show};
use std::path::PathBuf;
use std::time::Duration;
use ui::theme::ActiveTheme;

use crate::io::{IoManager, IoManagerEvent};

actions!(show, [Save, SaveAs, Close]);

const CONTEXT: &str = "Show";

pub fn init(cx: &mut AppContext) {
    cx.bind_keys([
        KeyBinding::new("cmd-s", Save, Some(CONTEXT)),
        KeyBinding::new("shift-cmd-s", SaveAs, Some(CONTEXT)),
        KeyBinding::new("cmd-w", Close, Some(CONTEXT)),
    ]);
}

pub fn open_show_window(
    path: Option<PathBuf>,
    cx: &mut AppContext,
) -> Result<WindowHandle<ShowView>> {
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

    cx.open_window(options, |cx| {
        let show_view = match path {
            Some(path) => ShowView::read_from_file(path, cx)
                .map_err(|err| log::error!("Failed to open show: {err}. Opening default show."))
                .unwrap_or(ShowView::build(Show::default(), cx)),
            None => ShowView::build(Show::default(), cx),
        };

        // FIXME: This is a hack to make sure the 'is edited' indicator is disabled on opening of window.
        cx.spawn::<_, Result<()>>(|mut cx| async move {
            Timer::after(Duration::from_millis(100)).await;
            cx.update(|cx| cx.set_window_edited(false))?;
            Ok(())
        })
        .detach_and_log_err(cx);

        cx.on_window_should_close({
            let show_view = show_view.clone();
            move |cx| {
                show_view.update(cx, |view, cx| view.close_window(cx));
                false
            }
        });

        show_view
    })
}

pub struct ShowView {
    _io_manager: Model<IoManager>,
    show: Model<Show>,
    editor_view: View<GraphEditorView<EffectGraphDefinition>>,
    focus_handle: FocusHandle,
    path: Option<PathBuf>,
}

impl ShowView {
    pub fn read_from_file(path: PathBuf, cx: &mut WindowContext) -> Result<View<Self>> {
        let show = Show::read_from_file(&path)?;
        let this = Self::build(show, cx);
        this.update(cx, |this, _cx| {
            this.path = Some(path);
        });
        Ok(this)
    }

    pub fn build(show: Show, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let effect_graph = show.effect_graph().clone();
            let effect_graph_model = cx.new_model(|_cx| effect_graph);

            cx.observe(&effect_graph_model, |this: &mut Self, graph_model, cx| {
                this.show.update(cx, |show, cx| {
                    *show.effect_graph_mut() = graph_model.read(cx).clone();
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
                _io_manager: io_manager,
                show: cx.new_model(|_cx| show),
                editor_view,
                focus_handle,
                path: None,
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
        if let Some(path) = &self.path {
            log::debug!("Saving {}", path.as_path().display());

            self.show
                .update(cx, |show, _cx| show.save_to_file(path.as_path()))
                .map_err(|err| log::error!("{err}"))
                .ok();

            cx.set_window_edited(false);
            cx.add_recent_document(&path);

            return;
        }

        cx.dispatch_action(Box::new(SaveAs));
    }

    fn handle_save_as(&mut self, _: &SaveAs, cx: &mut ViewContext<Self>) {
        let initial_directory = dirs::desktop_dir().unwrap_or_else(|| dirs::home_dir().unwrap());

        let path = cx.prompt_for_new_path(initial_directory.as_path());
        cx.spawn({
            move |this, mut cx| async move {
                let path = match path.await? {
                    Ok(path) => path,
                    Err(err) => bail!("Failed to get save-path: {}", err),
                };

                let Some(path) = path else {
                    bail!("Failed to get save-path: Dialog cancelled.")
                };

                cx.update(|cx| -> Result<()> {
                    this.update(cx, |this, _cx| this.path = Some(path))?;
                    cx.dispatch_action(Box::new(Save));
                    Ok(())
                })??;

                Ok(())
            }
        })
        .detach_and_log_err(cx);
    }

    fn handle_close(&mut self, _: &Close, cx: &mut ViewContext<Self>) {
        self.close_window(cx);
    }

    fn close_window(&self, cx: &mut ViewContext<Self>) {
        let answer_task = cx.prompt(
            PromptLevel::Warning,
            "You have unsaved changes",
            None,
            &["Save and close", "Close without saving", "Cancel"],
        );

        cx.spawn::<_, Result<()>>(move |this, mut cx| async move {
            let Ok(answer) = answer_task.await else {
                return Ok(());
            };

            if matches!(answer, 0 | 1) {
                cx.update(move |cx| -> Result<()> {
                    this.update(cx, |this: &mut Self, cx| {
                        if answer == 0 {
                            this.handle_save(&Save, cx);
                        }
                        cx.remove_window();
                    })
                })??;
            }

            Ok(())
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
            .on_action(cx.listener(Self::handle_save_as))
            .on_action(cx.listener(Self::handle_close))
    }
}

impl FocusableView for ShowView {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}
