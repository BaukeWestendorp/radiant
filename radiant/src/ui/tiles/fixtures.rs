use gpui::{AnyElement, App, Entity, Window, prelude::*};
use rui::{Table, TableState, TileDelegate};

use crate::ui::fixture_table::FixtureTableDelegate;

pub struct FixturesTile {
    table_state: Entity<TableState<FixtureTableDelegate>>,
}

impl FixturesTile {
    pub fn new(window: &mut Window, cx: &mut App) -> Self {
        Self {
            table_state: cx.new(|cx| TableState::new(FixtureTableDelegate::new(cx), window, cx)),
        }
    }
}

impl TileDelegate for FixturesTile {
    fn title(&self) -> &str {
        "Fixtures"
    }

    fn render_content(&self, _window: &mut Window, _cx: &App) -> AnyElement {
        Table::new(self.table_state.clone()).into_any_element()
    }
}
