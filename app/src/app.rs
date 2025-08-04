use std::path::PathBuf;

use gpui::{App, Application};
use radiant::engine::Engine;

use crate::main_window::MainWindow;
use crate::state;

pub fn run(showfile_path: Option<PathBuf>) {
    let engine = Engine::new(showfile_path.as_ref()).expect("failed to create engine");

    Application::new().with_assets(ui::Assets).run(move |cx: &mut App| {
        cx.activate(true);

        ui::init(cx).expect("failed to initialize ui crate");
        actions::init(cx);

        state::init(engine, cx);

        MainWindow::open(cx).expect("failed to open main window");
    });
}

mod actions {
    use gpui::{App, KeyBinding};
    use radiant::engine::Command;

    use crate::state::exec_cmd_and_log_err;

    gpui::actions!(app, [ClearFixtureSelection]);

    pub fn init(cx: &mut App) {
        bind_keys(cx);
        bind_actions(cx);
    }

    fn bind_keys(cx: &mut App) {
        cx.bind_keys([KeyBinding::new("escape", ClearFixtureSelection, None)]);
    }

    fn bind_actions(cx: &mut App) {
        cx.on_action::<ClearFixtureSelection>(|_, cx| {
            exec_cmd_and_log_err(Command::ClearFixtureSelection, cx);
        });
    }
}
