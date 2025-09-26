use gpui::prelude::*;
use gpui::{App, Entity, IntoElement, Window, div, px};
use nui::AppExt;
use nui::event::SubmitEvent;
use nui::table::{Column, Table, TableDelegate};
use nui::theme::{ActiveTheme, InteractiveColor};
use nui::wm::Overlay;
use radlib::builtin::{FixtureId, GdtfFixtureTypeId};
use radlib::cmd::{Command, PatchCommand};
use radlib::engine::event::EngineEvent;
use uuid::Uuid;

use std::num::NonZeroU32;

use super::ft_picker::FixtureTypePicker;
use crate::engine::EngineManager;

const FIXTURE_PICKER_OVERLAY_ID: &str = "ft_picker";

#[derive(Clone)]
pub struct FixtureTable {
    columns: Vec<Column>,
}

impl FixtureTable {
    pub fn new(window: &mut Window, cx: &mut Context<Table<Self>>) -> Self {
        let event_handler = EngineManager::event_handler(cx);
        cx.subscribe_in(&event_handler, window, |table, _, event, window, cx| match event {
            EngineEvent::PatchChanged => table.refresh(window, cx),
        })
        .detach();

        Self {
            columns: vec![
                Column::new("fid", "Fixture Id"),
                Column::new("name", "Name").with_width(px(250.0)),
                Column::new("addr", "Address"),
                Column::new("ft", "Fixture Type").with_width(px(400.0)),
            ],
        }
    }

    fn edit_fids(&self, row_ids: Vec<Uuid>, window: &mut Window, cx: &mut Context<Table<Self>>) {
        fn generate_fids(start_fid: FixtureId, n: usize) -> Vec<FixtureId> {
            let mut new_fids = Vec::new();
            for i in 0..n as u32 {
                let new_fid = FixtureId(NonZeroU32::new(u32::from(start_fid.0) + i).unwrap());
                new_fids.push(new_fid);
            }
            new_fids
        }

        let initial_fid =
            EngineManager::read_patch(cx, |patch| patch.fixture(row_ids[0]).unwrap().fid);

        cx.update_wm(|wm, cx| {
            let value = initial_fid.map(|fid| fid.to_string()).unwrap_or_default();
            wm.open_text_modal(
                "fid_modal",
                "Set Fixture Id",
                value,
                window,
                cx,
                move |value, _, cx| {
                    let value = value.trim();

                    if value.is_empty() {
                        for &row_id in row_ids.iter() {
                            EngineManager::exec_and_log_err(
                                Command::Patch(PatchCommand::SetFixtureId {
                                    fixture_ref: row_id.into(),
                                    new_fid: None,
                                }),
                                cx,
                            );
                        }

                        return;
                    }

                    let Some(start_fid) = value.parse().ok() else {
                        return;
                    };

                    let generated_fids = generate_fids(start_fid, row_ids.len());

                    for (&row_id, new_fid) in row_ids.iter().zip(generated_fids) {
                        EngineManager::exec_and_log_err(
                            Command::Patch(PatchCommand::SetFixtureId {
                                fixture_ref: row_id.into(),
                                new_fid: Some(new_fid),
                            }),
                            cx,
                        );
                    }
                },
            );
        });
    }

    fn edit_names(&self, row_ids: Vec<Uuid>, window: &mut Window, cx: &mut Context<Table<Self>>) {
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

        let initial_name =
            EngineManager::read_patch(cx, |patch| patch.fixture(row_ids[0]).unwrap().name.clone());

        cx.update_wm(|wm, cx| {
            wm.open_text_modal(
                "name_modal",
                "Set Name",
                initial_name,
                window,
                cx,
                move |value, _, cx| {
                    let name = value.trim();
                    let generated_names = generate_names(&name, row_ids.len());
                    for (&row_id, new_name) in row_ids.iter().zip(generated_names) {
                        EngineManager::exec_and_log_err(
                            Command::Patch(PatchCommand::SetName {
                                fixture_ref: row_id.into(),
                                name: new_name,
                            }),
                            cx,
                        );
                    }
                },
            );
        });
    }

    fn edit_addrs(&self, row_ids: Vec<Uuid>, window: &mut Window, cx: &mut Context<Table<Self>>) {
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

        let initial_address = EngineManager::read_patch(cx, |patch| {
            let fixture = patch.fixture(row_ids[0]).unwrap();
            fixture.address.clone()
        });

        cx.update_wm(|wm, cx| {
            wm.open_text_modal(
                "addr_modal",
                "Set Address",
                initial_address.map(|a| a.to_string()).unwrap_or_default(),
                window,
                cx,
                move |value, _, cx| {
                    let value = value.trim();

                    if value.is_empty() {
                        for &row_id in row_ids.iter() {
                            EngineManager::exec_and_log_err(
                                Command::Patch(PatchCommand::SetAddress {
                                    fixture_ref: row_id.into(),
                                    address: None,
                                }),
                                cx,
                            );
                        }

                        return;
                    }

                    let Some(start_address) = value.parse().ok() else {
                        return;
                    };

                    let generated_addresses = generate_addresses(start_address, &row_ids, cx);

                    for (&row_id, new_address) in row_ids.iter().zip(generated_addresses) {
                        EngineManager::exec_and_log_err(
                            Command::Patch(PatchCommand::SetAddress {
                                fixture_ref: row_id.into(),
                                address: Some(new_address),
                            }),
                            cx,
                        );
                    }
                },
            );
        });
    }

