use std::collections::BTreeMap;

use gpui::{App, Entity, Window, div, prelude::*, px};
use rd_ui::{Column, Table, TableDelegate, TableState};

use rd_engine::zv::project::{Fixture, FixtureId, FixtureIdPart};

use crate::engine::EngineManager;

pub struct PatchSettingsView {
    table_state: Entity<TableState<PatchTableDelegate>>,
}

impl PatchSettingsView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let fixtures = cx.new(|cx| EngineManager::snapshot(cx).stage().fixtures().clone());

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
    data: Entity<BTreeMap<FixtureId, Fixture>>,
    columns: Vec<Column>,
}

impl PatchTableDelegate {
    pub fn new(fixtures: Entity<BTreeMap<FixtureId, Fixture>>) -> Self {
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
        self.data.read(cx).keys().filter(|fid| fid.is_root()).map(|fid| fid.root()).collect()
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
        let row = self.data.read(cx).iter().find(|(fid, _)| fid.root() == *row_id).unwrap();
        let col = &self.columns[col_ix];
        let fixture = row.1;

        let content = match col.id().as_ref() {
            "root_id" => row.0.to_string(),
            "name" => fixture.name().to_string(),
            "address" => fixture.base_address().to_string(),
            "kind" => {
                format!(
                    "{} ({})",
                    fixture.gdtf_fixture_type_id().as_uuid(),
                    fixture.gdtf_dmx_mode()
                )
            }
            _ => "".to_string(),
        };

        div().mx_1().child(content)
    }
}
