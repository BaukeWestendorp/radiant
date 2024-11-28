use anyhow::bail;
use dmx::DmxOutput;
use gpui::*;
use show::{EffectGraphProcessingContext, Show};
use ui::theme::ActiveTheme;

use std::cell::RefCell;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;

use crate::io::{IoManager, IoManagerEvent};

actions!(show, [Save, SaveAs, Close]);

const KEY_CONTEXT: &str = "Show";

pub fn init(cx: &mut AppContext) {
    cx.bind_keys([
        KeyBinding::new("cmd-s", Save, Some(KEY_CONTEXT)),
        KeyBinding::new("shift-cmd-s", SaveAs, Some(KEY_CONTEXT)),
        KeyBinding::new("cmd-w", Close, Some(KEY_CONTEXT)),
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
                .unwrap_or_else(|_| ShowView::build(Show::default(), cx)),
            None => ShowView::build(Show::default(), cx),
        };

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
    show: Show,
    _io_manager: Model<IoManager>,

    focus_handle: FocusHandle,

    path: Option<PathBuf>,
    has_unsaved_changes: bool,
}

impl ShowView {
    pub fn read_from_file(path: PathBuf, cx: &mut WindowContext) -> Result<View<Self>> {
        let show_json = fs::read_to_string(&path)?;
        let show: Show = serde_json::from_str(&show_json)?;

        let this = Self::build(show, cx);
        this.update(cx, |this, _cx| {
            this.path = Some(path);
        });
        Ok(this)
    }

    pub fn build(show: Show, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let io_manager = cx.new_model(|cx| {
                let io_manager = IoManager::new(show.dmx_protocols()).unwrap();
                io_manager.start_emitting(cx);
                io_manager
            });

            cx.subscribe(&io_manager, Self::handle_io_manager_event)
                .detach();

            // NOTE: This makes sure the 'unsaved changes' indicator is not shown when a window is opened.
            cx.defer(|this: &mut Self, cx| this.set_has_unsaved_changes(false, cx));

            Self {
                show,
                _io_manager: io_manager,

                focus_handle: cx.focus_handle().clone(),

                path: None,
                has_unsaved_changes: false,
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
            IoManagerEvent::OutputRequested => io_manager.update(cx, |io_manager, _cx| {
                let dmx_output = compute_dmx_output(&self.show);
                io_manager.set_dmx_output(dmx_output);
            }),
        }
    }

    fn handle_save(&mut self, _: &Save, cx: &mut ViewContext<Self>) {
        if let Some(path) = self.path.clone() {
            log::debug!("Saving {}", path.as_path().display());

            let Some(show_json) = serde_json::to_string_pretty(&self.show)
                .map_err(|err| {
                    log::error!("Failed to serialize show: {err}");
                    err
                })
                .ok()
            else {
                log::error!("Failed to save show");
                return;
            };

            fs::write(&path, show_json)
                .map_err(|err| log::error!("{err}"))
                .ok();

            self.set_has_unsaved_changes(false, cx);
            cx.add_recent_document(&path);
        } else {
            cx.dispatch_action(Box::new(SaveAs));
        }
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
        if !self.has_unsaved_changes {
            cx.remove_window();
            return;
        }

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

    fn set_has_unsaved_changes(&mut self, state: bool, cx: &mut WindowContext) {
        cx.set_window_edited(state);
        self.has_unsaved_changes = state;
    }
}

impl Render for ShowView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .key_context(KEY_CONTEXT)
            .flex()
            .size_full()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .text_size(cx.theme().font_size)
            .font_family(cx.theme().font_family.clone())
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

fn compute_dmx_output(show: &Show) -> DmxOutput {
    let dmx_output = Rc::new(RefCell::new(DmxOutput::new()));

    // Set default DMX values
    for fixture in show.patch().fixtures() {
        for channel in &fixture.dmx_mode(show.patch()).dmx_channels {
            if let Some((_, channel_function)) = channel.initial_function() {
                if let Some(offsets) = &channel.offset {
                    let default_bytes = match &channel_function.default.bytes().get() {
                        1 => channel_function.default.to_u8().to_be_bytes().to_vec(),
                        2 => channel_function.default.to_u16().to_be_bytes().to_vec(),
                        _ => panic!("Unsupported default value size"),
                    };

                    for (i, offset) in offsets.iter().enumerate() {
                        let default = default_bytes[i];
                        let address = fixture.dmx_address.with_channel_offset(*offset as u16 - 1);

                        dmx_output.borrow_mut().set_channel_value(address, default)
                    }
                }
            }
        }
    }

    for effect in show.assets().effects() {
        // Initialize context
        let mut context =
            EffectGraphProcessingContext::new(show.clone(), effect.id(), dmx_output.clone());

        // Process frame
        context
            .process_frame()
            .map_err(|err| log::warn!("Failed to process frame: {err}"))
            .ok();
    }

    dmx_output.take()
}
