use std::time::Duration;

use gpui::{App, Entity, Global, ReadGlobal as _, prelude::*};
use rd_engine::{
    EngineHandle, EngineSnapshot,
    cmd::Command,
    event::{Event, EventListener},
    patch::FixtureId,
};

const SYNC_INTERVAL: Duration = Duration::from_nanos(16_666_667);

pub struct EngineManager {
    handle: EngineHandle,

    snapshot: Entity<EngineSnapshot>,
    selection: Entity<Vec<FixtureId>>,
}

impl EngineManager {
    pub fn new(handle: EngineHandle, cx: &mut App) -> Self {
        let initial_snapshot = handle.snapshot();

        let selection = cx.new(|_| initial_snapshot.selection().fixture_ids().to_vec());
        let snapshot = cx.new(|_| initial_snapshot);

        spawn_engine(
            handle.clone(),
            handle.event_listener().clone(),
            snapshot.clone(),
            selection.clone(),
            cx,
        );

        observe_ui_selection_to_engine(handle.clone(), selection.clone(), cx);

        Self { handle, snapshot, selection }
    }

    pub fn snapshot<'a>(cx: &'a App) -> &'a EngineSnapshot {
        Self::global(cx).snapshot.read(cx)
    }

    pub fn execute(cx: &App, command: Command) {
        if let Err(err) = Self::global(cx).handle.execute(command) {
            log::error!("Failed to execute command: {err}");
        }
    }

    pub fn selection(cx: &App) -> &Entity<Vec<FixtureId>> {
        &Self::global(cx).selection
    }
}

impl Global for EngineManager {}

fn spawn_engine(
    engine: EngineHandle,
    event_listener: EventListener,
    snapshot: Entity<EngineSnapshot>,
    selection: Entity<Vec<FixtureId>>,
    cx: &mut App,
) {
    cx.spawn(async move |cx| {
        loop {
            cx.update(|cx| {
                let mut handled_event = false;
                let mut selection_changed = false;
                while let Some(event) = event_listener.try_recv() {
                    match event {
                        Event::SelectionChanged => selection_changed = true,
                        Event::HighlightChanged => {}
                        Event::ExecutorChanged(_) => {}
                    }
                    handled_event = true;
                }

                if !handled_event {
                    return;
                }

                let latest = engine.snapshot();

                if selection_changed {
                    apply_engine_selection(&latest, &selection, cx);
                }

                snapshot.write(cx, latest);
            });
            cx.background_executor().timer(SYNC_INTERVAL).await;
        }
    })
    .detach();
}

fn apply_engine_selection(
    latest: &EngineSnapshot,
    selection: &Entity<Vec<FixtureId>>,
    cx: &mut App,
) {
    let new_selection = latest.selection().fixture_ids().to_vec();
    selection.write(cx, new_selection);
}

fn observe_ui_selection_to_engine(
    engine: EngineHandle,
    selection: Entity<Vec<FixtureId>>,
    cx: &mut App,
) {
    cx.observe(&selection, move |selection, cx| {
        let fixture_ids = selection.read(cx).clone();
        let snapshot = engine.snapshot();
        let current_fixture_ids = snapshot.selection().fixture_ids();

        if fixture_ids.as_slice() != current_fixture_ids {
            if let Err(err) = engine.execute(Command::SelectionSet { fixture_ids }) {
                log::error!("Failed to execute command: {err}");
            }
        }
    })
    .detach();
}
