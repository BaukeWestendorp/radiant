use std::path::PathBuf;

use gpui::{App, Application};

use crate::engine::EngineManager;
use crate::window;
use crate::window::main::MainWindow;

pub fn run(showfile_path: Option<PathBuf>) {
    Application::new().with_assets(ui::assets::Assets).run(move |cx: &mut App| {
        cx.activate(true);

        ui::init(cx).expect("failed to initialize ui crate");
        window::main::actions::init(cx);

        EngineManager::init(showfile_path, cx).expect("failed to initialize AppState");
        MainWindow::open(cx);
    });
}
