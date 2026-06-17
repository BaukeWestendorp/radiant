use gpui::{AnyElement, App, Bounds, Entity, SharedString, Window, div, prelude::*};
use rd_engine::{cmd::Command, event::Event};
use rd_ui::{Table, TableState, TileDelegate};

use crate::{app::ui::FixtureTableDelegate, engine::EngineAppExt};

pub struct FixturesTile {
    table_state: Entity<TableState<FixtureTableDelegate>>,
}

impl FixturesTile {
    pub fn new(window: &mut Window, cx: &mut App) -> Self {
        let selection = cx.new(|cx| cx.engine_snapshot().selection().fixture_ids().to_vec());

        cx.observe(&selection, |selection, cx| {
            let fixture_ids = selection.read(cx).clone();
            cx.execute_engine_cmd(Command::SelectionSet { fixture_ids });
        })
        .detach();

        cx.on_engine_event({
            let selection = selection.clone();
            move |event, cx| match event {
                Event::SelectionChanged => {
                    let fixture_ids = cx.engine_snapshot().selection().fixture_ids().to_vec();
                    if fixture_ids != selection.read(cx).as_slice() {
                        selection.write(cx, fixture_ids);
                    }
                }
                _ => {}
            }
        })
        .detach();

        Self {
            table_state: cx.new(|cx| {
                TableState::new(FixtureTableDelegate::new(window, cx), selection, window, cx)
            }),
        }
    }
}

impl TileDelegate for FixturesTile {
    fn title(&self, _cx: &App) -> SharedString {
        "Fixtures".into()
    }

    fn render_content(&self, _bounds: Bounds<u32>, _window: &mut Window, _cx: &App) -> AnyElement {
        div()
            .size_full()
            .p_px()
            .pt_0()
            .child(Table::new(self.table_state.clone()))
            .into_any_element()
    }
}
