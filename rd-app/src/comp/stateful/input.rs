use rd::project::FixtureKind;
use rd_artnet::PortAddress;
use rd_rigger::gdtf::{FixtureTypeId, Name};
use rd_ui::{
    ActiveTheme, Emphasis, InputPopup, PopupAppExt, PopupSize, StyledExt, StyledParentExt,
    StyledStatefulInteractiveElementExt, c_flex,
    comp::{
        Disableable, FocusableComponent, Icon, IconSize, IconVariant, Identifiable, Labelled,
        stateful::{
            Field, Form, FormInput, FormWidget, InputEvent, InputValue, KeyPath, SelectionEvent,
            Submittable, Table, TableColumn, TableSelection, TableSelectionMode,
        },
    },
    gpui::{
        App, Div, ElementId, Entity, EventEmitter, FocusHandle, Focusable, StyleRefinement, Window,
        div, prelude::*,
    },
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
    Field::custom(
        id,
        window,
        cx,
        |s| u32::from_str_radix(s, 10).ok().into(),
        |v| v.to_string().into(),
    )
    .with_placeholder("101", cx)
    .with_text_validator(cx, |s| s.is_empty() || u32::from_str_radix(s, 10).is_ok())
    .with_validator(cx, |v| *v > 0)
    .with_submit_validator(cx, |v| *v > 0)
}

pub fn address_field(
    id: impl Into<ElementId>,
    window: &mut Window,
    cx: &mut Context<Field<rd_dmx::Address>>,
) -> Field<rd_dmx::Address> {
    Field::custom(
        id,
        window,
        cx,
        |s| rd_dmx::Address::from_str(s).ok().into(),
        |v| v.to_string().into(),
    )
    .with_placeholder("1.1", cx)
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
        |s| {
            let Ok(v) = u16::from_str(s) else { return InputValue::Invalid };
            let Ok(addr) = PortAddress::from_absolute(v) else {
                return InputValue::Invalid;
            };
            InputValue::Valid(addr)
        },
        |v| v.as_u16().to_string().into(),
    )
    .with_placeholder("1", cx)
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
        |s| {
            let Ok(v) = u16::from_str(s) else { return InputValue::Invalid };
            let Ok(universe_id) = rd_dmx::UniverseId::new(v) else {
                return InputValue::Invalid;
            };
            InputValue::Valid(universe_id)
        },
        |v| v.to_string().into(),
    )
    .with_placeholder("1", cx)
    .with_text_validator(cx, |s| s.is_empty() || rd_dmx::UniverseId::from_str(s).is_ok())
    .with_submit_validator(cx, |_| true)
}
pub struct FixtureKindPicker {
    id: ElementId,
    focus_handle: FocusHandle,
    style: StyleRefinement,
    compact: bool,
    disabled: bool,

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

        cx.observe_in(&mode, window, {
            move |this, mode, _window, cx| {
                let Some(ftid) = this.ftid.read(cx) else {
                    this.fixture_kind.write(cx, None);
                    cx.emit(InputEvent::Change::<FixtureKind>(InputValue::Invalid));
                    return;
                };

                let Some(mode) = mode.read(cx) else {
                    this.fixture_kind.write(cx, None);
                    cx.emit(InputEvent::Change::<FixtureKind>(InputValue::Invalid));
                    return;
                };

                let fixture_kind =
                    FixtureKind { fixture_type_id: *ftid, dmx_mode: mode.to_string() };
                this.fixture_kind.write(cx, Some(fixture_kind.clone()));

                cx.emit(InputEvent::Change::<FixtureKind>(InputValue::Valid(fixture_kind)));
                cx.notify();
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
            move |_, ftid_table, event: &SelectionEvent, cx| match event {
                SelectionEvent::Changed => {
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
                        let Some(gdtf) = cx.engine().with_project(|project| {
                            project.patch.gdtfs.get(&selected_ftid).cloned()
                        }) else {
                            mode_rows.update(cx, |mode_rows, cx| {
                                mode_rows.clear();
                                cx.notify();
                            });
                            return;
                        };

                        let modes =
                            gdtf.dmx_modes().iter().map(|mode| mode.name().clone()).collect();
                        mode_rows.write(cx, modes);
                    }
                }
            }
        })
        .detach();

