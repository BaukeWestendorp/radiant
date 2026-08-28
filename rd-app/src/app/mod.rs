use std::path::{Path, PathBuf};

use rd_ui::{
    AppBuilder, SettingsAppExt,
    gpui::{App, Entity, Window, div, prelude::*},
};

use crate::{comp::stateful::FixtureKindPicker, engine::EngineAppExt};

mod keymap;
mod settings;

pub(crate) mod action {
    use rd_ui::gpui;

    gpui::actions!([SettingsOpen]);
    gpui::actions!(cmd, [Save, Highlight]);
}

pub(crate) fn init(cx: &mut App) {
    cx.on_action::<action::Save>(|_, cx| match cx.engine().with_project(|p| p.path.clone()) {
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

    cx.on_action::<action::Highlight>(|_, cx| {
        cx.engine().execute(rd::EngineCommand::HighlightToggle);
    });

    cx.on_action::<action::SettingsOpen>(|_, cx| {
        cx.open_settings(Some(rd_ui::settings_window_options(cx)), |window, cx| {
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

    AppBuilder::new().with_window_title("Radiant").run(|window, cx| {
        crate::engine::init(engine, cx);
        init(cx);

        keymap::default_keymap().apply(cx);

        cx.new(|cx| AppView::new(window, cx))
    });

    Ok(())
}

struct AppView {
    fk_picker: Entity<FixtureKindPicker>,
}

impl AppView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self { fk_picker: cx.new(|cx| FixtureKindPicker::new(window, cx)) }
    }
}

impl Render for AppView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().p_2().child(self.fk_picker.clone())
    }
}
