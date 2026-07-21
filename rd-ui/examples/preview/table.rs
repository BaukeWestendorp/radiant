use std::collections::HashMap;

use gpui::prelude::*;
use gpui::{Entity, Window, div};
use rd_ui::{
    ActiveTheme, Column, DropdownItem, Table, TableDelegate, TableSelection, TableState, section,
};

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
            #[rustfmt::skip]
            items: HashMap::from([
                ("row-001".into(), Item { alpha: 1, beta: "one".into(), gamma: Protocol::Artnet }),
                ("row-002".into(), Item { alpha: 2, beta: "two".into(), gamma: Protocol::Artnet }),
                ("row-003".into(), Item { alpha: 3, beta: "three".into(), gamma: Protocol::Artnet }),
                ("row-004".into(), Item { alpha: 5, beta: "five".into(), gamma: Protocol::Artnet }),
                ("row-005".into(),Item { alpha: 8, beta: "eight".into(), gamma: Protocol::Artnet }),
                ("row-006".into(),Item { alpha: 13, beta: "thirteen".into(), gamma: Protocol::Artnet }),
            ]),
            columns: vec![
                Column::new("alpha", "Alpha")
                    .with_clonable_field_edit_handler(|row: &mut Item| &mut row.alpha)
                    .with_sort_handler(|a: &Item, b: &Item| a.alpha.cmp(&b.alpha))
                    .with_cell_builder(|row: &Item, _, _| row.alpha.to_string().into_any_element()),
                Column::new("beta", "Beta")
                    .with_enumerable_field_edit_handler(|row: &mut Item| &mut row.beta)
                    .with_sort_handler(|a: &Item, b: &Item| a.beta.cmp(&b.beta))
                    .with_cell_builder(|row: &Item, _, _| row.beta.to_string().into_any_element()),
                Column::new("gamma", "Gamma")
                    .with_dropdown_edit_handler(|row: &mut Item| &mut row.gamma)
                    .with_sort_handler(|a: &Item, b: &Item| {
                        a.gamma.partial_cmp(&b.gamma).unwrap_or(std::cmp::Ordering::Equal)
                    })
                    .with_cell_builder(|row: &Item, _, _| row.gamma.to_string().into_any_element()),
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

    fn rows(&self) -> impl Iterator<Item = (&Self::RowId, &Self::Row)> {
        self.items.iter()
    }

    fn row(&self, row_id: &Self::RowId) -> Option<&Self::Row> {
        self.items.get(row_id)
    }

    fn row_mut(&mut self, row_id: &Self::RowId) -> Option<&mut Self::Row> {
        self.items.get_mut(row_id)
    }
}

#[derive(Debug)]
struct Item {
    alpha: u32,
    beta: String,
    gamma: Protocol,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Protocol {
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

impl DropdownItem for Protocol {
    fn label(&self) -> String {
        match self {
            Protocol::Artnet => "Art-Net".to_string(),
            Protocol::Sacn => "sACN".to_string(),
            Protocol::PosiStageNet => "PosiStageNet".to_string(),
            Protocol::Dmx512 => "DMX512".to_string(),
        }
    }

    fn variants() -> Vec<Self> {
        vec![Protocol::Artnet, Protocol::Sacn, Protocol::PosiStageNet, Protocol::Dmx512]
    }
}