        cx.subscribe(&mode_table, {
            let mode = mode.clone();
            move |_, mode_table, _: &SelectionEvent, cx| {
                let selected_mode =
                    mode_table.read(cx).selected_rows(cx).first().map(|mode| (*mode).clone());

                mode.write(cx, selected_mode);
            }
        })
        .detach();

        Self {
            id: id.into(),
            focus_handle: cx.focus_handle().tab_stop(true),
            style: StyleRefinement::default(),
            compact: false,
            disabled: false,
            ftid_table,
            mode_table,
            ftid,
            mode,
            fixture_kind,
        }
    }

    pub fn compact(&self) -> bool {
        self.compact
    }

    pub fn set_compact(&mut self, compact: bool, cx: &mut Context<Self>) {
        self.compact = compact;
        cx.notify();
    }

    pub fn with_compact(mut self, compact: bool, cx: &mut Context<Self>) -> Self {
        self.set_compact(compact, cx);
        self
    }

    fn render_picker(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> Div {
        v_flex()
            .emphasis(Emphasis::Primary, cx)
            .p_2()
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
    }
}

impl Render for FixtureKindPicker {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.compact() {
            let selected_label = if let Some(fixture_kind) = self.fixture_kind.read(cx) {
                cx.engine().with_project(|project| fixture_kind.display(project)).into_any_element()
            } else {
                "Select a Fixture Kind...".into_any_element()
            };

            div()
                .id((self.id(cx).clone(), "preview"))
                .when(!self.disabled(cx), |e| {
                    e.track_focus(&self.focus_handle(cx)).focus_ring(
                        &self.focus_handle(cx),
                        window,
                        cx,
                    )
                })
                .h_flex()
                .justify_between()
                .gap_2()
                .size_full()
                .h(rd_ui::comp::INPUT_SIZE)
                .px_1p5()
                .py_0p5()
                .min_w(rd_ui::comp::INPUT_SIZE * 2.0)
                .w(rd_ui::comp::INPUT_SIZE * 6.0)
                .when(self.disabled(cx), |e| e.disabled_emphasis_bordered(Emphasis::Secondary, cx))
                .when(!self.disabled(cx), |e| {
                    e.interactive_emphasis_bordered(Emphasis::Secondary, cx).on_click(cx.listener(
                        |this, _, window, cx| {
                            let input = cx.new(|cx| {
                                FixtureKindPicker::new("fixture_kind", window, cx).size_full()
                            });
                            let fixture_kind = this.fixture_kind.clone();
                            let popup = cx.new(|cx| {
                                InputPopup::new(input, window, cx).with_on_submit(
                                    window,
                                    cx,
                                    move |value, _, cx| {
                                        fixture_kind.write(cx, Some(value.clone()));
                                    },
                                )
                            });

                            cx.push_popup(
                                "Select Fixture Kind",
                                popup,
                                PopupSize::Max,
                                Some(this.focus_handle.clone()),
                            );

                            cx.notify();
                        },
                    ))
                })
                .child(
                    div()
                        .w_full()
                        .text_color(cx.theme().fg_primary)
                        .when(self.fixture_kind.read(cx).is_none(), |e| {
                            e.text_color(cx.theme().fg_secondary)
                        })
                        .overflow_x_hidden()
                        .truncate()
                        .text_ellipsis()
                        .child(selected_label),
                )
                .child(Icon::new(IconVariant::ChevronDown, IconSize::ExtraSmall))
                .refine_style(&self.style)
                .into_any_element()
        } else {
            self.render_picker(window, cx).refine_style(&self.style).into_any_element()
        }
    }
}

