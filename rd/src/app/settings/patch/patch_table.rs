use gpui::{App, Entity, Window, div, prelude::*, px};
use rd_engine::patch::{FixtureDefinition, FixtureIdPart};
use rd_ui::{Column, TableDelegate};

use crate::engine::EngineAppExt;

pub struct PatchTable {
    fixture_definitions: Entity<Vec<FixtureDefinition>>,

    columns: Vec<Column>,
}

impl PatchTable {
    pub fn new(fixture_definitions: Entity<Vec<FixtureDefinition>>) -> Self {
        Self {
            fixture_definitions,
            columns: vec![
                Column::new("fixture_id", "Id").with_min_width(px(100.0)),
                Column::new("name", "Name").with_min_width(px(150.0)),
                Column::new("address", "Address").with_min_width(px(100.0)),
                Column::new("fixture_type", "Type").with_min_width(px(150.0)),
            ],
        }
    }
}

impl TableDelegate for PatchTable {
    type RowId = FixtureIdPart;

    fn column_count(&self, _cx: &App) -> usize {
        self.columns.len()
    }

    fn column(&self, col_ix: usize, _cx: &App) -> &Column {
        &self.columns[col_ix]
    }

    fn root_row_ids(&self, cx: &App) -> Vec<Self::RowId> {
        self.fixture_definitions.read(cx).iter().map(|f| f.id).collect()
    }

    fn render_cell(
        &self,
        row_id: &Self::RowId,
        col_ix: usize,
        _window: &mut Window,
        cx: &App,
    ) -> impl IntoElement {
        let Some(fixture) = self.fixture_definitions.read(cx).iter().find(|f| f.id == *row_id)
        else {
            return div();
        };
        let column = self.column(col_ix, cx);

        match column.id().as_str() {
            "fixture_id" => div().px_1().child(fixture.id.to_string()),
            "name" => div().px_1().child(fixture.name.to_string()),
            "address" => div().px_1().child(fixture.dmx_address.to_string()),
            "fixture_type" => {
                let patch = cx.engine_snapshot().patch();
                div().px_1().child(
                    fixture.fixture_kind.display(&patch).unwrap_or("".to_string()).to_string(),
                )
            }
            _ => div(),
        }
    }
}
