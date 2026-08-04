use std::collections::HashMap;

use gpui::{App, prelude::*};
use gpui::{Entity, Window, div};
use rd_ui::{ActiveTheme, Column, Table, TableDelegate, TableSelection, TableState, section};

pub struct TablePreview {
    table_a: Entity<TableState<PreviewTableDelegate>>,
    table_b: Entity<TableState<PreviewTableDelegate>>,
}

impl TablePreview {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let selection_a = cx.new(|_| TableSelection::multiple(None, Vec::new()));
        let selection_b = cx.new(|_| TableSelection::single(None, None));

        Self {
            table_a: cx.new(|cx| {
                let delegate = PreviewTableDelegate::new(cx);
                TableState::new(delegate, cx.focus_handle(), window, cx).with_selection(selection_a)
            }),
            table_b: cx.new(|cx| {
                let delegate = PreviewTableDelegate::new(cx);
                TableState::new(delegate, cx.focus_handle(), window, cx).with_selection(selection_b)
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
    items: Entity<HashMap<String, Item>>,

    columns: Vec<Column<Self>>,
}

impl PreviewTableDelegate {
    fn new(cx: &mut App) -> Self {
        Self {
            #[rustfmt::skip]
            items: cx.new(|_| HashMap::from([
                ("row-001".into(), Item { alpha: 1, beta: "one".into(), gamma: Protocol::Artnet }),
                ("row-002".into(), Item { alpha: 2, beta: "two".into(), gamma: Protocol::Artnet }),
                ("row-003".into(), Item { alpha: 3, beta: "three".into(), gamma: Protocol::Artnet }),
                ("row-004".into(), Item { alpha: 5, beta: "five".into(), gamma: Protocol::Artnet }),
                ("row-005".into(),Item { alpha: 8, beta: "eight".into(), gamma: Protocol::Artnet }),
                ("row-006".into(),Item { alpha: 13, beta: "thirteen".into(), gamma: Protocol::Artnet }),
            ])),
            columns: vec![
                Column::new("alpha", "Alpha")
                    .with_sort_handler(|a: &Item, b: &Item| a.alpha.cmp(&b.alpha))
                    .with_cell_builder(|row: &Item, _, _| row.alpha.to_string().into_any_element())
                    .with_auto_enumerable_editor(|row: &mut Item| &mut row.alpha),
                Column::new("beta", "Beta")
                    .with_sort_handler(|a: &Item, b: &Item| a.beta.cmp(&b.beta))
                    .with_cell_builder(|row: &Item, _, _| row.beta.to_string().into_any_element())
                    .with_auto_enumerable_editor(|row: &mut Item| &mut row.beta),
                Column::new("gamma", "Gamma")
                    .with_sort_handler(|a: &Item, b: &Item| {
                        a.gamma.partial_cmp(&b.gamma).unwrap_or(std::cmp::Ordering::Equal)
                    })
                    .with_cell_builder(|row: &Item, _, _| row.gamma.to_string().into_any_element())
                    .with_auto_editor(|row: &mut Item| &mut row.gamma),
            ],
        }
    }
}

impl TableDelegate for PreviewTableDelegate {
    type Row = Item;
    type RowId = String;

    fn columns(&self, _cx: &App) -> impl Iterator<Item = &Column<Self>> {
        self.columns.iter()
    }

    fn column(&self, column_id: &str, _cx: &App) -> Option<&Column<Self>> {
        self.columns.iter().find(|c| c.id() == column_id)
    }

    fn rows(&self) -> Entity<HashMap<Self::RowId, Self::Row>> {
        self.items.clone()
    }

    fn insert_new_row(
        &self,
        _last_item_id: Option<Self::RowId>,
        cx: &mut App,
    ) -> Option<Self::RowId> {
        let new_id = format!("row-{:03}", self.rows().read(cx).len() + 1);
        let new_item = Item::default();
        self.items.update(cx, |items, cx| {
            items.insert(new_id.clone(), new_item);
            cx.notify();
        });
        Some(new_id)
    }
}

#[derive(Debug, Default)]
struct Item {
    alpha: u32,
    beta: String,
    gamma: Protocol,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[derive(rd_ui::Input)]
pub enum Protocol {
    #[default]
    Artnet,
    Sacn,
    PosiStageNet,
    Dmx512,
}

impl std::fmt::Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Protocol::Artnet => write!(f, "Art-Net"),
            Protocol::Sacn => write!(f, "sACN"),
            Protocol::PosiStageNet => write!(f, "PosiStageNet"),
            Protocol::Dmx512 => write!(f, "DMX512"),
        }
    }
}
