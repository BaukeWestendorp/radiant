use gpui::{ClickEvent, Context, Entity, Window, div, prelude::*};
use rd_engine::patch::FixtureDefinition;
use rd_ui::{
    ActiveTheme, Button, Form, FormEvent, FormState, Popup, PopupAppExt, Table, TableSelection,
    TableState, h_flex, v_flex,
};

use crate::{app::settings::patch::add_fixture::AddFixtureFormData, engine::EngineAppExt};

mod add_fixture;
mod patch_table;

pub struct PatchView {
    table: Entity<TableState<patch_table::PatchTable>>,
}

impl PatchView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let fixture_definitions =
            cx.new(|cx| cx.engine_snapshot().patch().definition().fixtures().to_vec());

        let selection = cx.new(|_| TableSelection::Multiple(Vec::new()));

        Self {
            table: cx.new(|cx| {
                TableState::new(
                    patch_table::PatchTable::new(fixture_definitions),
                    selection,
                    window,
                    cx,
                )
            }),
        }
    }

    fn show_add_fixtures_popup(
        &mut self,
        _event: &ClickEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        cx.open_popup(window, |window, cx| {
            struct AddFixturePopup {
                form: Entity<FormState<add_fixture::AddFixtureForm>>,
            }

            impl Render for AddFixturePopup {
                fn render(
                    &mut self,
                    _window: &mut Window,
                    _cx: &mut Context<Self>,
                ) -> impl IntoElement {
                    div().size_full().p_2().child(Form::new(self.form.clone()))
                }
            }

            let form = cx
                .new(|cx| FormState::new(add_fixture::AddFixtureForm::new(window, cx), window, cx));

            window
                .subscribe(&form, cx, |_, event, window, cx| match event {
                    FormEvent::Submit { data } => {
                        let AddFixtureFormData { fixture_id, address, name, fixture_kind, count } =
                            data;

                        let patch = cx.engine_snapshot().patch();
                        let Some(dmx_mode) = fixture_kind.dmx_mode(&patch) else {
                            log::error!("Could not find DMX mode for FixtureKind");
                            return;
                        };

                        let mut digit_start = name.len();
                        for (idx, c) in name.char_indices().rev() {
                            if c.is_ascii_digit() {
                                digit_start = idx;
                            } else {
                                break;
                            }
                        }

                        let (base_name, start_num) = if digit_start < name.len() {
                            (&name[..digit_start], name[digit_start..].parse::<u32>().ok())
                        } else {
                            (&name[..], None)
                        };

                        for i in 0..*count {
                            let enumerated_id = fixture_id
                                .offset(i as i32)
                                .expect("Offset should always be positive");

                            let enumerated_name = match start_num {
                                Some(num) => format!("{}{}", base_name, num + i as u32),
                                None => name.to_string(),
                            };

                            let enumerated_dmx_address = address
                                .with_channel_offset(
                                    (dmx_mode.max_channel_offset() * i as u32) as i32,
                                )
                                .expect("Offset should always be positive");

                            let fixture = FixtureDefinition::new(
                                enumerated_id,
                                enumerated_name,
                                enumerated_dmx_address,
                                fixture_kind.clone(),
                            );

                            // FIXME: Find out how to actually add them to the patch. Maybe a Command?
                            dbg!(fixture);
                        }

                        cx.close_popup(window);
                    }
                })
                .detach();

            let popup = cx.new(|_| AddFixturePopup { form });

            Popup::custom(popup, "Add Fixtures")
        });
    }
}

impl Render for PatchView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let bottom_bar = h_flex()
            .px_1()
            .py_0p5()
            .w_full()
            .border_t_1()
            .border_color(cx.theme().border_primary)
            .child(
                // FIXME: Disable this button with missing fields or if the fixtures that would be created are invalid.
                Button::new("add-fixtures")
                    .child("Add Fixture(s)")
                    .on_click(cx.listener(Self::show_add_fixtures_popup)),
            );

        v_flex().size_full().child(Table::new(self.table.clone())).child(bottom_bar)
    }
}
