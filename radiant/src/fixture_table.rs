use gpui::{App, Entity, Window, div, prelude::*};
use rui::{Column, TableDelegate};
use zeevonk::project::stage::{Fixture, FixtureId};

pub struct FixtureTableDelegate {
    data: Entity<Vec<Fixture>>,
    columns: Vec<Column>,
}

impl FixtureTableDelegate {
    pub fn new(fixtures: Entity<Vec<Fixture>>) -> Self {
        Self { data: fixtures, columns: vec![Column::new("id", "Id"), Column::new("name", "Name")] }
    }
}

impl TableDelegate for FixtureTableDelegate {
    type RowId = FixtureId;

    fn column_count(&self, _cx: &App) -> usize {
        self.columns.len()
    }

    fn row_count(&self, cx: &App) -> usize {
        self.data.read(cx).len()
    }

    fn column(&self, col_ix: usize, _cx: &App) -> &Column {
        &self.columns[col_ix]
    }

    fn row_id(&self, row_ix: usize, cx: &App) -> Self::RowId {
        self.data.read(cx)[row_ix].id()
    }

    fn render_td(
        &self,
        row_id: &Self::RowId,
        col_ix: usize,
        _window: &mut Window,
        cx: &App,
    ) -> impl IntoElement {
        let row = self.data.read(cx).iter().find(|f| f.id() == *row_id).unwrap();
        let col = &self.columns[col_ix];

        let content = match col.id().as_ref() {
            "id" => row.id().to_string(),
            "name" => row.name().to_string(),
            _ => "".to_string(),
        };

        div().mx_1().child(content)
    }
}
