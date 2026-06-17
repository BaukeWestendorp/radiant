use std::sync::Arc;

use gpui::{App, AppContext, Entity, EventEmitter, Global, ReadGlobal, Subscription};
use rd_engine::{EngineHandle, EngineSnapshot, cmd::Command, event::Event};

pub(crate) fn init(handle: EngineHandle, cx: &mut App) {
    let engine_global = EngineGlobal::new(handle, cx);
    cx.set_global(engine_global);
}

pub trait EngineAppExt {
    fn engine(&self) -> &EngineHandle;

    fn engine_snapshot(&self) -> Arc<EngineSnapshot>;

    fn execute_engine_cmd(&self, command: Command);

    fn on_engine_event(&mut self, handler: impl FnMut(&Event, &mut App) + 'static) -> Subscription;
}

impl EngineAppExt for App {
    fn engine(&self) -> &EngineHandle {
        &EngineGlobal::global(self).handle
    }

    fn engine_snapshot(&self) -> Arc<EngineSnapshot> {
        self.engine().snapshot()
    }

    fn execute_engine_cmd(&self, command: Command) {
        if let Err(err) = self.engine().execute(command) {
            log::error!("Failed to execute command: {err}");
        }
    }

    fn on_engine_event(
        &mut self,
        mut handler: impl FnMut(&Event, &mut App) + 'static,
    ) -> Subscription {
        let event_buffer = EngineGlobal::global(self).event_buffer.clone();
        self.subscribe(&event_buffer, move |_, event, cx| handler(event, cx))
    }
}

trait EngineAppExtPrivate {
    fn emit_engine_event(&mut self, event: Event);
}

impl EngineAppExtPrivate for App {
    fn emit_engine_event(&mut self, event: Event) {
        let event_buffer = EngineGlobal::global(self).event_buffer.clone();
        event_buffer.update(self, |_, cx| cx.emit(event));
    }
}

struct EngineEventBus;

impl EventEmitter<Event> for EngineEventBus {}

struct EngineGlobal {
    handle: EngineHandle,
    event_buffer: Entity<EngineEventBus>,
}

impl EngineGlobal {
    pub fn new(handle: EngineHandle, cx: &mut App) -> Self {
        let event_buffer = cx.new(|_| EngineEventBus);

        cx.spawn({
            let handle = handle.clone();
            async move |cx| {
                let event_listener = handle.event_listener();
                while let Ok(first_event) = event_listener.recv_async().await {
                    let _ = cx.update(|cx| {
                        cx.emit_engine_event(first_event);
                        while let Ok(event) = event_listener.try_recv() {
                            cx.emit_engine_event(event);
                        }
                    });
                }
            }
        })
        .detach();

        Self { handle, event_buffer }
    }
}

impl Global for EngineGlobal {}
