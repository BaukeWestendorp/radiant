use gpui::Hsla;

#[derive(Debug, Clone)]
pub struct ThemeColors {
    pub accent: Hsla,

    pub background: Hsla,

    pub border: Hsla,
    pub border_disabled: Hsla,
    pub border_selected: Hsla,

    pub element_background: Hsla,
    pub element_background_secondary: Hsla,
    pub element_background_hover: Hsla,
    pub element_background_hover_secondary: Hsla,
    pub element_background_selected: Hsla,
    pub element_background_active: Hsla,

    pub window_header: Hsla,
    pub window_header_border: Hsla,
    pub window_background: Hsla,

    pub text: Hsla,
    pub text_muted: Hsla,
    pub text_placeholder: Hsla,
    pub text_disabled: Hsla,
    pub text_accent: Hsla,

    pub programmer_change: Hsla,
}
