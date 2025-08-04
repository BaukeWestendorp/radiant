use std::path::PathBuf;
use std::time::Duration;

use gpui::{
    App, AppContext, Application, Context, Entity, EventEmitter, Global, KeyBinding, ReadGlobal,
    Subscription, Timer, Window,
};
use radiant::engine::{Command, Engine, EngineEvent};
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
                        for event in engine.drain_pending_events().into_iter().collect::<Vec<_>>() {
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

        cx.bind_keys([KeyBinding::new("escape", actions::ClearFixtureSelection, None)]);

        cx.on_action::<actions::ClearFixtureSelection>(|_, cx| {
            AppState::global(cx)
                .engine
                .exec(Command::ClearFixtureSelection)
                .map_err(|err| log::error!("failed to clear fixture selection: {err}"))
                .ok();
        });

        cx.activate(true);
        ui::init(cx);
        AppState::init(showfile_path, cx);

        MainWindow::open(cx).expect("failed to open main window");
    });
}

pub fn with_show<F: FnOnce(&Show) -> R, R>(cx: &App, f: F) -> R {
    AppState::global(cx).engine.show().read(f)
}

pub fn on_engine_event<
    F: Fn(&mut V, &EngineEvent, &mut Window, &mut Context<V>) + 'static,
    V: 'static,
>(
    cx: &mut Context<V>,
    window: &mut Window,
    f: F,
) -> Subscription {
    let engine_event_handler = AppState::global(cx).engine_event_handler.clone();
    cx.subscribe_in(&engine_event_handler, window, move |view: &mut V, _, event, window, cx| {
        f(view, event, window, cx)
    })
}

mod actions {
    gpui::actions!(app, [ClearFixtureSelection]);
}
