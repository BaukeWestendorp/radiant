use gpui::Hsla;

#[derive(Debug, Clone)]
pub struct ThemeColors {
    pub background: Hsla,

    pub border: Hsla,
    pub border_variant: Hsla,
    pub border_selected: Hsla,

    pub element_background: Hsla,
    pub element_background_raised: Hsla,
    pub element_background_hover: Hsla,
    pub element_background_selected: Hsla,

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
