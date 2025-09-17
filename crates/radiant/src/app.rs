use std::path::PathBuf;

use gpui::{App, Application, Menu, MenuItem};

use crate::engine::EngineManager;
use crate::window;
use crate::window::main::MainWindow;

pub mod actions {
    use gpui::{App, KeyBinding};

    gpui::actions!(radiant, [Quit]);

    pub(crate) fn init(cx: &mut App) {
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
    }
}

pub fn run(showfile_path: Option<PathBuf>) {
    Application::new().with_assets(ui::assets::Assets).run(move |cx: &mut App| {
        cx.activate(true);

        ui::init(cx).expect("failed to initialize ui crate");

        actions::init(cx);
        init_menus(cx);
        init_actions(cx);
        AppState::init(cx);

        EngineManager::init(showfile_path, cx).expect("failed to initialize AppState");
        MainWindow::open(cx);
    });
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
    cx.on_action::<actions::Quit>(|_, cx| cx.quit())
}
