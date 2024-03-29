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
                border: gpui::rgb(0xa77c00).into(),
                border_variant: gpui::rgb(0x404040).into(),
                border_focused: gpui::rgb(0xffffff).into(),
                border_selected: gpui::rgb(0x00ff00).into(),
                border_disabled: gpui::rgb(0x404040).into(),
                window_header: gpui::rgb(0x1010b0).into(),
                window_header_border: gpui::rgb(0x0000ff).into(),
                window_background: gpui::rgb(0x101010).into(),
                background: gpui::rgb(0x242424).into(),
                text: gpui::rgb(0xffffff).into(),
                text_muted: gpui::rgb(0xbbbbbb).into(),
                text_placeholder: gpui::rgb(0x808080).into(),
                text_disabled: gpui::rgb(0x606060).into(),
                text_accent: gpui::rgb(0xff7931).into(),
                element_background: gpui::rgb(0x000000).into(),
                element_background_hover: gpui::rgb(0x202020).into(),
                element_background_active: gpui::rgb(0x1a212f).into(),
                element_background_selected: gpui::rgb(0x000080).into(),
                element_background_disabled: gpui::rgb(0x070709).into(),
                programmer_change: gpui::rgb(0xc90000).into(),
            },
        },
    }
}

impl Default for Theme {
    fn default() -> Self {
        radiant_dark()
    }
}
