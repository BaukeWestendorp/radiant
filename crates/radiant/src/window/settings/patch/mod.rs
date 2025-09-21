use gpui::prelude::*;
use gpui::{Entity, Window, div};
use nui::button::button;
use nui::table::{Table, TableDelegate};
use nui::theme::ActiveTheme;

use patch_table::PatchTable;

mod ft_picker;
mod patch_table;

pub struct PatchSettings {
    table: Entity<Table<PatchTable>>,
}

impl PatchSettings {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self { table: cx.new(|cx| Table::new(PatchTable::new(window, cx), window, cx)) }
    }
}

impl Render for PatchSettings {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let table = div().size_full().child(self.table.clone());

        let selected_rows = self.table.read(cx).selected_row_ids(cx).len();
        let table_valid = self.table.read(cx).validate(cx);
        let save_button = button("save_patch", None, "Save Patch")
            .disabled(!table_valid)
            .on_click(|_, window, _| window.remove_window());
        let info_bar = div()
            .flex()
            .justify_between()
            .items_center()
            .w_full()
            .h_10()
            .px_2()
            .border_t_1()
            .border_color(cx.theme().border)
            .bg(cx.theme().background)
            .child(if selected_rows > 0 {
                format!("Selected rows: {}", selected_rows)
            } else {
                "".to_string()
            })
            .child(save_button);

        div().flex().flex_col().size_full().overflow_hidden().child(table).child(info_bar)
    }
}
