use gpui::{App, Entity, ReadGlobal, Window, div, prelude::*};
use rui::{Column, Table, TableDelegate, TableState};
use zeevonk::project::file::patch::FixtureDefinition;

use crate::app::state::AppState;

pub struct PatchSettingsView {
    table_state: Entity<TableState<PatchTableDelegate>>,
}

impl PatchSettingsView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let fixtures =
            cx.new(|cx| AppState::global(cx).zeevonk().project().file().patch.fixtures.clone());
        Self {
            table_state: cx
                .new(|cx| TableState::new(PatchTableDelegate::new(fixtures), window, cx)),
        }
    }
}

impl Render for PatchSettingsView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(Table::new(self.table_state.clone()))
    }
}

struct PatchTableDelegate {
    data: Entity<Vec<FixtureDefinition>>,
    columns: Vec<Column>,
}

impl PatchTableDelegate {
    fn new(fixtures: Entity<Vec<FixtureDefinition>>) -> Self {
        Self {
            data: fixtures,
            columns: vec![
                Column::new("root_id", "Root Id"),
                Column::new("name", "Name"),
                Column::new("address", "Address"),
                Column::new("kind", "Kind"),
            ],
        }
    }
}

impl TableDelegate for PatchTableDelegate {
    fn columns_count(&self, _cx: &App) -> usize {
        self.columns.len()
    }

    fn rows_count(&self, cx: &App) -> usize {
        self.data.read(cx).len()
    }

    fn column(&self, col_ix: usize, _cx: &App) -> &Column {
        &self.columns[col_ix]
    }

    fn render_td(
        &mut self,
        row_ix: usize,
        col_ix: usize,
        _window: &mut Window,
        cx: &mut Context<TableState<Self>>,
    ) -> impl IntoElement {
        let row = &self.data.read(cx)[row_ix];
        let col = &self.columns[col_ix];

        match col.id().as_ref() {
            "root_id" => row.root_id.to_string(),
            "name" => row.name.clone(),
            "address" => row.address.to_string(),
            "kind" => format!("{} ({})", row.kind.gdtf_fixture_type_id, row.kind.gdtf_dmx_mode),
            _ => "".to_string(),
        }
    }
}
