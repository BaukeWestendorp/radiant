use rd_ui::{
    Emphasis, StyledExt,
    comp::stateful::{Table, TableColumn, TableSelection, TableSelectionMode},
    gpui::{Entity, Window, div, prelude::*},
    h_flex,
};

pub struct TablePreview {
    table: Entity<Table<Row>>,
}

impl TablePreview {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let rows = cx.new(|_| {
            vec![
                Row { alpha: 1.0, beta: "Row 1".to_string(), gamma: true },
                Row { alpha: 2.0, beta: "Row 2".to_string(), gamma: false },
                Row { alpha: 3.0, beta: "Row 3".to_string(), gamma: true },
                Row { alpha: 4.0, beta: "Row 4".to_string(), gamma: false },
            ]
        });

        Self {
            table: cx.new(|cx| {
                Table::new("table", rows, window, cx)
                    .with_columns(
                        vec![
                            TableColumn::new("Alpha").with_element(|row: &Row, _, _| {
                                h_flex().child(row.alpha.to_string()).into_any_element()
                            }),
                            TableColumn::new("Beta").with_element(|row: &Row, _, _| {
                                h_flex().child(row.beta.clone()).into_any_element()
                            }),
                            TableColumn::new("Gamma").with_element(|row: &Row, _, _| {
                                h_flex().child(row.gamma.to_string()).into_any_element()
                            }),
                        ],
                        cx,
                    )
                    .with_selection(TableSelection::new(TableSelectionMode::Multiple), cx)
                    .with_on_delete(cx, |table, _, cx| {
                        let row_ixs = table.selection(cx).rows();
                        log::info!("Delete rows: {:?}", row_ixs);
                    })
                    .with_on_edit(cx, |table, _, _, cx| {
                        let row_ixs = table.selection(cx).rows();
                        log::info!("Edit rows: {:?}", row_ixs);
                    })
            }),
        }
    }
}

impl Render for TablePreview {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().p_2().child(
            div().size_full().emphasis_bordered(Emphasis::Secondary, cx).child(self.table.clone()),
        )
    }
}

struct Row {
    pub alpha: f32,
    pub beta: String,
    pub gamma: bool,
}
