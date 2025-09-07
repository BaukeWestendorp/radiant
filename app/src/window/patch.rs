use gpui::prelude::*;
use gpui::{App, Entity, ReadGlobal, Window, WindowHandle, div};
use radiant::builtin::Patch;
use ui::interactive::table::{Column, Selection, Table, TableDelegate};

use crate::engine::EngineManager;

pub struct PatchWindow {
    table: Entity<Table<PatchTable>>,
}

impl PatchWindow {
    pub fn open(cx: &mut App) -> WindowHandle<Self> {
        cx.open_window(super::window_options(), |window, cx| cx.new(|cx| Self::new(window, cx)))
            .expect("should open patch window")
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let patch =
            cx.new(|cx| EngineManager::global(cx).engine.patch().read(|patch| patch.clone()));
        Self {
            table: cx.new(|cx| {
                let mut table = Table::new(PatchTable::new(patch), window, cx);
                table.start_selection("name", 2, cx);
                table.end_selection(5, cx);
                table
            }),
        }
    }
}

impl Render for PatchWindow {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        super::window_root(window, cx)
            .child(div().size_full().id("patch_window_root").child(self.table.clone()))
    }
}

struct PatchTable {
    patch: Entity<Patch>,
    columns: Vec<Column>,
}

impl PatchTable {
    pub fn new(patch: Entity<Patch>) -> Self {
        Self {
            patch,
            columns: vec![
                Column::new("fid", "Fixture Id"),
                Column::new("name", "Name"),
                Column::new("addr", "Address"),
                Column::new("ft_id", "Fixture Type"),
                Column::new("dmx_mode", "Dmx Mode"),
            ],
        }
    }
}

impl TableDelegate for PatchTable {
    fn column_count(&self, _cx: &App) -> usize {
        self.columns.len()
    }

    fn row_count(&self, cx: &App) -> usize {
        self.patch.read(cx).fixtures().len()
    }

    fn column(&self, col_ix: usize, _cx: &App) -> &Column {
        &self.columns[col_ix]
    }

    fn render_cell(
        &self,
        row_ix: usize,
        col_ix: usize,
        window: &mut Window,
        cx: &mut Context<Table<Self>>,
    ) -> impl IntoElement {
        let column = self.column(col_ix, cx);
        let Some(fixture) = self.patch.read(cx).fixtures().get(row_ix) else {
            return self.render_empty(window, cx).into_any_element();
        };

        match column.id.to_string().as_str() {
            "fid" => {
                return div()
                    .size_full()
                    .flex()
                    .items_center()
                    .px_1()
                    .child(fixture.fid.to_string())
                    .into_any_element();
            }
            "name" => {
                return div()
                    .size_full()
                    .flex()
                    .items_center()
                    .px_1()
                    .child(fixture.name.to_string())
                    .into_any_element();
            }
            "addr" => {
                return div()
                    .size_full()
                    .flex()
                    .items_center()
                    .px_1()
                    .child(fixture.address.to_string())
                    .into_any_element();
            }
            "ft_id" => {
                return div()
                    .size_full()
                    .flex()
                    .items_center()
                    .px_1()
                    .child(fixture.fixture_type_id.to_string())
                    .into_any_element();
            }
            "dmx_mode" => {
                return div()
                    .size_full()
                    .flex()
                    .items_center()
                    .px_1()
                    .child(fixture.dmx_mode.to_string())
                    .into_any_element();
            }
            _ => self.render_empty(window, cx).into_any_element(),
        }
    }

    fn edit_selection(&mut self, selection: Selection, _cx: &mut App) {
        match selection.column_id.as_str() {
            "fid" => {}
            "name" => {}
            "addr" => {}
            "ft_id" => {}
            "dmx_mode" => {}
            _ => {}
        }
    }
}
