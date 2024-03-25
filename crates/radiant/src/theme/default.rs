use super::colors::ThemeColors;
use super::Theme;

#[derive(Debug, Clone)]
pub struct ThemeStyles {
    pub colors: ThemeColors,
}

fn radiant_dark() -> Theme {
    Theme {
        styles: ThemeStyles {
            colors: ThemeColors {
                border: gpui::rgb(0x1e232b).into(),
                border_variant: gpui::rgb(0x181c22).into(),
                border_focused: gpui::rgb(0x2d3440).into(),
                border_selected: gpui::rgb(0xff7931).into(),
                border_disabled: gpui::rgb(0x111317).into(),
                background: gpui::rgb(0x0d1017).into(),
                surface_background: gpui::rgb(0x0b0e14).into(),
                elevated_surface_background: gpui::rgb(0x141823).into(),
                text: gpui::rgb(0xffffff).into(),
                text_muted: gpui::rgb(0xbbbbbb).into(),
                text_placeholder: gpui::rgb(0x808080).into(),
                text_disabled: gpui::rgb(0x606060).into(),
                text_accent: gpui::rgb(0xff7931).into(),
            },
        },
    }
}

impl Default for Theme {
    fn default() -> Self {
        radiant_dark()
    }
}
