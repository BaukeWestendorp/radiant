use gpui::{
    App, AppContext, Entity, EventEmitter, Global, ReadGlobal, Subscription, UpdateGlobal, Window,
};
use rd_engine::{Engine, Event, Project};

pub(crate) fn init(engine: Engine, cx: &mut App) {
    let engine_global = EngineGlobal::new(engine, cx);
    cx.set_global(engine_global);
}

pub trait EngineAppExt {
    fn engine(&self) -> &Engine;

    fn update_project<R, F: FnOnce(&mut Project, &mut App) -> R>(
        &mut self,
        f: F,
    ) -> anyhow::Result<R>;

    fn on_engine_event(&mut self, handler: impl FnMut(&Event, &mut App) + 'static) -> Subscription;

    fn on_engine_event_in(
        &mut self,
        window: &mut Window,
        handler: impl FnMut(&Event, &mut Window, &mut App) + 'static,
    ) -> Subscription;
}

impl EngineAppExt for App {
    fn engine(&self) -> &Engine {
        &EngineGlobal::global(self).engine
    }

    fn update_project<R, F: FnOnce(&mut Project, &mut App) -> R>(
        &mut self,
        f: F,
    ) -> anyhow::Result<R> {
        EngineGlobal::update_global(self, |engine, cx| {
            engine.engine.update_project(|project| (f)(project, cx))
        })
    }

    fn on_engine_event(
        &mut self,
        mut handler: impl FnMut(&Event, &mut App) + 'static,
    ) -> Subscription {
        let event_buffer = EngineGlobal::global(self).event_buffer.clone();
        self.subscribe(&event_buffer, move |_, event, cx| handler(event, cx))
    }

    fn on_engine_event_in(
        &mut self,
        window: &mut Window,
        mut handler: impl FnMut(&Event, &mut Window, &mut App) + 'static,
    ) -> Subscription {
        let event_buffer = EngineGlobal::global(self).event_buffer.clone();
        window
            .subscribe(&event_buffer, self, move |_, event, window, cx| handler(event, window, cx))
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
    engine: Engine,
    event_buffer: Entity<EngineEventBus>,
}

impl EngineGlobal {
    pub fn new(engine: Engine, cx: &mut App) -> Self {
        let event_buffer = cx.new(|_| EngineEventBus);

        cx.spawn({
            let engine = engine.clone();
            async move |cx| {
                let events = engine.events();
                while let Ok(first_event) = events.recv_async().await {
                    let _ = cx.update(|cx| {
                        cx.emit_engine_event(first_event);
                        while let Ok(event) = events.try_recv() {
                            cx.emit_engine_event(event);
                        }
                    });
                }
            }
        })
        .detach();

        Self { engine, event_buffer }
    }
}

impl Global for EngineGlobal {}
