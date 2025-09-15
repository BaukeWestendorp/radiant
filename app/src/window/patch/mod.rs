use gpui::prelude::*;
use gpui::{App, Entity, Window, WindowHandle, div};
use ui::Disableable;
use ui::interactive::button::button;
use ui::interactive::table::{Table, TableDelegate};
use ui::theme::ActiveTheme;

use crate::window::patch::patch_table::PatchTable;

mod ft_picker;
mod patch_table;

pub struct PatchWindow {
    table: Entity<Table<PatchTable>>,
}

impl PatchWindow {
    pub fn open(cx: &mut App) -> WindowHandle<Self> {
        cx.open_window(super::window_options(), |window, cx| cx.new(|cx| Self::new(window, cx)))
            .expect("should open patch window")
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self { table: cx.new(|cx| Table::new(PatchTable::new(window, cx), window, cx)) }
    }
}

impl Render for PatchWindow {
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

        super::window_root().child(div().size_full().child(self.table.clone())).child(info_bar)
    }
}
