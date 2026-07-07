use std::{collections::HashMap, sync::Arc};

use gpui::{App, ElementId, Entity, FocusHandle, RenderOnce, Window, div, prelude::*, px};
use rd_engine::{
    gdtf::{FixtureTypeId, Gdtf, Name},
    patch::FixtureKind,
};
use rd_ui::{
    ActiveTheme, Button, Column, Popup, PopupAppExt, Table, TableDelegate, TableSelection,
    TableState, h_flex, interactive_container, v_flex,
};

use crate::engine::EngineAppExt;

pub struct FixtureKindPickerState {
    id: ElementId,
    focus_handle: FocusHandle,

    disabled: bool,

    fixture_type_id: Option<FixtureTypeId>,
    dmx_mode: Option<String>,
}

impl FixtureKindPickerState {
    pub fn new(
        id: impl Into<ElementId>,
        focus_handle: FocusHandle,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Self {
        Self {
            id: id.into(),
            focus_handle: focus_handle.tab_stop(true),

            disabled: false,

            fixture_type_id: None,
            dmx_mode: None,
        }
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn set_disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.set_disabled(disabled);
        self
    }

    pub fn fixture_type_id(&self) -> Option<FixtureTypeId> {
        self.fixture_type_id
    }

    pub fn dmx_mode(&self) -> Option<&str> {
        self.dmx_mode.as_deref()
    }

    pub fn fixture_kind(&self) -> Option<FixtureKind> {
        Some(FixtureKind::new(self.fixture_type_id?, self.dmx_mode.as_ref()?.to_string()))
    }
}

#[derive(IntoElement)]
pub struct FixtureKindPicker {
    state: Entity<FixtureKindPickerState>,
}

impl FixtureKindPicker {
    pub fn new(state: Entity<FixtureKindPickerState>) -> Self {
        Self { state }
    }
}

impl RenderOnce for FixtureKindPicker {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state_entity = self.state.clone();
        let state = self.state.read(cx);
        let id = state.id.clone();
        let focus_handle = state.focus_handle.clone();
        let disabled = state.disabled();
        let fixture_kind = state.fixture_kind();

        let preview = fixture_kind
            .map(|fk| div().child(fk.display(&cx.engine_snapshot().patch()).unwrap_or_default()))
            .unwrap_or(div().text_color(cx.theme().fg_secondary).child("Select a fixture kind..."));

        interactive_container(id, Some(focus_handle))
            .relative()
            .w_full()
            .disabled(disabled)
            .child(div().size_full().px_1().py_0p5().child(preview))
            .on_click(move |_, window, cx| {
                let state_entity = state_entity.clone();
                cx.open_popup(window, move |window, cx| {
                    Popup::custom(
                        cx.new(|cx| FixtureKindPickerPopup::new(state_entity, window, cx)),
                        "Select a fixture kind",
                    )
                })
            })
    }
}

struct FixtureKindPickerPopup {
    picker: Entity<FixtureKindPickerState>,

    fixture_type_id_table: Entity<TableState<GdtfTable>>,
    dmx_mode_table: Entity<TableState<DmxModeTable>>,
}

impl FixtureKindPickerPopup {
    pub fn new(
        picker: Entity<FixtureKindPickerState>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let fixture_type_id_table = cx.new(|_| TableSelection::Single(None));
        let dmx_mode_selection = cx.new(|_| TableSelection::Single(None));

        let gdtf_table = cx.new(|cx| {
            let gdtfs = cx.engine_snapshot().patch().gdtfs().clone();
            TableState::new(GdtfTable::new(gdtfs), fixture_type_id_table.clone(), window, cx)
        });

        let dmx_mode_table = cx.new(|cx| {
            TableState::new(
                DmxModeTable::new(fixture_type_id_table.clone()),
                dmx_mode_selection.clone(),
                window,
                cx,
            )
        });

        cx.observe(&fixture_type_id_table, |this, fixture_type_id_table, cx| {
            this.dmx_mode_table.update(cx, |table, cx| {
                table.reload(cx);
            });

            this.picker.update(cx, |picker, cx| {
                picker.fixture_type_id = fixture_type_id_table.read(cx).single().cloned();
                picker.dmx_mode = None;
                cx.notify();
            })
        })
        .detach();

        cx.observe(&dmx_mode_selection, |this, dmx_mode_selection, cx| {
            this.picker.update(cx, |picker, cx| {
                picker.dmx_mode = dmx_mode_selection.read(cx).single().map(|dm| dm.to_string());
                cx.notify();
            })
        })
        .detach();

        Self { picker, fixture_type_id_table: gdtf_table, dmx_mode_table }
    }
}

