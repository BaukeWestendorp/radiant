use std::collections::HashMap;

use gpui::prelude::*;
use gpui::{Entity, Window, div};
use rd_ui::{ActiveTheme, Column, Table, TableDelegate, TableSelection, TableState, section};

pub struct TablePreview {
    table_a: Entity<TableState<PreviewTableDelegate>>,
    table_b: Entity<TableState<PreviewTableDelegate>>,
}

impl TablePreview {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let selection_a = cx.new(|_| TableSelection::Multiple(Vec::new()));
        let selection_b = cx.new(|_| TableSelection::Single(None));

        Self {
            table_a: cx.new(|cx| {
                let delegate = PreviewTableDelegate::new();
                TableState::new(delegate, window, cx).with_selection(selection_a)
            }),
            table_b: cx.new(|cx| {
                let delegate = PreviewTableDelegate::new();
                TableState::new(delegate, window, cx).with_selection(selection_b)
            }),
        }
    }
}

impl Render for TablePreview {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .gap_2()
            .p_2()
            .child(
                section("Table Full").w_full().h_48().child(
                    div()
                        .size_full()
                        .border_1()
                        .border_color(cx.theme().border_primary)
                        .child(Table::new("table-full", self.table_a.clone(), window, cx)),
                ),
            )
            .child(
                section("Table Small").w_48().h_48().child(
                    div()
                        .size_full()
                        .border_1()
                        .border_color(cx.theme().border_primary)
                        .child(Table::new("table-small", self.table_b.clone(), window, cx)),
                ),
            )
    }
}

struct PreviewTableDelegate {
    items: HashMap<String, Item>,

    columns: Vec<Column<Self>>,
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
                Column::new("alpha", "Alpha")
                    .with_cell_builder(|row: &Item, _window, _cx| {
                        row.alpha.to_string().into_any_element()
                    })
                    .with_edit_handler(|_rows| {
                        todo!();
                    })
                    .with_sort_handler(|a: &Item, b: &Item| a.alpha.cmp(&b.alpha)),
                Column::new("beta", "Beta")
                    .with_cell_builder(|row: &Item, _window, _cx| {
                        row.beta.to_string().into_any_element()
                    })
                    .with_edit_handler(|_rows| {
                        todo!();
                    })
                    .with_sort_handler(|a: &Item, b: &Item| a.beta.cmp(&b.beta)),
                Column::new("gamma", "Gamma")
                    .with_cell_builder(|row: &Item, _window, _cx| {
                        row.gamma.to_string().into_any_element()
                    })
                    .with_edit_handler(|_rows| {
                        todo!();
                    })
                    .with_sort_handler(|a: &Item, b: &Item| {
                        a.gamma.partial_cmp(&b.gamma).unwrap_or(std::cmp::Ordering::Equal)
                    }),
            ],
        }
    }
}

impl TableDelegate for PreviewTableDelegate {
    type RowId = String;
    type Row = Item;

    fn columns(&self) -> &[Column<Self>] {
        &self.columns
    }

    fn rows(&self) -> impl IntoIterator<Item = &Self::Row> {
        self.items.values()
    }
}

#[derive(Debug)]
struct Item {
    alpha: u32,
    beta: &'static str,
    gamma: f32,
}
