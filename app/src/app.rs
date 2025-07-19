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
        ui::init(cx);

        let showfile = load_showfile(showfile_path);
        let engine = Engine::new(showfile).expect("failed to create engine");
        cx.set_global(AppState { engine });

        spawn_dmx_resolver(cx);
        spawn_adapter_handler(cx);

        MainWindow::open(cx).expect("failed to open main window");
    });
}

fn load_showfile(path: Option<PathBuf>) -> Showfile {
    match path {
        Some(path) => Showfile::load(&path).expect("failed to load showfile"),
        None => Showfile::default(),
    }
}

fn spawn_dmx_resolver(cx: &mut App) {
    cx.spawn(async move |cx| {
        loop {
            if let Err(err) =
                cx.update(|cx| AppState::update_global(cx, |state, _| state.engine.resolve_dmx()))
            {
                log::error!("failed to resolve DMX: {err}");
            }

            Timer::interval(DMX_OUTPUT_UPDATE_INTERVAL).await;
        }
    })
    .detach();
}

fn spawn_adapter_handler(cx: &mut App) {
    cx.spawn(async move |cx| {
        loop {
            if let Err(err) = cx.update(|cx| {
                AppState::update_global(cx, |state, _| state.engine.handle_adapter_input())
            }) {
                log::error!("failed to handle adapter input: {err}");
            }

            Timer::after(DMX_OUTPUT_UPDATE_INTERVAL).await;
        }
    })
    .detach();
}
