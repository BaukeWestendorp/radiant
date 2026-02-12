use gpui::{App, Window, div, prelude::*, px};
use rui::{Column, TableDelegate, TableEvent, TableState};
use zeevonk::project::stage::FixtureId;

use crate::app::state::AppState;

pub struct FixtureTableDelegate {
    columns: Vec<Column>,
}

impl FixtureTableDelegate {
    pub fn new(cx: &mut Context<TableState<Self>>) -> Self {
        cx.observe(&AppState::show(cx).selection(), |table_state, selection, cx| {
            let selection = selection.read(cx).clone();
            table_state.clear_selection(cx);
            for fixture_id in selection {
                table_state.expand_parents(&fixture_id, cx);
                table_state.select_cell(table_state.selected_column(), &fixture_id, cx);
            }
        })
        .detach();

        cx.subscribe_self(|this, event, cx| match event {
            TableEvent::SelectionChanged => {
                let new_selection = this.selected_rows();

                AppState::show(cx).selection().update(cx, |selection, cx| {
                    if *selection != new_selection {
                        *selection = new_selection;
                        cx.notify();
                    }
                })
            }
        })
        .detach();

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
        let stage = AppState::zeevonk(cx).project().stage();
        let mut row_ids = stage.roots().map(|(id, _)| *id).collect::<Vec<_>>();
        row_ids.sort();
        row_ids
    }

    fn row_children(&self, row_id: &Self::RowId, cx: &App) -> Vec<Self::RowId> {
        let stage = AppState::zeevonk(cx).project().stage();
        let mut sub_ids = stage.children(&row_id).map(|(id, _)| *id).collect::<Vec<_>>();
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
        let stage = AppState::zeevonk(cx).project().stage();

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
