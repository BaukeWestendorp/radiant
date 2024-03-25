use gpui::Hsla;

#[derive(Debug, Clone)]
pub struct ThemeColors {
    /// Border color. Used for most borders, is usually a high contrast color.
    pub border: Hsla,
    /// Border color. Used for deemphasized borders, like a visual divider
    /// between two sections
    pub border_variant: Hsla,
    /// Border color. Used for focused elements, like keyboard focused list
    /// item.
    pub border_focused: Hsla,
    /// Border color. Used for selected elements, like an active search filter
    /// or selected checkbox.
    pub border_selected: Hsla,
    /// Border color. Used for disabled elements, like a disabled input or
    /// button.
    pub border_disabled: Hsla,

    /// Background Color. Used for the app background and blank panels or
    /// windows.
    pub background: Hsla,
    /// Background Color. Used for grounded surfaces like a panel or tab.
    pub surface_background: Hsla,
    /// Border color. Used for elevated surfaces, like a context menu, popup, or
    /// dialog.
    pub elevated_surface_background: Hsla,
    /// Background Color. Used for elements that are selected.
    pub background_selected: Hsla,

    /// Windoww Header Color. Used for the color of the headers of windows in
    /// the window grid.
    pub window_header: Hsla,
    /// Window Header Border Color. Used for the color of the borders of the
    /// headers of windows in the window grid.
    pub window_header_border: Hsla,

    /// Text Color. Default text color used for most text.
    pub text: Hsla,
    /// Text Color. Color of muted or deemphasized text. It is a subdued version
    /// of the standard text color.
    pub text_muted: Hsla,
    /// Text Color. Color of the placeholder text typically shown in input
    /// fields to guide the user to enter valid data.
    pub text_placeholder: Hsla,
    /// Text Color. Color used for text denoting disabled elements. Typically,
    /// the color is faded or grayed out to emphasize the disabled state.
    pub text_disabled: Hsla,
    /// Text Color. Color used for emphasis or highlighting certain text, like
    /// an active filter or a matched character in a search.
    pub text_accent: Hsla,
}
