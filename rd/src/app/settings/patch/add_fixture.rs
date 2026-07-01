use gpui::{App, Context, Entity, FlexDirection, SharedString, Window, prelude::*};
use rd_engine::{dmx::Address, patch::FixtureIdPart};
use rd_ui::{Button, Field, FieldState, FormDelegate, FormNode, FormState};

use crate::engine::EngineAppExt;

pub struct AddFixtureForm {
    pub fixture_id: Entity<FieldState<FixtureIdPart>>,
    pub address: Entity<FieldState<Address>>,
    pub name: Entity<FieldState<SharedString>>,
    pub fixture_type: Entity<FieldState<SharedString>>,
    pub dmx_mode: Entity<FieldState<SharedString>>,
}

impl AddFixtureForm {
    pub fn new(window: &mut Window, cx: &mut App) -> Self {
        let next_fixture_id = cx
            .engine_snapshot()
            .patch()
            .root_fixtures()
            .map(|f| f.id().root().offset(1).expect("Should always be positive"))
            .max()
            .unwrap_or_default();

        let next_name =
            format!("Fixture {}", cx.engine_snapshot().patch().root_fixtures().count() + 1).into();

        let next_address = cx
            .engine_snapshot()
            .patch()
            .fixtures()
            .iter()
            .map(|f| {
                f.dmx_address()
                    .with_channel_offset(f.dmx_mode().max_channel_offset() as i32 + 1)
                    .expect("Channel count should always be positive")
            })
            .max()
            .unwrap_or_default();

        Self {
            fixture_id: cx.new(|cx| {
                FieldState::new("id-field", cx.focus_handle(), window, cx)
                    .with_value(next_fixture_id, cx)
            }),
            address: cx.new(|cx| {
                FieldState::new("address-field", cx.focus_handle(), window, cx)
                    .with_value(next_address, cx)
            }),
            name: cx.new(|cx| {
                FieldState::new("name-field", cx.focus_handle(), window, cx)
                    .with_value(next_name, cx)
            }),
            fixture_type: cx.new(|cx| FieldState::new("gdtf-field", cx.focus_handle(), window, cx)),
            dmx_mode: cx.new(|cx| FieldState::new("dmx-mode-field", cx.focus_handle(), window, cx)),
        }
    }
}

impl FormDelegate for AddFixtureForm {
    type Id = AddFixtureFormId;
    type Data = AddFixtureFormData;

    fn layout(&self, _cx: &App) -> Vec<FormNode<Self::Id>> {
        vec![
            FormNode::section(
                "Identification",
                FlexDirection::Row,
                vec![
                    FormNode::field(AddFixtureFormId::FixtureId, "Fixture ID"),
                    FormNode::field(AddFixtureFormId::Address, "Address"),
                ],
            ),
            FormNode::field(AddFixtureFormId::Name, "Name"),
            FormNode::section(
                "GDTF",
                FlexDirection::Row,
                vec![
                    FormNode::field(AddFixtureFormId::FixtureType, "Fixture Type"),
                    FormNode::field(AddFixtureFormId::DmxMode, "DMX Mode"),
                ],
            ),
            FormNode::section_headless(
                FlexDirection::Row,
                vec![FormNode::custom(AddFixtureFormId::Submit)],
            ),
        ]
    }

    fn render_input(
        &self,
        id: &Self::Id,
        _window: &mut Window,
        cx: &mut Context<FormState<Self>>,
    ) -> impl IntoElement {
        match id {
            AddFixtureFormId::FixtureId => Field::new(self.fixture_id.clone()).into_any_element(),
            AddFixtureFormId::Address => Field::new(self.address.clone()).into_any_element(),
            AddFixtureFormId::Name => Field::new(self.name.clone()).into_any_element(),
            AddFixtureFormId::FixtureType => {
                Field::new(self.fixture_type.clone()).into_any_element()
            }
            AddFixtureFormId::DmxMode => Field::new(self.dmx_mode.clone()).into_any_element(),
            AddFixtureFormId::Submit => Button::new("submit")
                .w_full()
                .child("Add Fixture(s)")
                .on_click(cx.listener(|state, _event, _window, cx| {
                    state.submit(cx);
                }))
                .into_any_element(),
        }
    }

    fn extract_data(&self, cx: &App) -> Option<Self::Data> {
        Some(AddFixtureFormData {
            fixture_id: self.fixture_id.read(cx).value(cx)?,
            address: self.address.read(cx).value(cx)?,
            name: self.name.read(cx).value(cx)?.to_string(),
            fixture_type: self.fixture_type.read(cx).value(cx)?.to_string(),
            dmx_mode: self.dmx_mode.read(cx).value(cx)?.to_string(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct AddFixtureFormData {
    pub fixture_id: FixtureIdPart,
    pub address: Address,
    pub name: String,
    pub fixture_type: String,
    pub dmx_mode: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AddFixtureFormId {
    FixtureId,
    Address,
    Name,
    FixtureType,
    DmxMode,
    Submit,
}
