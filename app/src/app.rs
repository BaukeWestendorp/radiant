use std::path::PathBuf;
use std::time::Duration;

use gpui::{
    App, AppContext, Application, Context, Entity, EventEmitter, Global, ReadGlobal, Timer,
};
use radiant::engine::{Engine, EngineEvent};
use radiant::show::Show;

use crate::assets::{self, Assets};
use crate::error::Result;
use crate::main_window::MainWindow;

pub struct AppState {
    pub engine: Engine,
    pub engine_event_handler: Entity<EngineEventHandler>,
}

impl AppState {
    pub fn init(showfile_path: Option<PathBuf>, cx: &mut App) {
        let engine_event_handler = cx.new(|cx| EngineEventHandler::new(cx));

        let engine = Engine::new(showfile_path.as_ref()).expect("failed to create engine");
        engine.start();
        cx.set_global(AppState { engine, engine_event_handler: engine_event_handler.clone() });
    }
}

impl Global for AppState {}

pub struct EngineEventHandler {}

impl EngineEventHandler {
    fn new(cx: &mut Context<Self>) -> Self {
        cx.spawn(async move |handler, cx| -> Result<()> {
            loop {
                handler
                    .update(cx, |_, cx| {
                        let engine = &AppState::global(cx).engine;
                        for event in engine.pending_events() {
                            cx.emit(event);
                        }
                    })
                    .map_err(|err| eyre::eyre!(err))?;

                Timer::after(Duration::from_secs_f32(1.0 / 30.0)).await;
            }
        })
        .detach_and_log_err(cx);

        Self {}
    }
}

impl EventEmitter<EngineEvent> for EngineEventHandler {}

pub fn run(showfile_path: Option<PathBuf>) {
    Application::new().with_assets(Assets).run(move |cx: &mut App| {
        assets::load_fonts(cx).expect("failed to load fonts");

        cx.activate(true);
        ui::init(cx);
        AppState::init(showfile_path, cx);

        MainWindow::open(cx).expect("failed to open main window");
    });
}

pub fn with_show<F: FnOnce(&Show) -> R, R>(cx: &App, f: F) -> R {
    AppState::global(cx).engine.show().read(f)
}
