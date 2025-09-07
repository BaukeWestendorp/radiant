use std::path::PathBuf;
use std::time::Duration;

use crossbeam_channel::Receiver;
use gpui::{App, AppContext, Context, Entity, EventEmitter, Global, Timer};
use radiant::builtin::{Objects, Patch, Pools, Programmer, ProtocolConfig};
use radiant::comp::Component;
use radiant::engine::Engine;
use radiant::engine::event::EngineEvent;

use crate::error::Result;

pub struct EngineManager {
    pub engine: Engine,
    event_handler: Entity<EventHandler>,
}

impl EngineManager {
    pub fn init(showfile_path: PathBuf, cx: &mut App) -> Result<()> {
        let mut engine = Engine::new(showfile_path);
        engine.start()?;

        let event_rx = engine.event_rx();
        let event_handler = cx.new(|cx| EventHandler::new(event_rx, cx));

        cx.set_global(Self { engine, event_handler });

        Ok(())
    }

    pub fn event_handler(&self) -> Entity<EventHandler> {
        self.event_handler.clone()
    }

    pub fn read_patch<R, F: FnOnce(&Patch) -> R>(cx: &App, f: F) -> R {
        Self::read::<Patch, _, _>(cx, f)
    }

    pub fn read_objects<R, F: FnOnce(&Objects) -> R>(cx: &App, f: F) -> R {
        Self::read::<Objects, _, _>(cx, f)
    }

    pub fn read_pools<R, F: FnOnce(&Pools) -> R>(cx: &App, f: F) -> R {
        Self::read::<Pools, _, _>(cx, f)
    }

    pub fn read_programmer<R, F: FnOnce(&Programmer) -> R>(cx: &App, f: F) -> R {
        Self::read::<Programmer, _, _>(cx, f)
    }

    pub fn read_protocol_config<R, F: FnOnce(&ProtocolConfig) -> R>(cx: &App, f: F) -> R {
        Self::read::<ProtocolConfig, _, _>(cx, f)
    }

    pub(crate) fn read<T: Component, R, F: FnOnce(&T) -> R>(cx: &App, f: F) -> R {
        cx.global::<Self>().engine.component().read(f)
    }
}

impl Global for EngineManager {}

pub struct EventHandler {}

impl EventHandler {
    pub fn new(event_rx: Receiver<EngineEvent>, cx: &mut Context<Self>) -> Self {
        cx.spawn(async move |event_handler, cx| {
            loop {
                let Some(event_handler) = event_handler.upgrade() else {
                    continue;
                };

                let Ok(event) = event_rx.try_recv() else {
                    continue;
                };

                cx.update_entity(&event_handler, |_, cx| cx.emit(event)).unwrap();

                Timer::after(Duration::from_millis(16)).await;
            }
        })
        .detach();

        Self {}
    }
}

impl EventEmitter<EngineEvent> for EventHandler {}
