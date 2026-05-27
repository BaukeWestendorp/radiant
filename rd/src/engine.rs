use std::{sync::Arc, time::Duration};

use gpui::{App, Entity, Global, ReadGlobal as _, prelude::*};
use rd_engine::{
    Command, Engine, EngineHandle, EngineSnapshot, Event, EventListener, RunningEngine,
    zv::project::FixtureId,
};

const SYNC_INTERVAL: Duration = Duration::from_nanos(16_666_667);

pub struct EngineManager {
    _running: RunningEngine,
    engine: EngineHandle,

    snapshot: Entity<Arc<EngineSnapshot>>,
    selection: Entity<Vec<FixtureId>>,
}

impl EngineManager {
    pub fn new(engine: Engine, cx: &mut App) -> Self {
        let running = engine.spawn();
        let engine = running.handle().clone();

        let initial_snapshot = engine.snapshot();

        let snapshot = cx.new(|_| Arc::clone(&initial_snapshot));
        let selection = cx.new(|_| initial_snapshot.selection().fixtures().to_vec());
        let pending_selection = cx.new(|_| None::<Vec<FixtureId>>);

        spawn_engine(
            engine.clone(),
            engine.event_listener().clone(),
            snapshot.clone(),
            selection.clone(),
            pending_selection.clone(),
            cx,
        );

        observe_ui_selection_to_engine(
            engine.clone(),
            selection.clone(),
            pending_selection.clone(),
            cx,
        );

        Self { _running: running, engine, snapshot, selection }
    }

    pub fn snapshot<'a>(cx: &'a App) -> &'a EngineSnapshot {
        Self::global(cx).snapshot.read(cx).as_ref()
    }

    pub fn execute(cx: &App, command: Command) {
        Self::global(cx).engine.execute(command);
    }

    pub fn selection(&self) -> &Entity<Vec<FixtureId>> {
        &self.selection
    }
}

impl Global for EngineManager {}

#[derive(Default, Debug, Clone, Copy)]
struct DrainedEvents {
    saw_any: bool,
    saw_selection_changed: bool,
}

fn drain_events(listener: &EventListener) -> DrainedEvents {
    let mut drained = DrainedEvents::default();
    while let Some(event) = listener.try_recv() {
        drained.saw_any = true;
        if matches!(event, Event::SelectionChanged) {
            drained.saw_selection_changed = true;
        }
    }
    drained
}

fn spawn_engine(
    engine: EngineHandle,
    event_listener: EventListener,
    snapshot: Entity<Arc<EngineSnapshot>>,
    selection: Entity<Vec<FixtureId>>,
    pending_selection: Entity<Option<Vec<FixtureId>>>,
    cx: &mut App,
) {
    cx.spawn(async move |cx| {
        loop {
            cx.update(|cx| {
                let drained = drain_events(&event_listener);
                if !drained.saw_any {
                    return;
                }

                let latest = engine.snapshot();
                snapshot.write(cx, Arc::clone(&latest));

                if drained.saw_selection_changed {
                    apply_engine_selection(&latest, &selection, &pending_selection, cx);
                }
            });

            cx.background_executor().timer(SYNC_INTERVAL).await;
        }
    })
    .detach();
}

fn apply_engine_selection(
    latest: &Arc<EngineSnapshot>,
    selection: &Entity<Vec<FixtureId>>,
    pending_selection: &Entity<Option<Vec<FixtureId>>>,
    cx: &mut App,
) {
    let new_selection = latest.selection().fixtures().to_vec();

    if let Some(pending) = pending_selection.read(cx).as_ref() {
        if pending.as_slice() == new_selection.as_slice() {
            pending_selection.write(cx, None);
        } else {
            return;
        }
    }

    if selection.read(cx).as_slice() != new_selection.as_slice() {
        selection.write(cx, new_selection);
    }
}

fn observe_ui_selection_to_engine(
    engine: EngineHandle,
    selection: Entity<Vec<FixtureId>>,
    pending_selection: Entity<Option<Vec<FixtureId>>>,
    cx: &mut App,
) {
    cx.observe(&selection, move |selection, cx| {
        let fixture_ids = selection.read(cx).clone();
        let snapshot = engine.snapshot();
        let current_fixture_ids = snapshot.selection().fixtures();

        if fixture_ids.as_slice() != current_fixture_ids {
            pending_selection.write(cx, Some(fixture_ids.clone()));
            engine.execute(Command::SelectionSet { fixture_ids });
        }
    })
    .detach();
}
