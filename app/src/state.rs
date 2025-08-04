use std::time::Duration;

use gpui::prelude::*;
use gpui::{App, Entity, EventEmitter, Global, ReadGlobal, Subscription, Timer, Window};
use radiant::engine::{Command, Engine, EngineEvent};
use radiant::show::Show;

pub fn init(engine: Engine, cx: &mut App) {
    AppState::init(engine, cx);
}

pub struct AppState {
    pub engine: Engine,
    event_handler: Entity<EngineEventHandler>,
}

impl AppState {
    fn init(mut engine: Engine, cx: &mut App) {
        let engine_event_handler = cx.new(|cx| EngineEventHandler::new(cx));
        engine.start();
        cx.set_global(AppState { engine, event_handler: engine_event_handler.clone() });
    }
}

impl Global for AppState {}

pub struct EngineEventHandler {}

impl EngineEventHandler {
    fn new(cx: &mut Context<Self>) -> Self {
        cx.spawn(async move |handler, cx| -> eyre::Result<()> {
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

pub fn with_show<F: FnOnce(&Show) -> R, R>(cx: &App, f: F) -> R {
    AppState::global(cx).engine.show().read(f)
}

pub fn exec_cmd(command: Command, cx: &App) -> radiant::error::Result<()> {
    AppState::global(cx).engine.exec(command)
}

pub fn exec_cmd_and_log_err(command: Command, cx: &App) {
    if let Err(err) = exec_cmd(command, cx) {
        log::error!("failed to run command: {err}");
    }
}

pub fn on_engine_event<
    F: Fn(&mut V, &EngineEvent, &mut Window, &mut Context<V>) + 'static,
    V: 'static,
>(
    cx: &mut Context<V>,
    window: &mut Window,
    f: F,
) -> Subscription {
    let event_handler = AppState::global(cx).event_handler.clone();
    cx.subscribe_in(&event_handler, window, move |view: &mut V, _, event, window, cx| {
        f(view, event, window, cx)
    })
}
