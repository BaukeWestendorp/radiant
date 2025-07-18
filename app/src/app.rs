use std::path::PathBuf;

use gpui::{App, Application, Global, Timer, UpdateGlobal};
use radiant::engine::{DMX_OUTPUT_UPDATE_INTERVAL, Engine};
use radiant::showfile::Showfile;

use crate::main_window::MainWindow;

pub struct AppState {
    pub engine: Engine,
}

impl Global for AppState {}

pub fn run(showfile_path: Option<PathBuf>) {
    Application::new().run(move |cx: &mut App| {
        cx.activate(true);

        crate::ui::init(cx);

        let showfile = match showfile_path {
            Some(path) => Showfile::load(&path).expect("failed to load showfile"),
            None => Showfile::default(),
        };

        // Start DMX resolver.
        cx.spawn(async move |cx| {
            loop {
                cx.update(|cx| AppState::update_global(cx, |state, _| state.engine.resolve_dmx()))
                    .map_err(|err| log::error!("failed to resolve dmx: {err}"))
                    .ok();

                Timer::interval(DMX_OUTPUT_UPDATE_INTERVAL).await;
            }
        })
        .detach();

        // Start adapter input handler.
        cx.spawn(async move |cx| {
            loop {
                cx.update(|cx| {
                    AppState::update_global(cx, |state, _| state.engine.handle_adapter_input())
                })
                .map_err(|err| log::error!("failed to handle adapter input: {err}"))
                .ok();

                Timer::after(DMX_OUTPUT_UPDATE_INTERVAL).await;
            }
        })
        .detach();

        let engine = Engine::new(showfile).expect("failed to create engine");
        cx.set_global(AppState { engine });

        MainWindow::open(cx).expect("failed to open main window");
    });
}
