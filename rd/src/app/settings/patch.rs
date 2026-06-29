use gpui::{App, Context, Entity, Window, div, prelude::*, px};
use rd_engine::patch::{FixtureDefinition, FixtureIdPart};
use rd_ui::{ActiveTheme, Column, Table, TableDelegate, TableState, v_flex};

use crate::engine::EngineAppExt;

pub struct PatchView {
    table: Entity<TableState<PatchTable>>,
}

impl PatchView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let fixture_definitions =
            cx.new(|cx| cx.engine_snapshot().patch().definition().fixtures().to_vec());

        let selection = cx.new(|_| Vec::new());

        Self {
            table: cx.new(|cx| {
                TableState::new(PatchTable::new(fixture_definitions), selection, window, cx)
            }),
        }
    }
}

impl Render for PatchView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let bottom_bar = div().w_full().h_8().border_t_1().border_color(cx.theme().border_primary);

        v_flex().size_full().child(Table::new(self.table.clone())).child(bottom_bar)
    }
}

struct PatchTable {
    fixture_definitions: Entity<Vec<FixtureDefinition>>,

    columns: Vec<Column>,
}

impl PatchTable {
    fn new(fixture_definitions: Entity<Vec<FixtureDefinition>>) -> Self {
        Self {
            fixture_definitions,
            columns: vec![
                Column::new("fixture_id", "Id").with_min_width(px(100.0)),
                Column::new("name", "Name").with_min_width(px(150.0)),
                Column::new("address", "Address").with_min_width(px(100.0)),
                Column::new("fixture_type_id", "Type").with_min_width(px(150.0)),
                Column::new("dmx_mode", "Mode").with_min_width(px(150.0)),
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
        self.fixture_definitions.read(cx).iter().map(|f|f.id()).collect()
    }

    fn render_cell(
        &self,
        row_id: &Self::RowId,
        col_ix: usize,
        _window: &mut Window,
        cx: &App,
    ) -> impl IntoElement {
        let Some(fixture) = self.fixture_definitions.read(cx).iter().find(|f|f.id() == *row_id) else { return div() };
        let column = self.column(col_ix, cx);

        match column.id().as_str() {
            "fixture_id" => div().px_1().child(fixture.id().to_string()),
            "name" => div().px_1().child(fixture.name().to_string()),
            "address" => div().px_1().child(fixture.dmx_address().to_string()),
            "fixture_type_id" => div().px_1().child(fixture.gdtf_file_name().to_string()),
            "dmx_mode" => div().px_1().child(fixture.gdtf_dmx_mode().to_string()),
            _ => div()
        }
    }
}
