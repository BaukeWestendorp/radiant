use rd::project::FixtureKind;
use rd_artnet::PortAddress;
use rd_rigger::gdtf::{FixtureTypeId, Name};
use rd_ui::{
    ActiveTheme, Emphasis, StyledExt, c_flex,
    comp::stateful::{self, Field, Table, TableColumn, TableSelection, TableSelectionMode},
    gpui::{ElementId, Entity, Window, div, prelude::*},
    h_flex, v_flex,
};
use std::str::FromStr as _;

use crate::engine::EngineAppExt;

pub fn fixture_id_field(
    id: impl Into<ElementId>,
    window: &mut Window,
    cx: &mut Context<Field<u32>>,
) -> Field<u32> {
    Field::custom(id, window, cx, |s| u32::from_str_radix(s, 10).ok(), |v| v.to_string().into())
        .with_text_validator(cx, |s| s.is_empty() || u32::from_str_radix(s, 10).is_ok())
        .with_validator(cx, |v| *v > 0)
        .with_submit_validator(cx, |v| *v > 0)
}

pub fn address_field(
    id: impl Into<ElementId>,
    window: &mut Window,
    cx: &mut Context<Field<rd_dmx::Address>>,
) -> Field<rd_dmx::Address> {
    Field::custom(id, window, cx, |s| rd_dmx::Address::from_str(s).ok(), |v| v.to_string().into())
        .with_text_validator(cx, |s| {
            if s.is_empty() {
                return true;
            }

            if s.starts_with('.') {
                return false;
            }

            let mut parts = s.split('.');

            let universe_str = parts.next().unwrap_or("");
            if rd_dmx::UniverseId::from_str(universe_str).is_err() {
                return false;
            }

            if let Some(channel_str) = parts.next() {
                if !channel_str.is_empty() && rd_dmx::Channel::from_str(channel_str).is_err() {
                    return false;
                }
            }

            parts.next().is_none()
        })
        .with_submit_validator(cx, |_| true)
}

pub fn port_address_field(
    id: impl Into<ElementId>,
    window: &mut Window,
    cx: &mut Context<Field<PortAddress>>,
) -> Field<PortAddress> {
    Field::custom(
        id,
        window,
        cx,
        |s| PortAddress::from_absolute(u16::from_str(s).ok()?).ok(),
        |v| v.as_u16().to_string().into(),
    )
    .with_text_validator(cx, |s| s.is_empty() || u16::from_str(s).is_ok())
    .with_submit_validator(cx, |v| *v <= PortAddress::MAX)
    .with_placeholder("Absolute address", cx)
}

pub fn universe_id_field(
    id: impl Into<ElementId>,
    window: &mut Window,
    cx: &mut Context<Field<rd_dmx::UniverseId>>,
) -> Field<rd_dmx::UniverseId> {
    Field::custom(
        id,
        window,
        cx,
        |s| rd_dmx::UniverseId::from_str(s).ok(),
        |v| v.to_string().into(),
    )
    .with_text_validator(cx, |s| s.is_empty() || rd_dmx::UniverseId::from_str(s).is_ok())
    .with_submit_validator(cx, |_| true)
}

pub struct FixtureKindPicker {
    ftid_table: Entity<Table<FixtureTypeId>>,
    mode_table: Entity<Table<Name>>,
    fixture_kind: Entity<Option<FixtureKind>>,

    ftid: Entity<Option<FixtureTypeId>>,
    mode: Entity<Option<Name>>,
}

impl FixtureKindPicker {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let ftid = cx.new(|_| None::<FixtureTypeId>);
        let mode = cx.new(|_| None::<Name>);
        let fixture_kind = cx.new(|_| None::<FixtureKind>);

        cx.observe(&mode, {
            move |this, mode, cx| {
                let Some(ftid) = this.ftid.read(cx) else {
                    this.fixture_kind.write(cx, None);
                    return;
                };

                let Some(mode) = mode.read(cx) else {
                    this.fixture_kind.write(cx, None);
                    return;
                };

                this.fixture_kind.write(
                    cx,
                    Some(FixtureKind { fixture_type_id: *ftid, dmx_mode: mode.to_string() }),
                );
            }
        })
        .detach();

        let ftid_rows = cx.new(|cx| {
            cx.engine().with_project(|project| project.patch.gdtfs.keys().copied().collect())
        });

        let mode_rows = cx.new(|_| Vec::new());

        let ftid_table = cx.new(|cx| {
            Table::new("ftid", ftid_rows, window, cx)
                .with_columns(
                    vec![
                        TableColumn::<FixtureTypeId>::new("manufacturer")
                            .with_label("Manufacturer")
                            .with_element(|ftid, _, cx| {
                                let manufacturer = cx.engine().with_project(|project| {
                                    project
                                        .patch
                                        .gdtfs
                                        .get(ftid)
                                        .map(|gdtf| gdtf.manufacturer().to_string())
                                });

                                manufacturer
                                    .unwrap_or_else(|| "<unknown>".to_string())
                                    .into_any_element()
                            }),
                        TableColumn::<FixtureTypeId>::new("name").with_label("Name").with_element(
                            |ftid, _, cx| {
                                let name = cx.engine().with_project(|project| {
                                    project
                                        .patch
                                        .gdtfs
                                        .get(ftid)
                                        .map(|gdtf| gdtf.name().to_string())
                                });

                                name.unwrap_or_else(|| "<unknown>".to_string()).into_any_element()
                            },
                        ),
                        TableColumn::<FixtureTypeId>::new("ftid")
                            .with_label("Fixture Type ID")
                            .with_element(|ftid, _, _| ftid.to_string().into_any_element()),
                    ],
                    cx,
                )
                .with_selection(TableSelection::new(TableSelectionMode::Single), cx)
        });