    fn edit_fts(&mut self, row_ids: Vec<Uuid>, window: &mut Window, cx: &mut Context<Table<Self>>) {
        let ft_picker = self.open_ft_picker(row_ids[0], window, cx);

        cx.subscribe_in(
            &ft_picker,
            window,
            move |_, _, event: &SubmitEvent<(GdtfFixtureTypeId, String)>, window, cx| {
                let (ft_id, dmx_mode) = &event.value;

                for row_id in &row_ids {
                    EngineManager::exec_and_log_err(
                        Command::Patch(PatchCommand::SetFixtureTypeId {
                            fixture_ref: (*row_id).into(),
                            fixture_type_id: *ft_id,
                            dmx_mode: dmx_mode.clone(),
                        }),
                        cx,
                    );

                    EngineManager::exec_and_log_err(
                        Command::Patch(PatchCommand::SetAddress {
                            fixture_ref: (*row_id).into(),
                            address: None,
                        }),
                        cx,
                    );
                }

                cx.update_wm(|wm, _| {
                    wm.close_overlay(FIXTURE_PICKER_OVERLAY_ID, &window.window_handle(), window)
                });
            },
        )
        .detach();
    }

    fn open_ft_picker(
        &mut self,
        fixture_uuid: Uuid,
        window: &mut Window,
        cx: &mut Context<Table<Self>>,
    ) -> Entity<FixtureTypePicker> {
        let (ft_id, dmx_mode) = EngineManager::read_patch(cx, |patch| {
            let fixture = patch.fixture(fixture_uuid).unwrap();
            (fixture.fixture_type_id, fixture.dmx_mode.clone())
        });

        let ft_picker = cx.new(|cx| {
            FixtureTypePicker::new(window, cx).with_selected(ft_id, dmx_mode, window, cx)
        });

        cx.update_wm(|wm, cx| {
            wm.open_overlay(
                Overlay::new(
                    FIXTURE_PICKER_OVERLAY_ID,
                    "Select a Fixture Type",
                    ft_picker.clone(),
                    cx.focus_handle(),
                ),
                &window.window_handle(),
                window,
                cx,
            )
        });

        ft_picker
    }
}

impl TableDelegate for FixtureTable {
    type RowId = Uuid;

    fn column_count(&self, _cx: &App) -> usize {
        self.columns.len()
    }

    fn column(&self, col_ix: usize, _cx: &App) -> &Column {
        &self.columns[col_ix]
    }

    fn column_ix(&self, column_id: &str, _cx: &App) -> usize {
        self.columns.iter().position(|c| &c.id == column_id).unwrap()
    }

    fn sorted_row_ids(&self, cx: &App) -> Vec<Self::RowId> {
        EngineManager::read_patch(cx, |patch| {
            let mut fixtures = patch.fixtures().to_vec();
            fixtures.sort_by(|a, b| a.fid.cmp(&b.fid));
            fixtures.iter().map(|f| f.uuid()).collect()
        })
    }

    fn validate(&self, cx: &App) -> bool {
        EngineManager::read_patch(cx, |patch| patch.validate())
    }

    fn edit_selection(
        &mut self,
        column_id: &str,
        row_ids: Vec<Self::RowId>,
        window: &mut Window,
        cx: &mut Context<Table<Self>>,
    ) {
        if row_ids.is_empty() {
            return;
        };

        match column_id {
            "fid" => self.edit_fids(row_ids, window, cx),
            "name" => self.edit_names(row_ids, window, cx),
            "addr" => self.edit_addrs(row_ids, window, cx),
            "ft" => self.edit_fts(row_ids, window, cx),
            _ => {}
        }
    }

    fn render_cell(
        &self,
        row_id: &Self::RowId,
        col_ix: usize,
        window: &mut Window,
        cx: &mut Context<Table<Self>>,
    ) -> impl IntoElement {
        let column = self.column(col_ix, cx);
        let fixture =
            EngineManager::read_patch(cx, |patch| patch.fixture(*row_id).unwrap().clone());

        let render_cell = |content| {
            div().size_full().flex().items_center().px_1().child(content).into_any_element()
        };

        match column.id.to_string().as_str() {
            "fid" => match fixture.fid {
                Some(fid) => render_cell(fid.to_string()).into_any_element(),
                None => self.render_empty(window, cx).into_any_element(),
            },
            "name" => render_cell(fixture.name.to_string()),
            "addr" => match fixture.address {
                Some(addr) => render_cell(addr.to_string()).into_any_element(),
                None => self.render_empty(window, cx).into_any_element(),
            },
            "ft" => render_cell(format!(
                "{} ({})",
                EngineManager::read_patch(cx, |patch| fixture
                    .fixture_type(patch)
                    .long_name
                    .clone()),
                fixture.dmx_mode
            )),
            _ => self.render_empty(window, cx).into_any_element(),
        }
    }

    fn render_empty(
        &self,
        _window: &mut Window,
        cx: &mut Context<Table<Self>>,
    ) -> impl IntoElement {
        div()
            .size_full()
            .bg(cx.theme().red.with_opacity(0.25))
            .border_1()
            .border_color(cx.theme().red)
    }
}
