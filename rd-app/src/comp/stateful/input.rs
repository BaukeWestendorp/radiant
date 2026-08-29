use rd::project::FixtureKind;
use rd_artnet::PortAddress;
use rd_rigger::gdtf::{FixtureTypeId, Name};
use rd_ui::{
    ActiveTheme, Emphasis, StyledExt, StyledParentExt, c_flex,
    comp::{
        FocusableComponent,
        stateful::{self, Field, Table, TableColumn, TableSelection, TableSelectionMode},
    },
    gpui::{App, ElementId, Entity, EventEmitter, FocusHandle, Focusable, Window, div, prelude::*},
    h_flex, v_flex,
};
use std::str::FromStr as _;

use crate::engine::EngineAppExt;

// FIXME: A lot of code in this file can be simplified with helpers for getting DMX Mode or Fixture Types.

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
    id: ElementId,
    focus_handle: FocusHandle,

    ftid_table: Entity<Table<FixtureTypeId>>,
    mode_table: Entity<Table<Name>>,
    fixture_kind: Entity<Option<FixtureKind>>,

    ftid: Entity<Option<FixtureTypeId>>,
    mode: Entity<Option<Name>>,
}

impl FixtureKindPicker {
    pub fn new(id: impl Into<ElementId>, window: &mut Window, cx: &mut Context<Self>) -> Self {
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

                let fixture_kind =
                    FixtureKind { fixture_type_id: *ftid, dmx_mode: mode.to_string() };
                this.fixture_kind.write(cx, Some(fixture_kind.clone()));

                cx.emit(stateful::event::Submit::<FixtureKind>(fixture_kind));
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

        Self {
            id: id.into(),
            focus_handle: cx.focus_handle(),
            ftid_table,
            mode_table,
            ftid,
            mode,
            fixture_kind,
        }
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
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .emphasis(Emphasis::Primary, cx)
            .p_2()
            .id(self.id.clone())
            .track_focus(&self.focus_handle)
            .focus_ring(&self.focus_handle, window, cx)
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
                                        .p_2()
                                        .text_color(cx.theme().fg_secondary)
                                        .child("Select a fixture type to see its DMX modes"),
                                )
                                .into_any_element()
                        },
                    )),
            )
            .child(div().w_full().p_2().emphasis_bordered(Emphasis::Primary, cx).child(
                if let Some(fixture_kind) = self.fixture_kind.read(cx) {
                    let fk_label =
                        cx.engine().with_project(|project| fixture_kind.display(project));
                    div().child(fk_label)
                } else {
                    div()
                        .text_color(cx.theme().fg_secondary)
                        .child("Select a fixture type and DMX mode to see its details")
                },
            ))
            .into_any_element()
    }
}

impl Focusable for FixtureKindPicker {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl FocusableComponent for FixtureKindPicker {
    fn set_focus_handle(&mut self, focus_handle: FocusHandle, _cx: &mut App) {
        self.focus_handle = focus_handle;
    }
}

impl EventEmitter<stateful::event::Submit<FixtureKind>> for FixtureKindPicker {}
impl EventEmitter<stateful::event::Change<FixtureKind>> for FixtureKindPicker {}
