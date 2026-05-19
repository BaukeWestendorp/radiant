use std::time::Duration;

use gpui::{App, Entity, Global, ReadGlobal, prelude::*};
use rd_engine::{Event, HighlightCommand, RadiantEngine, SelectionCommand, zv::project::FixtureId};

pub struct Engine {
    engine: RadiantEngine,

    selection: Entity<Vec<FixtureId>>,

    highlight: Entity<bool>,
}

impl Engine {
    pub fn new(engine: RadiantEngine, cx: &mut App) -> Self {
        let selection = cx.new(|_| Vec::new());
        let highlight = cx.new(|_| false);

        cx.spawn({
            let event_rx = engine.event_rx().clone();
            let selection = selection.clone();
            let highlight = highlight.clone();
            async move |cx| {
                loop {
                    cx.update(|cx| match event_rx.try_recv() {
                        Ok(event) => match event {
                            Event::SelectionChanged(v) => {
                                selection.write(cx, v);
                            }
                            Event::HighlightChanged(v) => {
                                highlight.write(cx, v);
                            }
                        },
                        Err(_) => {}
                    });

                    cx.background_executor().timer(Duration::from_secs_f32(1.0 / 60.0)).await;
                }
            }
        })
        .detach();

        cx.observe(&selection, |selection, cx| {
            let selection = selection.read(cx).clone();
            Engine::global(cx)
                .engine()
                .exec_without_emit(SelectionCommand::Overwrite(selection.into()));
        })
        .detach();

        cx.observe(&highlight, |highlight, cx| {
            let highlight = highlight.read(cx).clone();
            Engine::global(cx).engine().exec_without_emit(HighlightCommand::Set(highlight.into()));
        })
        .detach();

        Self { engine, highlight, selection }
    }

    pub fn engine(&self) -> &RadiantEngine {
        &self.engine
    }

    pub fn selection(&self) -> &Entity<Vec<FixtureId>> {
        &self.selection
    }

    pub fn highlight(&self) -> &Entity<bool> {
        &self.highlight
    }
}

impl Global for Engine {}
