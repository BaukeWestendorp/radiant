use gpui::prelude::*;
use gpui::{App, Entity, EventEmitter, Window, div, px};
use radiant::builtin::GdtfFixtureTypeId;
use ui::interactive::event::SubmitEvent;
use ui::interactive::table::{Column, Table, TableDelegate};
use ui::nav::tabs::{Tab, Tabs};
use ui::theme::ActiveTheme;

use crate::engine::EngineManager;

pub struct FixtureTypePicker {
    tabs: Entity<Tabs>,
}

impl FixtureTypePicker {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let picker = cx.entity();
        let from_showfile = cx.new(|cx| FromShowfileTab::new(picker, window, cx));
        let from_library = cx.new(|_| FromLibraryTab {});

        let tabs = cx.new(|cx| {
            Tabs::new(
                vec![
                    Tab::new("from_showfile", "From Showfile", from_showfile.into()),
                    Tab::new("from_library", "From Library", from_library.into()),
                ],
                window,
                cx,
            )
        });

        Self { tabs }
    }
}

impl Render for FixtureTypePicker {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().bg(cx.theme().background).size_full().child(self.tabs.clone())
    }
}

impl EventEmitter<SubmitEvent<GdtfFixtureTypeId>> for FixtureTypePicker {}

struct FromShowfileTab {
    table: Entity<Table<FixtureTypeTable>>,
}

impl FromShowfileTab {
    pub fn new(
        picker: Entity<FixtureTypePicker>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let ft_ids =
            EngineManager::read_patch(cx, |patch| patch.fixture_types().keys().cloned().collect());
        let table = cx.new(|cx| Table::new(FixtureTypeTable::new(ft_ids), window, cx));

        Self { table }
    }
}

impl Render for FromShowfileTab {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(self.table.clone())
    }
}

struct FromLibraryTab {}

impl Render for FromLibraryTab {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().child("TODO")
    }
}

struct FixtureTypeTable {
    columns: Vec<Column>,
    ft_ids: Vec<GdtfFixtureTypeId>,
}

impl FixtureTypeTable {
    pub fn new(ft_ids: Vec<GdtfFixtureTypeId>) -> Self {
        Self {
            columns: vec![
                Column::new("manufacturer", "Manufacturer").with_width(px(300.0)),
                Column::new("name", "Name").with_width(px(300.0)),
                Column::new("ft_id", "Fixture Type Id").with_width(px(300.0)),
            ],
            ft_ids,
        }
    }
}

impl TableDelegate for FixtureTypeTable {
    type RowId = GdtfFixtureTypeId;

    fn column_count(&self, _cx: &App) -> usize {
        self.columns.len()
    }

    fn column(&self, col_ix: usize, _cx: &App) -> &Column {
        &self.columns[col_ix]
    }

    fn column_ix(&self, column_id: &str, _cx: &App) -> usize {
        self.columns.iter().position(|column| column.id == column_id).unwrap()
    }

    fn sorted_row_ids(&self, _cx: &App) -> Vec<Self::RowId> {
        let mut ft_ids = self.ft_ids.clone();
        ft_ids.sort();
        ft_ids
    }

    fn render_cell(
        &self,
        row_id: &Self::RowId,
        col_ix: usize,
        window: &mut Window,
        cx: &mut Context<Table<Self>>,
    ) -> impl IntoElement {
        let column = self.column(col_ix, cx);
        let ft = EngineManager::read_patch(cx, |patch| patch.fixture_type(row_id).unwrap().clone());

        let render_cell = |content| {
            div().size_full().flex().items_center().px_1().child(content).into_any_element()
        };

        match column.id.as_str() {
            "manufacturer" => render_cell(ft.manufacturer.clone()).into_any_element(),
            "name" => render_cell(ft.long_name.clone()).into_any_element(),
            "ft_id" => render_cell(row_id.to_string()).into_any_element(),
            _ => self.render_empty(window, cx).into_any_element(),
        }
    }
}
