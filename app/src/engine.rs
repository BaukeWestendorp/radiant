use std::path::PathBuf;
use std::time::Duration;

use gpui::{App, AppContext, Context, Entity, EventEmitter, Global, Timer};
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
}

impl Global for EngineManager {}

pub struct EventHandler {}

impl EventHandler {
    pub fn new(event_rx: crossbeam_channel::Receiver<EngineEvent>, cx: &mut Context<Self>) -> Self {
        cx.spawn(async move |event_handler, cx| {
            loop {
                if let Some(event_handler) = event_handler.upgrade() {
                    if let Ok(event) = event_rx.try_recv() {
                        cx.update_entity(&event_handler, |_, cx| cx.emit(event)).unwrap();
                    }
                }
                Timer::after(Duration::from_millis(16)).await;
            }
        })
        .detach();

        Self {}
    }
}

impl EventEmitter<EngineEvent> for EventHandler {}
