use std::path::PathBuf;

use gpui::{App, Application};
use radiant::engine::Engine;

use crate::window::patch::PatchWindow;

pub fn run(showfile_path: PathBuf) {
    let engine = Engine::new(showfile_path);

    Application::new().with_assets(ui::assets::Assets).run(move |cx: &mut App| {
        cx.activate(true);
        ui::init(cx).expect("failed to initialize ui crate");
        PatchWindow::open(cx);
    });
}
