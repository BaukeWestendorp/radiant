use std::path::PathBuf;

use gpui::{App, Application, Global};
use radiant::engine::Engine;

use crate::main_window::MainWindow;

pub struct AppState {
    pub engine: Engine,
}

impl Global for AppState {}

pub fn run(showfile_path: Option<PathBuf>) {
    Application::new().run(move |cx: &mut App| {
        cx.activate(true);
        ui::init(cx);

        let engine = Engine::new(showfile_path.as_ref()).expect("failed to create engine");
        engine.start();
        cx.set_global(AppState { engine });

        MainWindow::open(cx).expect("failed to open main window");
    });
}
