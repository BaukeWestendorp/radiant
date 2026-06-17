use gpui::{App, Window, div, prelude::*, px};
use rd_engine::patch::FixtureId;
use rd_ui::{Column, TableDelegate, TableState};

use crate::engine::EngineAppExt;

pub struct FixtureTableDelegate {
    columns: Vec<Column>,
}

impl FixtureTableDelegate {
    pub fn new(_window: &mut Window, _cx: &mut Context<TableState<Self>>) -> Self {
        Self {
            columns: vec![
                Column::new("id", "Id").with_min_width(px(150.0)),
                Column::new("name", "Name"),
            ],
        }
    }
}

impl TableDelegate for FixtureTableDelegate {
    type RowId = FixtureId;

    fn column_count(&self, _cx: &App) -> usize {
        self.columns.len()
    }

    fn column(&self, col_ix: usize, _cx: &App) -> &Column {
        &self.columns[col_ix]
    }

    fn root_row_ids(&self, cx: &App) -> Vec<Self::RowId> {
        let patch = cx.engine_snapshot().patch();
        let mut row_ids =
            patch.fixture_ids().filter(|fid| fid.is_root()).copied().collect::<Vec<_>>();
        row_ids.sort();
        row_ids
    }

    fn row_children(&self, row_id: &Self::RowId, cx: &App) -> Vec<Self::RowId> {
        let patch = cx.engine_snapshot().patch();
        let mut sub_ids = patch
            .fixture(&row_id)
            .into_iter()
            .flat_map(|f| f.child_ids().to_vec())
            .collect::<Vec<_>>();
        sub_ids.sort();
        sub_ids
    }

    fn render_cell(
        &self,
        row_id: &Self::RowId,
        col_ix: usize,
        _window: &mut Window,
        cx: &App,
    ) -> impl IntoElement {
        let patch = cx.engine_snapshot().patch();
        let row = patch.fixture(row_id).unwrap();
        let col = &self.columns[col_ix];
        let content = match col.id().as_ref() {
            "id" => row.id().to_string(),
            "name" => row.name().to_string(),
            _ => "".to_string(),
        };
        div().mx_1().child(content)
    }
}
