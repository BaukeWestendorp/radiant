use std::path::PathBuf;
use std::time::Duration;

use crossbeam_channel::Receiver;
use gpui::{App, AppContext, Context, Entity, EventEmitter, Global, Timer};
use radiant::builtin::{Objects, Patch, Pools, Programmer, ProtocolConfig};
use radiant::cmd::Command;
use radiant::comp::Component;
use radiant::engine::Engine;
use radiant::engine::event::EngineEvent;

use crate::error::Result;

pub struct EngineManager {
    pub engine: Engine,
    event_handler: Entity<EventHandler>,
}

impl EngineManager {
    pub fn init(showfile_path: Option<PathBuf>, cx: &mut App) -> Result<()> {
        let mut engine = Engine::new(showfile_path);
        engine.start()?;

        let event_rx = engine.event_rx();
        let event_handler = cx.new(|cx| EventHandler::new(event_rx, cx));

        cx.set_global(Self { engine, event_handler });

        Ok(())
    }

    #[inline]
    pub fn event_handler(cx: &App) -> Entity<EventHandler> {
        cx.global::<Self>().event_handler.clone()
    }

    #[inline]
    pub fn read_patch<R, F: FnOnce(&Patch) -> R>(cx: &App, f: F) -> R {
        Self::read::<Patch, _, _>(cx, f)
    }

    #[inline]
    pub fn read_objects<R, F: FnOnce(&Objects) -> R>(cx: &App, f: F) -> R {
        Self::read::<Objects, _, _>(cx, f)
    }

    #[inline]
    pub fn read_pools<R, F: FnOnce(&Pools) -> R>(cx: &App, f: F) -> R {
        Self::read::<Pools, _, _>(cx, f)
    }

    #[inline]
    pub fn read_programmer<R, F: FnOnce(&Programmer) -> R>(cx: &App, f: F) -> R {
        Self::read::<Programmer, _, _>(cx, f)
    }

    #[inline]
    pub fn read_protocol_config<R, F: FnOnce(&ProtocolConfig) -> R>(cx: &App, f: F) -> R {
        Self::read::<ProtocolConfig, _, _>(cx, f)
    }

    #[inline]
    fn read<T: Component, R, F: FnOnce(&T) -> R>(cx: &App, f: F) -> R {
        cx.global::<Self>().engine.component().read(f)
    }

    #[inline]
    pub fn exec(command: Command, cx: &mut App) -> Result<()> {
        cx.global_mut::<Self>().engine.exec(command)
    }

    #[inline]
    pub fn exec_and_log_err(command: Command, cx: &mut App) {
        cx.global_mut::<Self>().engine.exec_and_log_err(command)
    }
}

impl Global for EngineManager {}

pub struct EventHandler {}

impl EventHandler {
    pub fn new(event_rx: Receiver<EngineEvent>, cx: &mut Context<Self>) -> Self {
        cx.spawn(async move |event_handler, cx| {
            loop {
                Timer::after(Duration::from_millis(16)).await;

                let Some(event_handler) = event_handler.upgrade() else {
                    continue;
                };

                let Ok(event) = event_rx.try_recv() else {
                    continue;
                };

                cx.update_entity(&event_handler, |_, cx| cx.emit(event)).unwrap();
            }
        })
        .detach();

        Self {}
    }
}

impl EventEmitter<EngineEvent> for EventHandler {}
