use rd_artnet::PortAddress;
use rd_ui::{
    comp::stateful::Field,
    gpui::{Context, ElementId, Window},
};
use std::str::FromStr as _;

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
