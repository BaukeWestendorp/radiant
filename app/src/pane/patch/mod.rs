use gpui::prelude::*;
use gpui::{Entity, Window, div};
use ui::Disableable;
use ui::interactive::button::button;
use ui::interactive::table::{Table, TableDelegate};
use ui::theme::ActiveTheme;

use patch_table::PatchTable;

use crate::window::main::MainWindow;

mod ft_picker;
mod patch_table;

pub struct PatchSettings {
    table: Entity<Table<PatchTable>>,
}

impl PatchSettings {
    pub fn new(
        main_window: Entity<MainWindow>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            table: cx.new(|cx| Table::new(PatchTable::new(main_window, window, cx), window, cx)),
        }
    }
}

impl Render for PatchSettings {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let selected_rows = self.table.read(cx).selected_row_ids(cx).len();
        let table_valid = self.table.read(cx).validate(cx);
        let info_bar = div()
            .w_full()
            .h_10()
            .flex()
            .px_2()
            .justify_between()
            .items_center()
            .border_t_1()
            .border_color(cx.theme().border)
            .child(if selected_rows > 0 {
                format!("Selected rows: {}", selected_rows)
            } else {
                "".to_string()
            })
            .child(button("save_patch", None, "Save Patch").disabled(!table_valid));

        div().size_full().child(self.table.clone()).child(info_bar)
    }
}
