use std::path::PathBuf;

use gpui::{App, Application, Menu, MenuItem};
use nui::AppExt;

use crate::engine::EngineManager;
use crate::window::main::MainWindow;
use crate::window::settings::SettingsWindow;

mod actions {
    use gpui::{App, KeyBinding};

    gpui::actions!(radiant, [Quit, OpenSettings, LogDebugInfo]);

    pub(crate) fn init(cx: &mut App) {
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
        cx.bind_keys([KeyBinding::new("secondary-,", OpenSettings, None)]);
        cx.bind_keys([KeyBinding::new("f12", LogDebugInfo, None)]);
    }
}

pub struct RadiantApp {}

impl Default for RadiantApp {
    fn default() -> Self {
        Self::new()
    }
}

impl RadiantApp {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(self, showfile_path: Option<PathBuf>) {
        Application::new().with_assets(nui::assets::Assets).run(move |cx: &mut App| {
            cx.activate(true);

            nui::init(cx).expect("failed to initialize ui crate");

            actions::init(cx);
            init_menus(cx);
            init_actions(cx);

            EngineManager::init(showfile_path, cx).expect("failed to initialize AppState");

            cx.update_wm(|wm, cx| wm.open_singleton_window::<MainWindow>(cx));

            cx.on_window_closed(|cx| {
                if cx.windows().is_empty() {
                    quit(cx);
                }
            })
            .detach();
        });
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

    cx.on_action::<actions::OpenSettings>(|_, cx| {
        cx.update_wm(|wm, cx| wm.open_singleton_window::<SettingsWindow>(cx));
    });

    cx.on_action::<actions::LogDebugInfo>(|_, cx| {
        cx.defer(|cx| {
            if let Some(handle) = cx.active_window() {
                handle
                    .update(cx, |_, window, _| {
                        dbg!(window.context_stack());
                    })
                    .unwrap();
            }
        });
    });
}

fn quit(cx: &mut App) {
    cx.quit();
}
