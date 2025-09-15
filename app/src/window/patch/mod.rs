use gpui::prelude::*;
use gpui::{App, Entity, Window, WindowHandle, div};
use ui::interactive::table::Table;

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
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        super::window_root().child(div().size_full().p_2().child(self.table.clone()))
    }
}