        let mode_table = cx.new({
            let mode_rows = mode_rows.clone();
            |cx| {
                Table::new("mode", mode_rows, window, cx)
                    .with_columns(
                        vec![
                            TableColumn::<Name>::new("mode")
                                .with_label("DMX Mode")
                                .with_element(|mode, _, _| mode.to_string().into_any_element()),
                            TableColumn::<Name>::new("channels")
                                .with_label("Channels")
                                .with_element({
                                    let ftid = ftid.clone();
                                    move |mode, _, cx| {
                                        let Some(ftid) = ftid.read(cx) else {
                                            return "<unknown>".into_any_element();
                                        };

                                        let Some(gdtf) = cx.engine().with_project(|project| {
                                            project.patch.gdtfs.get(&ftid).cloned()
                                        }) else {
                                            return "<unknown>".into_any_element();
                                        };

                                        let Some(dmx_mode) = gdtf.dmx_mode(mode) else {
                                            return "<unknown>".into_any_element();
                                        };

                                        dmx_mode.max_channel_offset().to_string().into_any_element()
                                    }
                                }),
                        ],
                        cx,
                    )
                    .with_selection(TableSelection::new(TableSelectionMode::Single), cx)
            }
        });

        cx.subscribe(&ftid_table, {
            let mode_rows = mode_rows.clone();
            let mode_table = mode_table.clone();
            let ftid = ftid.clone();
            move |_, ftid_table, _: &stateful::event::SelectionChanged, cx| {
                let selected_ftid =
                    ftid_table.read(cx).selected_rows(cx).first().map(|ftid| **ftid);

                if &selected_ftid != ftid.read(cx) {
                    mode_table.update(cx, |mode_table, cx| {
                        mode_table.clear_selection(cx);
                        cx.notify();
                    })
                }

                ftid.write(cx, selected_ftid);

                if let Some(selected_ftid) = selected_ftid {
                    let Some(gdtf) = cx
                        .engine()
                        .with_project(|project| project.patch.gdtfs.get(&selected_ftid).cloned())
                    else {
                        mode_rows.update(cx, |mode_rows, cx| {
                            mode_rows.clear();
                            cx.notify();
                        });
                        return;
                    };

                    let modes = gdtf.dmx_modes().iter().map(|mode| mode.name().clone()).collect();
                    mode_rows.write(cx, modes);
                }
            }
        })
        .detach();

        cx.subscribe(&mode_table, {
            let mode = mode.clone();
            move |_, mode_table, _: &stateful::event::SelectionChanged, cx| {
                let selected_mode =
                    mode_table.read(cx).selected_rows(cx).first().map(|mode| (*mode).clone());

                mode.write(cx, selected_mode);
            }
        })
        .detach();

        Self { ftid_table, mode_table, ftid, mode, fixture_kind }
    }

    pub fn ftid(&self) -> Entity<Option<FixtureTypeId>> {
        self.ftid.clone()
    }

    pub fn mode(&self) -> Entity<Option<Name>> {
        self.mode.clone()
    }

    pub fn fixture_kind(&self) -> Entity<Option<FixtureKind>> {
        self.fixture_kind.clone()
    }
}

impl Render for FixtureKindPicker {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .size_full()
            .child(
                h_flex()
                    .gap_2()
                    .size_full()
                    .child(
                        div()
                            .size_full()
                            .emphasis_bordered(Emphasis::Primary, cx)
                            .child(self.ftid_table.clone()),
                    )
                    .child(div().size_full().emphasis_bordered(Emphasis::Primary, cx).child(
                        if self.ftid.read(cx).is_some() {
                            self.mode_table.clone().into_any_element()
                        } else {
                            c_flex()
                                .size_full()
                                .child(
                                    div()
                                        .text_color(cx.theme().fg_secondary)
                                        .child("Select a fixture type to see its DMX modes"),
                                )
                                .into_any_element()
                        },
                    )),
            )
            .child(div().w_full().p_2().emphasis_bordered(Emphasis::Primary, cx).child(
                if let Some(fixture_kind) = self.fixture_kind.read(cx) {
                    let gdtf = cx.engine().with_project(|project| {
                        project.patch.gdtfs.get(&fixture_kind.fixture_type_id).cloned()
                    });

                    if let Some(gdtf) = gdtf {
                        h_flex()
                            .child(div().text_color(cx.theme().fg_primary).child(format!(
                                "{} {} ",
                                gdtf.manufacturer().trim(),
                                gdtf.name().trim(),
                            )))
                            .child(
                                div()
                                    .text_color(cx.theme().fg_secondary)
                                    .child(format!("[{}]", fixture_kind.dmx_mode.trim())),
                            )
                    } else {
                        div()
                            .text_color(cx.theme().fg_secondary)
                            .child("Selected fixture type not found in project")
                    }
                } else {
                    div()
                        .text_color(cx.theme().fg_secondary)
                        .child("Select a fixture type and DMX mode to see its details")
                },
            ))
            .into_any_element()
    }
}