impl Identifiable for FixtureKindPicker {
    fn id<'a>(&'a self, _cx: &'a App) -> &'a ElementId {
        &self.id
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

impl Disableable for FixtureKindPicker {
    fn disabled(&self, _cx: &App) -> bool {
        self.disabled
    }

    fn set_disabled(&mut self, disabled: bool, _cx: &mut App) {
        self.disabled = disabled;
    }
}

impl Styled for FixtureKindPicker {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl FormWidget<FixtureKind> for FixtureKindPicker {
    fn value(&self, cx: &App) -> InputValue<FixtureKind> {
        self.fixture_kind.read(cx).clone().into()
    }

    fn set_value(&mut self, value: FixtureKind, cx: &mut Context<Self>) {
        self.ftid.write(cx, Some(value.fixture_type_id));
        self.mode.write(cx, Some(Name::new(value.dmx_mode)));
    }
}

impl EventEmitter<InputEvent<FixtureKind>> for FixtureKindPicker {}

impl Submittable<FixtureKind> for FixtureKindPicker {
    fn value(&self, cx: &App) -> InputValue<FixtureKind> {
        self.fixture_kind.read(cx).clone().into()
    }
}

pub struct FixtureConfigEditor {
    form: Entity<Form<PartialFixtureConfig>>,
    focus_handle: FocusHandle,
}

impl FixtureConfigEditor {
    pub fn new(partial: PartialFixtureConfig, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let data = cx.new(|_| partial);
        Self {
            form: cx.new(|cx| {
                Form::new(data, window, cx)
                    .with_input(
                        FormInput::new(
                            cx.new(|cx| fixture_id_field("fixture_id", window, cx)),
                            KeyPath::new(
                                |d: &PartialFixtureConfig| d.id.into(),
                                |d, value| d.id = Some(value),
                            ),
                            cx,
                        )
                        .with_label("Fixture ID"),
                        cx,
                    )
                    .with_input(
                        FormInput::new(
                            cx.new(|cx| {
                                Field::<String>::new("name", window, cx)
                                    .with_placeholder("Fixture 1", cx)
                            }),
                            KeyPath::new(
                                |d: &PartialFixtureConfig| InputValue::Valid(d.name.clone()),
                                |d, value| d.name = value,
                            ),
                            cx,
                        )
                        .with_label("Name"),
                        cx,
                    )
                    .with_input(
                        FormInput::new(
                            cx.new(|cx| address_field("dmx_address", window, cx)),
                            KeyPath::new(
                                |d: &PartialFixtureConfig| d.dmx_address.into(),
                                |d, value| d.dmx_address = Some(value),
                            ),
                            cx,
                        )
                        .with_label("DMX Address"),
                        cx,
                    )
                    .with_input(
                        FormInput::new(
                            cx.new(|cx| {
                                FixtureKindPicker::new("fixture_kind", window, cx)
                                    .with_compact(true, cx)
                            }),
                            KeyPath::new(
                                |d: &PartialFixtureConfig| d.fixture_kind.clone().into(),
                                |d, value| d.fixture_kind = Some(value),
                            ),
                            cx,
                        )
                        .with_label("Fixture Kind"),
                        cx,
                    )
            }),
            focus_handle: cx.focus_handle().tab_stop(true),
        }
    }
}

impl Render for FixtureConfigEditor {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.form.clone()
    }
}

impl Focusable for FixtureConfigEditor {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl FocusableComponent for FixtureConfigEditor {
    fn set_focus_handle(&mut self, focus_handle: FocusHandle, _cx: &mut App) {
        self.focus_handle = focus_handle;
    }
}

impl FormWidget<rd::project::FixtureConfig> for FixtureConfigEditor {
    fn value(&self, cx: &App) -> InputValue<rd::project::FixtureConfig> {
        let partial = self.form.read(cx).data().read(cx);
        let Some(id) = partial.id else { return InputValue::Invalid };
        let name = partial.name.clone();
        let Some(dmx_address) = partial.dmx_address else {
            return InputValue::Invalid;
        };
        // FIMXE: FixtureConfig.fixture_kind should be renamed to `kind`.
        let Some(fixture_kind) = partial.fixture_kind.clone() else {
            return InputValue::Invalid;
        };

        InputValue::Valid(rd::project::FixtureConfig { id, name, dmx_address, fixture_kind })
    }

    fn set_value(&mut self, value: rd::project::FixtureConfig, cx: &mut Context<Self>) {
        let partial = PartialFixtureConfig {
            id: Some(value.id),
            name: value.name,
            dmx_address: Some(value.dmx_address),
            fixture_kind: Some(value.fixture_kind),
        };

        self.form.read(cx).data().write(cx, partial);
    }
}

impl EventEmitter<InputEvent<rd::project::FixtureConfig>> for FixtureConfigEditor {}

impl Submittable<rd::project::FixtureConfig> for FixtureConfigEditor {
    fn value(&self, cx: &App) -> InputValue<rd::project::FixtureConfig> {
        <Self as FormWidget<rd::project::FixtureConfig>>::value(self, cx)
    }
}

#[derive(Debug, Default)]
pub struct PartialFixtureConfig {
    pub id: Option<u32>,
    pub name: String,
    pub dmx_address: Option<rd_dmx::Address>,
    pub fixture_kind: Option<FixtureKind>,
}
