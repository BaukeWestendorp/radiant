use gpui::prelude::*;
use gpui::{App, Entity, EventEmitter, SharedString, Window, div, px};
use nui::event::SubmitEvent;
use nui::section::section;
use nui::table::{Column, Table, TableDelegate, TableEvent};
use radlib::builtin::GdtfFixtureTypeId;
use radlib::gdtf::fixture_type::FixtureType;

use crate::engine::EngineManager;

pub struct FixtureTypePicker {
    ft_table: Entity<Table<FixtureTypeTable>>,
    dmx_mode_table: Option<Entity<Table<DmxModeTable>>>,
}

impl FixtureTypePicker {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let ft_ids =
            EngineManager::read_patch(cx, |patch| patch.fixture_types().keys().cloned().collect());
        let ft_table = cx.new(|cx| Table::new(FixtureTypeTable::new(ft_ids), window, cx));

        cx.subscribe_in(
            &ft_table,
            window,
            |this, table, event: &TableEvent, window, cx| match event {
                TableEvent::SelectionChanged => {
                    if let Some(ft_id) = table.read(cx).selected_row_ids(cx).get(0) {
                        this.open_dmx_mode_table(ft_id, window, cx);
                    } else {
                        this.close_dmx_mode_table(cx);
                    }
                }
            },
        )
        .detach();

        Self { ft_table, dmx_mode_table: None }
    }

    pub fn with_selected(
        mut self,
        ft_id: GdtfFixtureTypeId,
        dmx_mode: impl Into<SharedString>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        self.select_ft_id(&ft_id, window, cx);
        self.select_dmx_mode(dmx_mode.into(), cx);
        self
    }

    pub fn selected_ft_id(&self, cx: &App) -> Option<GdtfFixtureTypeId> {
        self.ft_table.read(cx).selected_row_ids(cx).get(0).copied()
    }

    pub fn select_ft_id(
        &mut self,
        ft_id: &GdtfFixtureTypeId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.ft_table.update(cx, |ft_table, cx| ft_table.select_row_id(&ft_id, cx));
        self.open_dmx_mode_table(ft_id, window, cx);
    }

    pub fn selected_dmx_mode(&self, cx: &App) -> Option<String> {
        self.dmx_mode_table.as_ref()?.read(cx).selected_row_ids(cx).get(0).cloned()
    }

    pub fn select_dmx_mode(&mut self, dmx_mode: SharedString, cx: &mut Context<Self>) {
        let Some(dmx_mode_table) = &self.dmx_mode_table else {
            return;
        };

        dmx_mode_table.update(cx, |dmx_mode_table, cx| {
            dmx_mode_table.select_row_id(&dmx_mode.into(), cx);
        });
    }

    fn open_dmx_mode_table(
        &mut self,
        ft_id: &GdtfFixtureTypeId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let table = cx.new(|cx| Table::new(DmxModeTable::new(ft_id, cx), window, cx));
        self.dmx_mode_table = Some(table);
        cx.notify();
    }

    fn close_dmx_mode_table(&mut self, cx: &mut Context<Self>) {
        self.dmx_mode_table = None;
        cx.notify();
    }
}

impl Render for FixtureTypePicker {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .gap_2()
            .child(section("Fixture Type").w_1_2().h_full().child(self.ft_table.clone()))
            .child(section("DMX Mode").w_1_2().h_full().children(self.dmx_mode_table.clone()))
    }
}

impl EventEmitter<SubmitEvent<(GdtfFixtureTypeId, String)>> for FixtureTypePicker {}

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

    fn can_select_multiple_rows(&self, _cx: &App) -> bool {
        false
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
            _ => self.render_empty(window, cx).into_any_element(),
        }
    }
}

struct DmxModeTable {
    columns: Vec<Column>,
    ft: FixtureType,
}

impl DmxModeTable {
    pub fn new(ft_id: &GdtfFixtureTypeId, cx: &mut Context<Table<Self>>) -> Self {
        let ft = EngineManager::read_patch(cx, |patch| patch.fixture_type(ft_id).unwrap().clone());
        Self {
            columns: vec![
                Column::new("name", "Name").with_width(px(200.0)),
                Column::new("channel_count", "Channel Count").with_width(px(200.0)),
            ],
            ft,
        }
    }
}

impl TableDelegate for DmxModeTable {
    type RowId = String;

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
        let mut dmx_modes = self
            .ft
            .dmx_modes
            .iter()
            .map(|dmx_mode| {
                dmx_mode
                    .name
                    .as_ref()
                    .map(|name| name.to_string())
                    .unwrap_or("<unknown>".to_string())
            })
            .collect::<Vec<_>>();
        dmx_modes.sort_by(|a, b| a.cmp(&b));
        dmx_modes
    }

    fn can_select_multiple_rows(&self, _cx: &App) -> bool {
        false
    }

    fn render_cell(
        &self,
        row_id: &Self::RowId,
        col_ix: usize,
        window: &mut Window,
        cx: &mut Context<Table<Self>>,
    ) -> impl IntoElement {
        let column = self.column(col_ix, cx);

        let render_cell = |content| {
            div().size_full().flex().items_center().px_1().child(content).into_any_element()
        };

        let dmx_mode = self.ft.dmx_mode(row_id).unwrap();

        match column.id.as_str() {
            "name" => {
                let name = dmx_mode
                    .name
                    .as_ref()
                    .map(|name| name.to_string())
                    .unwrap_or("<unknown>".to_string());

                render_cell(name).into_any_element()
            }
            "channel_count" => {
                let channel_count = radlib::gdtf::channel_count(dmx_mode);

                render_cell(channel_count.to_string()).into_any_element()
            }
            _ => self.render_empty(window, cx).into_any_element(),
        }
    }
}
