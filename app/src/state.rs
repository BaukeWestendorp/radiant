use std::time::Duration;

use gpui::prelude::*;
use gpui::{
    App, Entity, EventEmitter, Global, ReadGlobal, Subscription, Timer, UpdateGlobal, Window,
};
use radiant::engine::{Command, CommandBuilder, Engine, EngineEvent, Keyword, Parameter};
use radiant::show::Show;

pub fn init(engine: Engine, cx: &mut App) {
    AppState::init(engine, cx);
}

pub struct AppState {
    engine: Engine,
    pub command_builder: Entity<CommandBuilder>,
    event_handler: Entity<EngineEventHandler>,
}

impl AppState {
    fn init(mut engine: Engine, cx: &mut App) {
        let engine_event_handler = cx.new(|cx| EngineEventHandler::new(cx));
        let command_builder = cx.new(|_| CommandBuilder::new());

        engine.start();

        cx.set_global(AppState {
            engine,
            command_builder,
            event_handler: engine_event_handler.clone(),
        });
    }

    pub fn engine(&self) -> &Engine {
        &self.engine
    }

    pub fn interaction_state(&self, cx: &App) -> InteractionState {
        match self.command_builder.read(cx).first_keyword() {
            Some(Keyword::Store) => InteractionState::Store,
            Some(Keyword::Update) => InteractionState::Update,
            Some(Keyword::Delete) => InteractionState::Delete,
            _ => InteractionState::None,
        }
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

pub enum InteractionState {
    Store,
    Update,
    Delete,
    None,
}

pub fn with_show<F: FnOnce(&Show) -> R, R>(cx: &App, f: F) -> R {
    AppState::global(cx).engine.show().read(f)
}

pub fn exec_cmd(command: Command, cx: &mut App) -> radiant::error::Result<()> {
    AppState::update_global(cx, |state, _| state.engine.exec(command))
}

pub fn exec_cmd_and_log_err(command: Command, cx: &mut App) {
    if let Err(err) = exec_cmd(command, cx) {
        log::error!("failed to run command: {err}");
    }
}

pub fn exec_current_cmd_and_log_err(cx: &mut App) {
    let cb = AppState::global(cx).command_builder.clone();
    let command = cb.update(cx, |cb, cx| {
        let command = cb.resolve();
        cb.clear();
        cx.notify();
        command
    });

    match command {
        Ok(Some(command)) => exec_cmd_and_log_err(command, cx),
        Ok(None) => {}
        Err(err) => log::error!("failed to run command: {err}"),
    }
}

pub fn process_cmd_param(param: impl Into<Parameter>, cx: &mut App) {
    let is_complete = AppState::global(cx).command_builder.clone().update(cx, |cb, cx| {
        let param = param.into();
        if let Err(err) = cb.process_param(param.clone()) {
            log::error!("failed to process command param '{param}': {err}");
        }
        cx.notify();
        cb.is_complete()
    });

    if is_complete {
        exec_current_cmd_and_log_err(cx);
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
