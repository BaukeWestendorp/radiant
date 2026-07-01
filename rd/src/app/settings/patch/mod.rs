use gpui::{ClickEvent, Context, Entity, Window, div, prelude::*};
use rd_ui::{
    ActiveTheme, Button, Form, FormEvent, FormState, Popup, PopupAppExt, Table, TableState, h_flex,
    v_flex,
};

use crate::engine::EngineAppExt;

mod add_fixture;
mod patch_table;

pub struct PatchView {
    table: Entity<TableState<patch_table::PatchTable>>,
}

impl PatchView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let fixture_definitions =
            cx.new(|cx| cx.engine_snapshot().patch().definition().fixtures().to_vec());

        let selection = cx.new(|_| Vec::new());

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

            cx.subscribe(&form, |_, event, _| match event {
                FormEvent::Submit { data } => {
                    dbg!(data);
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
                Button::new("add-fixtures")
                    .child("Add Fixture(s)")
                    .on_click(cx.listener(Self::show_add_fixtures_popup)),
            );

        v_flex().size_full().child(Table::new(self.table.clone())).child(bottom_bar)
    }
}
