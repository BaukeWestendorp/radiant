use std::path::{Path, PathBuf};

use gpui::{App, Window, prelude::*};
use rd_ui::{SettingsAppExt, todo};

use crate::app::engine::EngineAppExt;

mod engine;
mod keymap;
mod settings;

gpui::actions!([SettingsOpen]);
gpui::actions!(cmd, [Save, Highlight]);

pub(crate) fn init(cx: &mut App) {
    cx.on_action::<Save>(|_, cx| match cx.engine().with_project(|p| p.path.clone()) {
        Some(path) => {
            cx.engine().execute(rd::EngineCommand::Save { path });
        }
        None => {
            let path_prompt = cx.prompt_for_new_path(Path::new(""), None);
            cx.spawn(async |cx| match path_prompt.await {
                Ok(Ok(path)) => match path {
                    Some(path) => {
                        cx.update(|cx| {
                            cx.engine().execute(rd::EngineCommand::Save { path });
                        });
                    }
                    None => {
                        log::warn!("Failed to save project: No path provided");
                    }
                },
                Ok(Err(err)) => {
                    log::warn!("Failed to save project: {err:?}");
                }
                Err(_) => {
                    log::warn!("Failed to save project: File prompt was cancelled.");
                }
            })
            .fallible()
            .detach();
        }
    });

    cx.on_action::<Highlight>(|_, cx| {
        cx.engine().execute(rd::EngineCommand::HighlightToggle);
    });

    cx.on_action::<SettingsOpen>(|_, cx| {
        cx.open_settings(Some(rd_ui::SETTINGS_WINDOW_OPTIONS), |window, cx| {
            cx.new(|cx| settings::SettingsRootView::new(window, cx)).into()
        });
    });
}

pub fn run(showfile_path: Option<PathBuf>) -> anyhow::Result<()> {
    let engine = rd::Engine::new();
    if let Some(showfile_path) = showfile_path {
        let project = rd::Project::load_from_folder(showfile_path)?;
        engine.load_project(project)?;
    }

    log::info!("Starting Radiant application");

    rd_ui::build_app().run(|window, cx| {
        engine::init(engine, cx);
        init(cx);

        keymap::default_keymap().apply(cx);

        cx.new(|cx| AppView::new(window, cx))
    });

    Ok(())
}

struct AppView {}

impl AppView {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {}
    }
}

impl Render for AppView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        todo(cx)
    }
}
