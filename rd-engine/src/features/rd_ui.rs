use crate::{
    dmx::{Address, Channel, UniverseId},
    patch::FixtureIdPart,
};
use rd_ui::{
    Button, FieldValue, Icon, IconSize, IconVariant, Popup, PopupAppExt,
    gpui::{App, IntoElement, ParentElement, SharedString, Styled, Window},
    h_flex,
};

impl FieldValue for FixtureIdPart {
    fn from_str(s: &str) -> Option<Self> {
        s.parse().ok()
    }

    fn to_shared_string(&self) -> impl Into<SharedString> {
        self.to_string()
    }

    fn validator(s: &str) -> bool {
        Self::from_str(s).is_some()
    }

    fn submit_validator(s: &str) -> bool {
        Self::from_str(s).is_some()
    }
}

impl FieldValue for Address {
    fn from_str(s: &str) -> Option<Self> {
        if !s.contains('.') {
            let single_channel_format = format!("1.{s}");
            return single_channel_format.parse().ok();
        }

        s.parse().ok()
    }

    fn to_shared_string(&self) -> impl Into<SharedString> {
        self.to_string()
    }

    fn validator(s: &str) -> bool {
        use std::str::FromStr;

        if s.is_empty() {
            return true;
        }

        let mut parts = s.split('.');
        let first = parts.next().unwrap_or("");
        let second = parts.next();

        if parts.next().is_some() {
            return false;
        }

        if let Some(channel_str) = second {
            let valid_universe = UniverseId::from_str(first).is_ok();
            let valid_channel = channel_str.is_empty() || Channel::from_str(channel_str).is_ok();

            valid_universe && valid_channel
        } else {
            UniverseId::from_str(first).is_ok() || Channel::from_str(first).is_ok()
        }
    }

    fn submit_validator(s: &str) -> bool {
        Self::from_str(s).is_some()
    }

    fn render_overlay(_window: &mut Window, _cx: &mut App) -> Option<impl IntoElement> {
        Some(
            h_flex().size_full().justify_end().px_1().child(
                Button::new("open-picker")
                    .icon(Icon::new(IconVariant::TableCellsMerge, IconSize::ExtraSmall))
                    .on_click(|_, window, cx| {
                        cx.open_popup(window, |_, _| {
                            Popup::message("Select an Address", "FIXME: Implement address selector")
                        });
                    }),
            ),
        )
    }
}
