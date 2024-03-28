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
    /// Border color. Used for active elements.
    pub border_active: Hsla,
    /// Border color. Used for selected elements, like an active search filter
    /// or selected checkbox.
    pub border_selected: Hsla,
    /// Border color. Used for disabled elements, like a disabled input or
    /// button.
    pub border_disabled: Hsla,

    /// Background Color. Used for the app background and blank panels or
    /// windows.
    pub background: Hsla,

    /// Background Color. Used for the background of an element that should have
    /// a different background than the surface it's on.
    ///
    /// Elements might include: Buttons, Inputs, Checkboxes, Radio Buttons...
    ///
    /// For an element that should have the same background as the surface it's
    /// on, use `ghost_element_background`.
    pub element_background: Hsla,
    /// Background Color. Used for the hover state of an element that should
    /// have a different background than the surface it's on.
    ///
    /// Hover states are triggered by the mouse entering an element, or a finger
    /// touching an element on a touch screen.
    pub element_background_hover: Hsla,
    /// Background Color. Used for the active state of an element that should
    /// have a different background than the surface it's on.
    ///
    /// Active states are triggered by the mouse button being pressed down on an
    /// element, or the Return button or other activator being pressd.
    pub element_background_active: Hsla,
    /// Background Color. Used for the selected state of an element that should
    /// have a different background than the surface it's on.
    ///
    /// Selected states are triggered by the element being selected (or
    /// "activated") by the user.
    ///
    /// This could include a selected checkbox, a toggleable button that is
    /// toggled on, etc.
    pub element_background_selected: Hsla,
    /// Background Color. Used for the disabled state of an element that should
    /// have a different background than the surface it's on.
    ///
    /// Disabled states are shown when a user cannot interact with an element,
    /// like a disabled button or input.
    pub element_background_disabled: Hsla,

    /// Window Header Color. Used for the color of the headers of windows in
    /// the window grid.
    pub window_header: Hsla,
    /// Window Header Border Color. Used for the color of the borders of the
    /// headers of windows in the window grid.
    pub window_header_border: Hsla,
    /// Window Background Color. Used for the color of the background of
    /// windows.
    pub window_background: Hsla,

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

    /// The indicator color for when a value changed in the programmer.
    pub programmer_change: Hsla,
}
