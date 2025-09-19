use std::path::PathBuf;

use gpui::{App, Application, Entity, Menu, MenuItem, Window};
use ui::overlay::OverlayContainer;

use crate::engine::EngineManager;
use crate::state::AppState;
use crate::window::main::MainWindow;
use crate::window::settings::SettingsWindow;

pub mod actions {
    use gpui::{App, KeyBinding};

    gpui::actions!(radiant, [Quit, OpenSettings]);

    pub(crate) fn init(cx: &mut App) {
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
        cx.bind_keys([KeyBinding::new("secondary-,", OpenSettings, None)]);
    }
}

pub struct RadiantApp {}

impl RadiantApp {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(self, showfile_path: Option<PathBuf>) {
        Application::new().with_assets(ui::assets::Assets).run(move |cx: &mut App| {
            cx.activate(true);

            ui::init(cx).expect("failed to initialize ui crate");

            actions::init(cx);
            init_menus(cx);
            init_actions(cx);
            AppState::init(cx);

            EngineManager::init(showfile_path, cx).expect("failed to initialize AppState");
            MainWindow::open(cx);

            cx.on_window_closed(|cx| {
                if cx.windows().is_empty() {
                    quit(cx);
                }
            })
            .detach();
        });
    }

    pub fn overlays(window: &Window, cx: &App) -> Entity<OverlayContainer> {
        if let Some(Some(main_window)) = window.root::<MainWindow>() {
            return main_window.read(cx).overlays();
        }

        if let Some(Some(settings_window)) = window.root::<SettingsWindow>() {
            return settings_window.read(cx).overlays();
        }

        panic!("could not find OverlayContainer for window");
    }
}

fn init_menus(cx: &mut App) {
    cx.set_menus(vec![Menu {
        name: "radiant".into(),
        items: vec![MenuItem::Action {
            name: "Quit".into(),
            action: Box::new(actions::Quit),
            os_action: None,
        }],
    }]);
}

fn init_actions(cx: &mut App) {
    cx.on_action::<actions::Quit>(|_, cx| quit(cx));
    cx.on_action::<actions::OpenSettings>(|_, cx| AppState::open_settings(cx));
}

fn quit(cx: &mut App) {
    cx.quit();
}
