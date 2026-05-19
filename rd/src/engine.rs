use std::time::Duration;

use gpui::{App, Entity, Global, prelude::*};
use rd_engine::{Event, RadiantEngine, zv::project::FixtureId};

pub struct Engine {
    engine: RadiantEngine,

    selection: Entity<Vec<FixtureId>>,
}

impl Engine {
    pub fn new(engine: RadiantEngine, cx: &mut App) -> Self {
        let selection = cx.new(|_| Vec::new());

        cx.spawn({
            let event_rx = engine.event_rx().clone();
            let selection = selection.clone();
            async move |cx| {
                loop {
                    cx.update(|cx| match event_rx.try_recv() {
                        Ok(event) => match event {
                            Event::SelectionChanged(v) => {
                                selection.write(cx, v);
                            }
                            Event::HighlightChanged(_) => {}
                        },
                        Err(_) => {}
                    });

                    cx.background_executor().timer(Duration::from_secs_f32(1.0 / 60.0)).await;
                }
            }
        })
        .detach();

        Self { engine, selection }
    }

    pub fn engine(&self) -> &RadiantEngine {
        &self.engine
    }

    pub fn selection(&self) -> &Entity<Vec<FixtureId>> {
        &self.selection
    }
}

impl Global for Engine {}
