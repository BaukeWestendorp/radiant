use std::collections::HashMap;

use gpui::prelude::*;
use gpui::{App, Entity, Window, div};
use rd_ui::{ActiveTheme, Column, Table, TableDelegate, TableState, section};

pub struct TablePreview {
    table_a: Entity<TableState<PreviewTableDelegate>>,
    table_b: Entity<TableState<PreviewTableDelegate>>,
}

impl TablePreview {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let selection_a = cx.new(|_| Vec::new());
        let selection_b = cx.new(|_| Vec::new());

        Self {
            table_a: cx.new(|cx| {
                let delegate = PreviewTableDelegate::new();
                TableState::new(delegate, selection_a, window, cx)
            }),
            table_b: cx.new(|cx| {
                let delegate = PreviewTableDelegate::new();
                TableState::new(delegate, selection_b, window, cx)
            }),
        }
    }
}

impl Render for TablePreview {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .gap_2()
            .p_2()
            .child(
                section("Table Full").w_full().h_24().child(
                    div()
                        .size_full()
                        .border_1()
                        .border_color(cx.theme().border_primary)
                        .child(Table::new(self.table_a.clone())),
                ),
            )
            .child(
                section("Table Small").w_48().h_24().child(
                    div()
                        .size_full()
                        .border_1()
                        .border_color(cx.theme().border_primary)
                        .child(Table::new(self.table_b.clone())),
                ),
            )
    }
}

struct PreviewTableDelegate {
    items: HashMap<String, Item>,

    columns: Vec<Column>,
}

impl PreviewTableDelegate {
    fn new() -> Self {
        Self {
            items: HashMap::from([
                ("row-001".into(), Item { alpha: 1, beta: "one", gamma: 0.1 }),
                ("row-002".into(), Item { alpha: 2, beta: "two", gamma: 0.25 }),
                ("row-003".into(), Item { alpha: 3, beta: "three", gamma: 0.5 }),
                ("row-004".into(), Item { alpha: 5, beta: "five", gamma: 0.9 }),
                ("row-005".into(), Item { alpha: 8, beta: "eight", gamma: 1.3 }),
                ("row-006".into(), Item { alpha: 13, beta: "thirteen", gamma: 2.1 }),
            ]),
            columns: vec![
                Column::new("alpha", "Alpha"),
                Column::new("beta", "Beta"),
                Column::new("gamma", "Gamma"),
            ],
        }
    }
}

impl TableDelegate for PreviewTableDelegate {
    type RowId = String;

    fn root_row_ids(&self, _cx: &App) -> Vec<Self::RowId> {
        self.items.keys().cloned().collect()
    }

    fn column_count(&self, _cx: &App) -> usize {
        self.columns.len()
    }

    fn column(&self, col_ix: usize, _cx: &App) -> &Column {
        &self.columns[col_ix]
    }

    fn render_cell(
        &self,
        row_id: &Self::RowId,
        col_ix: usize,
        _window: &mut Window,
        cx: &App,
    ) -> impl IntoElement {
        let item = self.items.get(row_id).expect("should have item");
        let content = match self.column(col_ix, cx).id().as_str() {
            "alpha" => item.alpha.to_string().into_any_element(),
            "beta" => item.beta.to_string().into_any_element(),
            "gamma" => item.gamma.to_string().into_any_element(),
            _ => gpui::Empty.into_any_element(),
        };

        div().px_1().child(content)
    }
}

#[derive(Debug)]
struct Item {
    alpha: u32,
    beta: &'static str,
    gamma: f32,
}
