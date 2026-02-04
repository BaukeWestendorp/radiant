use gpui::{App, ReadGlobal, Window, div, prelude::*};
use rui::{Column, TableDelegate};
use zeevonk::project::stage::FixtureId;

use crate::app::state::AppState;

pub struct FixtureTableDelegate {
    columns: Vec<Column>,
}

impl FixtureTableDelegate {
    pub fn new() -> Self {
        Self { columns: vec![Column::new("id", "Id"), Column::new("name", "Name")] }
    }
}

impl TableDelegate for FixtureTableDelegate {
    type RowId = FixtureId;

    fn column_count(&self, _cx: &App) -> usize {
        self.columns.len()
    }

    fn row_count(&self, cx: &App) -> usize {
        let stage = AppState::global(cx).zeevonk().project().stage();
        stage.fixture_count()
    }

    fn column(&self, col_ix: usize, _cx: &App) -> &Column {
        &self.columns[col_ix]
    }

    fn row_id(&self, row_ix: usize, cx: &App) -> Self::RowId {
        let stage = AppState::global(cx).zeevonk().project().stage();

        // FIXME: Prevent sorting this each lookup.
        let mut fixtures = stage.fixtures().values().collect::<Vec<_>>();
        fixtures.sort_by_key(|f| f.id());
        fixtures[row_ix].id()
    }

    fn root_rows(&self, cx: &App) -> impl Iterator<Item = Self::RowId> {
        let stage = AppState::global(cx).zeevonk().project().stage();
        stage.root_fixtures().map(|(id, _)| *id)
    }

    fn row_children(&self, row_id: Self::RowId, cx: &App) -> impl Iterator<Item = Self::RowId> {
        let stage = AppState::global(cx).zeevonk().project().stage();
        stage.sub_fixtures(&row_id).map(|(id, _)| *id).collect::<Vec<_>>().into_iter()
    }

    fn render_td(
        &self,
        row_id: &Self::RowId,
        col_ix: usize,
        _window: &mut Window,
        cx: &App,
    ) -> impl IntoElement {
        let stage = AppState::global(cx).zeevonk().project().stage();

        let row = stage.fixture(row_id).unwrap();
        let col = &self.columns[col_ix];

        let content = match col.id().as_ref() {
            "id" => row.id().to_string(),
            "name" => row.name().to_string(),
            _ => "".to_string(),
        };

        div().mx_1().child(content)
    }
}
