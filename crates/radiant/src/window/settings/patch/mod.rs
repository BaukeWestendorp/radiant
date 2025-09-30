use gpui::prelude::*;
use gpui::{App, ClickEvent, Entity, SharedString, Window, div};
use nui::AppExt;
use nui::button::button;
use nui::container::container;
use nui::infobar::infobar;
use nui::input::{NumberField, TextField};
use nui::section::section;
use nui::table::Table;
use nui::theme::ActiveTheme;
use nui::wm::Overlay;
use radlib::builtin::{FixtureId, GdtfFixtureTypeId};
use radlib::cmd::{Command, PatchCommand};
use uuid::Uuid;

use std::num::NonZeroU32;

use crate::engine::EngineManager;
use crate::ui::fields::{address_field, fid_field, uint_field};
use crate::window::settings::patch::fixture_table::FixtureTable;
use crate::window::settings::patch::ft_picker::FixtureTypePicker;

mod fixture_table;
mod ft_picker;

const ADD_FIXTURES_OVERLAY_ID: &str = "add_fixtures";

pub struct PatchSettings {
    table: Entity<Table<FixtureTable>>,
}

impl PatchSettings {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self { table: cx.new(|cx| Table::new(FixtureTable::new(window, cx), window, cx)) }
    }

    fn open_add_fixtures_overlay(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let overlay = Overlay::new(
            ADD_FIXTURES_OVERLAY_ID,
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
                format!("Selected rows: {selected_rows}")
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
    fid_field: Entity<NumberField>,
    name_field: Entity<TextField>,
    start_addr_field: Entity<TextField>,
    count_field: Entity<NumberField>,
}

impl AddFixtureOverlay {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let address = EngineManager::read_patch(cx, |patch| patch.first_unbounded_address());
        Self {
            ft_picker: cx.new(|cx| FixtureTypePicker::new(window, cx)),
            fid_field: cx.new(|cx| fid_field(None, window, cx)),
            name_field: cx.new(|cx| TextField::new("name_field", cx.focus_handle(), window, cx)),
            start_addr_field: cx.new(|cx| address_field(Some(address), window, cx)),
            count_field: cx.new(|cx| uint_field(Some(1), window, cx).with_min(Some(1.0), cx)),
        }
    }

    fn ft_id(&self, cx: &App) -> Option<GdtfFixtureTypeId> {
        self.ft_picker.read(cx).selected_ft_id(cx)
    }

    fn dmx_mode(&self, cx: &App) -> Option<String> {
        self.ft_picker.read(cx).selected_dmx_mode(cx)
    }

    fn fid(&self, cx: &App) -> Option<FixtureId> {
        let u32_value = self.fid_field.read(cx).value(cx)? as u32;
        Some(FixtureId(NonZeroU32::new(u32_value)?))
    }

    fn name(&self, cx: &App) -> SharedString {
        self.name_field.read(cx).value(cx).trim().to_string().into()
    }

    fn address(&self, cx: &App) -> Option<dmx::Address> {
        let addr_str = self.start_addr_field.read(cx).value(cx);
        addr_str.to_string().parse().ok()
    }

    fn count(&self, cx: &App) -> u32 {
        self.count_field.read(cx).value(cx).unwrap_or_default() as u32
    }

    fn validate(&self, cx: &App) -> bool {
        self.ft_id(cx).is_some()
            && self.dmx_mode(cx).is_some()
            && self.fid(cx).is_some()
            && !self.name(cx).is_empty()
            && self.address(cx).is_some()
            && self.count(cx) > 0
    }

    fn add_fixtures(&mut self, _: &ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
        if !self.validate(cx) {
            return;
        }

        let fixture_type_id = self.ft_id(cx).unwrap();
        let dmx_mode = self.dmx_mode(cx).unwrap();

        let (fids, names) = (
            generate_fids(self.fid(cx).unwrap(), self.count(cx) as usize),
            generate_names(&self.name(cx), self.count(cx) as usize),
        );

        let mut fixture_uuids = Vec::new();
        for (fid, name) in fids.iter().zip(names) {
            EngineManager::exec_and_log_err(
                Command::Patch(PatchCommand::AddFixture {
                    fid: Some(*fid),
                    fixture_type_id,
                    address: None,
                    dmx_mode: dmx_mode.clone(),
                    name: Some(name.clone()),
                }),
                cx,
            );

            if let Some(uuid) =
                EngineManager::read_patch(cx, |patch| patch.fixture(*fid).map(|f| f.uuid()))
            {
                fixture_uuids.push(uuid);
            }
        }

        let addresses = generate_addresses(self.address(cx).unwrap(), &fixture_uuids, cx);
        for (uuid, address) in fixture_uuids.into_iter().zip(addresses) {
            EngineManager::exec_and_log_err(
                Command::Patch(PatchCommand::SetAddress {
                    fixture_ref: uuid.into(),
                    address: Some(address),
                }),
                cx,
            );
        }

        cx.update_wm(|wm, _| wm.close_overlay(ADD_FIXTURES_OVERLAY_ID, window));
    }
}

impl Render for AddFixtureOverlay {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .child(
                div()
                    .size_full()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .p_2()
                    .child(container(window, cx).size_full().p_2().child(self.ft_picker.clone()))
                    .child(section("Fixture Id").max_w_40().child(self.fid_field.clone()))
                    .child(section("Name").max_w_40().child(self.name_field.clone()))
                    .child(section("Start Address").max_w_40().child(self.start_addr_field.clone()))
                    .child(section("Count").max_w_40().child(self.count_field.clone())),
            )
            .child(
                infobar(cx).justify_end().child(
                    button(
                        "Add Fixture",
                        None,
                        format!(
                            "Add {} Fixture{}",
                            self.count(cx),
                            if self.count(cx) > 1 { "s" } else { "" }
                        ),
                    )
                    .disabled(!self.validate(cx))
                    .on_click(cx.listener(Self::add_fixtures)),
                ),
            )
    }
}

fn generate_fids(start_fid: FixtureId, n: usize) -> Vec<FixtureId> {
    let mut new_fids = Vec::new();
    for i in 0..n as u32 {
        let new_fid = FixtureId(NonZeroU32::new(u32::from(start_fid.0) + i).unwrap());
        new_fids.push(new_fid);
    }
    new_fids
}

fn generate_names(base_name: &str, n: usize) -> Vec<String> {
    use regex::Regex;
    let re = Regex::new(r"^(.*?)(\d+)$").unwrap();
    if let Some(caps) = re.captures(base_name) {
        let base = caps.get(1).map_or("", |m| m.as_str()).trim_end();
        let num_str = caps.get(2).map_or("0", |m| m.as_str());
        let start_num: usize = num_str.parse().unwrap_or(0);
        let pad_len = num_str.len();
        (0..n)
            .map(|offset| format!("{} {:0pad$}", base, start_num + offset, pad = pad_len))
            .collect()
    } else {
        (0..n).map(|_| base_name.to_string()).collect()
    }
}

fn generate_addresses(
    start_address: dmx::Address,
    fixture_uuids: &[Uuid],
    cx: &App,
) -> Vec<dmx::Address> {
    fixture_uuids
        .iter()
        .map(|uuid| {
            EngineManager::read_patch(cx, |patch| {
                radlib::gdtf::channel_count(patch.fixture(*uuid).unwrap().dmx_mode(patch))
            })
        })
        .scan(0, |state, channel_count| {
            let offset = *state;
            *state += channel_count;
            Some(offset)
        })
        .map(|offset| start_address.with_channel_offset(offset))
        .collect()
}
