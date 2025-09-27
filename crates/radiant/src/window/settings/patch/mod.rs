use gpui::prelude::*;
use gpui::{Entity, Window, div};
use nui::button::button;
use nui::container::container;
use nui::input::TextField;
use nui::section::section;
use nui::table::Table;
use nui::theme::ActiveTheme;

use fixture_table::FixtureTable;
use nui::AppExt;
use nui::wm::Overlay;

use ft_picker::FixtureTypePicker;

mod fixture_table;
mod ft_picker;

pub struct PatchSettings {
    table: Entity<Table<FixtureTable>>,
}

impl PatchSettings {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self { table: cx.new(|cx| Table::new(FixtureTable::new(window, cx), window, cx)) }
    }

    fn open_add_fixtures_overlay(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let overlay = Overlay::new(
            "add_fixtures",
            "Add Fixtures",
            cx.new(|cx| AddFixtureOverlay::new(window, cx)),
            cx.focus_handle(),
        );

        cx.update_wm(|wm, cx| wm.open_overlay(overlay, window, cx));
    }
}

impl Render for PatchSettings {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let table = div().size_full().child(self.table.clone());

        let selected_rows = self.table.read(cx).selected_row_ids(cx).len();

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
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(
                        button("delete", None, "Delete Fixtures")
                            .disabled(selected_rows == 0)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.table
                                    .update(cx, |table, cx| table.delete_selection(window, cx))
                            })),
                    )
                    .child(button("add", None, "Add Fixtures").on_click(cx.listener(
                        |this, _, window, cx| this.open_add_fixtures_overlay(window, cx),
                    ))),
            );

        div().flex().flex_col().size_full().overflow_hidden().child(table).child(info_bar)
    }
}

struct AddFixtureOverlay {
    ft_picker: Entity<FixtureTypePicker>,
    fid_field: Entity<TextField>,
    name_field: Entity<TextField>,
    addr_field: Entity<TextField>,
    count_field: Entity<TextField>,
}

impl AddFixtureOverlay {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            ft_picker: cx.new(|cx| FixtureTypePicker::new(window, cx)),
            fid_field: cx.new(|cx| TextField::new("fid_field", cx.focus_handle(), window, cx)),
            name_field: cx.new(|cx| TextField::new("name_field", cx.focus_handle(), window, cx)),
            addr_field: cx.new(|cx| TextField::new("addr_field", cx.focus_handle(), window, cx)),
            count_field: cx.new(|cx| TextField::new("count_field", cx.focus_handle(), window, cx)),
        }
    }
}

impl Render for AddFixtureOverlay {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        window.focus_next();
        div()
            .size_full()
            .flex()
            .flex_col()
            .gap_2()
            .p_2()
            .child(container(window, cx).size_full().p_2().child(self.ft_picker.clone()))
            .child(section("Fixture Id").child(self.fid_field.clone()))
            .child(section("Name").child(self.name_field.clone()))
            .child(section("Address").child(self.addr_field.clone()))
            .child(section("Count").child(self.count_field.clone()))
    }
}
