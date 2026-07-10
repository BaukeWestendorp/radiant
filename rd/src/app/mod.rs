use std::path::PathBuf;

use gpui::{Window, prelude::*};
use rd_engine::{Engine, Project};
use rd_ui::todo;

mod engine;

pub fn run(showfile_path: Option<PathBuf>) -> anyhow::Result<()> {
    let mut engine = Engine::new();
    if let Some(showfile_path) = showfile_path {
        let project = Project::load_from_folder(showfile_path)?;
        engine.load_project(project)?;
    }

    log::info!("Starting Radiant application");

    rd_ui::build_simple_app().run(|window, cx| {
        engine::init(engine, cx);

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
