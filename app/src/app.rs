use std::path::PathBuf;

use gpui::{App, Application, Global, ReadGlobal};
use radiant::engine::Engine;
use radiant::show::Show;

use crate::assets::{self, Assets};
use crate::main_window::MainWindow;

pub struct AppState {
    pub engine: Engine,
}

impl Global for AppState {}

pub fn run(showfile_path: Option<PathBuf>) {
    Application::new().with_assets(Assets).run(move |cx: &mut App| {
        assets::load_fonts(cx).expect("failed to load fonts");

        cx.activate(true);
        ui::init(cx);

        let engine = Engine::new(showfile_path.as_ref()).expect("failed to create engine");
        engine.start();
        cx.set_global(AppState { engine });

        MainWindow::open(cx).expect("failed to open main window");
    });
}

pub fn with_show<F: FnOnce(&Show) -> R, R>(cx: &App, f: F) -> R {
    AppState::global(cx).engine.show().read(f)
}
