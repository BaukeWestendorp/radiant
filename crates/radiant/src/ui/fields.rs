use gpui::{Context, Window};
use nui::input::{NumberField, TextField};
use radlib::builtin::FixtureId;

pub fn int_field(
    value: Option<i32>,
    window: &mut Window,
    cx: &mut Context<NumberField>,
) -> NumberField {
    NumberField::new("int_field", cx.focus_handle(), window, cx)
        .with_value(value.map(|value| value.into()), cx)
        .with_step(Some(1.0), cx)
}

pub fn uint_field(
    value: Option<u32>,
    window: &mut Window,
    cx: &mut Context<NumberField>,
) -> NumberField {
    NumberField::new("uint_field", cx.focus_handle(), window, cx)
        .with_value(value.map(|value| value.into()), cx)
        .with_min(Some(0.0), cx)
        .with_step(Some(1.0), cx)
}

pub fn fid_field(
    value: Option<FixtureId>,
    window: &mut Window,
    cx: &mut Context<NumberField>,
) -> NumberField {
    NumberField::new("fid_field", cx.focus_handle(), window, cx)
        .with_value(value.map(|value| u32::from(value).into()), cx)
        .with_step(Some(1.0), cx)
        .with_min(Some(1.0), cx)
}

pub fn address_field(
    value: Option<dmx::Address>,
    window: &mut Window,
    cx: &mut Context<TextField>,
) -> TextField {
    let string_value = value
        .and_then(|value| dmx::Address::try_from(value).ok())
        .map(|addr| addr.to_string())
        .unwrap_or_default()
        .into();

    TextField::new("addr_field", cx.focus_handle(), window, cx)
        .with_value(string_value, cx)
        .with_submit_validator(|text| text.parse::<dmx::Address>().is_ok(), cx)
}