impl Render for FixtureKindPickerPopup {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .p_2()
            .gap_2()
            .child(
                div().text_color(cx.theme().fg_secondary).child("FIXME: This picker still sucks"),
            )
            .child(
                h_flex()
                    .size_full()
                    .gap_2()
                    .child(
                        div()
                            .size_full()
                            .border_1()
                            .border_color(cx.theme().border_primary)
                            .child(Table::new(self.fixture_type_id_table.clone())),
                    )
                    .child(
                        div()
                            .size_full()
                            .border_1()
                            .border_color(cx.theme().border_primary)
                            .child(Table::new(self.dmx_mode_table.clone())),
                    ),
            )
            .child(
                Button::new("submit")
                    .disabled(!self.dmx_mode_table.read(cx).has_selection(cx))
                    .child("Select")
                    .on_click(|_, window, cx| cx.close_popup(window)),
            )
    }
}

struct GdtfTable {
    gdtfs: HashMap<FixtureTypeId, Arc<Gdtf>>,
    columns: Vec<Column>,
}

impl GdtfTable {
    pub fn new(gdtfs: HashMap<FixtureTypeId, Arc<Gdtf>>) -> Self {
        Self {
            gdtfs,
            columns: vec![
                Column::new("manufacturer", "Manufacturer").with_min_width(px(200.0)),
                Column::new("name", "Name").with_min_width(px(200.0)),
            ],
        }
    }
}

impl TableDelegate for GdtfTable {
    type RowId = FixtureTypeId;

    fn column_count(&self, _cx: &App) -> usize {
        self.columns.len()
    }

    fn column(&self, col_ix: usize, _cx: &App) -> &Column {
        &self.columns[col_ix]
    }

    fn root_row_ids(&self, _cx: &App) -> Vec<Self::RowId> {
        self.gdtfs.keys().copied().collect()
    }

    fn render_cell(
        &self,
        row_id: &Self::RowId,
        col_ix: usize,
        _window: &mut Window,
        _cx: &App,
    ) -> impl IntoElement {
        let Some(gdtf) = self.gdtfs.get(row_id) else { return div() };
        let col = &self.columns[col_ix];
        let content = match col.id().as_ref() {
            "manufacturer" => gdtf.manufacturer().to_string(),
            "name" => gdtf.name().to_string(),
            _ => "".to_string(),
        };
        div().mx_1().child(content)
    }
}

struct DmxModeTable {
    fixture_type_id: Entity<TableSelection<FixtureTypeId>>,
    columns: Vec<Column>,
}

impl DmxModeTable {
    pub fn new(fixture_type_id: Entity<TableSelection<FixtureTypeId>>) -> Self {
        Self {
            fixture_type_id,
            columns: vec![
                Column::new("name", "Name").with_min_width(px(250.0)),
                Column::new("channel_count", "Channels").with_min_width(px(75.0)),
            ],
        }
    }
}

impl TableDelegate for DmxModeTable {
    type RowId = Name;

    fn column_count(&self, _cx: &App) -> usize {
        self.columns.len()
    }

    fn column(&self, col_ix: usize, _cx: &App) -> &Column {
        &self.columns[col_ix]
    }

    fn root_row_ids(&self, cx: &App) -> Vec<Self::RowId> {
        self.fixture_type_id
            .read(cx)
            .single()
            .and_then(|ftid| cx.engine_snapshot().patch().gdtfs().get(&ftid).cloned())
            .map(|gdtf| gdtf.dmx_modes().iter().map(|dm| dm.name().clone()).collect())
            .unwrap_or_default()
    }

    fn render_cell(
        &self,
        row_id: &Self::RowId,
        col_ix: usize,
        _window: &mut Window,
        cx: &App,
    ) -> impl IntoElement {
        let Some((name, channel_count)) = self.fixture_type_id.read(cx).single().and_then(|ftid| {
            let patch = cx.engine_snapshot().patch();
            let dmx_mode = patch
                .gdtfs()
                .get(&ftid)
                .and_then(|gdtf| gdtf.dmx_modes().iter().find(|dm| dm.name() == row_id))?;
            Some((dmx_mode.name().to_string(), dmx_mode.max_channel_offset()))
        }) else {
            return div();
        };

        let col = &self.columns[col_ix];
        let content = match col.id().as_ref() {
            "name" => name,
            "channel_count" => channel_count.to_string(),
            _ => "".to_string(),
        };
        div().mx_1().child(content)
    }
}
