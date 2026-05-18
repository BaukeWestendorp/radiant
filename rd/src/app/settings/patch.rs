use gpui::{App, Entity, ReadGlobal, Window, div, prelude::*, px};
use rd_ui::{Column, Table, TableDelegate, TableState};
use zeevonk::project::{FixtureDefinition, FixtureIdPart};

use crate::engine::Engine;

pub struct PatchSettingsView {
    table_state: Entity<TableState<PatchTableDelegate>>,
}

impl PatchSettingsView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let fixtures =
            cx.new(|cx| Engine::global(cx).zeevonk().project().file().patch.fixtures.clone());

        Self {
            table_state: cx.new(|cx| {
                TableState::new(
                    PatchTableDelegate::new(fixtures),
                    cx.new(|_| Vec::new()),
                    window,
                    cx,
                )
            }),
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
    pub fn new(fixtures: Entity<Vec<FixtureDefinition>>) -> Self {
        Self {
            data: fixtures,
            columns: vec![
                Column::new("root_id", "Root Id").with_min_width(px(75.0)),
                Column::new("name", "Name").with_min_width(px(250.0)),
                Column::new("address", "Address").with_min_width(px(100.0)),
                Column::new("kind", "Kind"),
            ],
        }
    }
}

impl TableDelegate for PatchTableDelegate {
    type RowId = FixtureIdPart;

    fn column_count(&self, _cx: &App) -> usize {
        self.columns.len()
    }

    fn column(&self, col_ix: usize, _cx: &App) -> &Column {
        &self.columns[col_ix]
    }

    fn root_row_ids(&self, cx: &App) -> Vec<Self::RowId> {
        self.data.read(cx).iter().map(|f| f.root_id).collect()
    }

    fn edit_rows(&self, _row_ids: &[Self::RowId], _cx: &App) {
        eprintln!("edit: {_row_ids:?}");
    }

    fn delete_rows(&self, _row_ids: &[Self::RowId], _cx: &App) {
        eprintln!("delete: {_row_ids:?}");
    }

    fn render_cell(
        &self,
        row_id: &Self::RowId,
        col_ix: usize,
        _window: &mut Window,
        cx: &App,
    ) -> impl IntoElement {
        let row = self.data.read(cx).iter().find(|f| &f.root_id == row_id).unwrap();
        let col = &self.columns[col_ix];

        let content = match col.id().as_ref() {
            "root_id" => row.root_id.to_string(),
            "name" => row.name.clone(),
            "address" => row.address.to_string(),
            "kind" => format!("{} ({})", row.kind.gdtf_fixture_type_id, row.kind.gdtf_dmx_mode),
            _ => "".to_string(),
        };

        div().mx_1().child(content)
    }
}
