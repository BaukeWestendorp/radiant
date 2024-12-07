use std::ops::Deref;

use gpui::*;

pub trait ActiveTheme {
    fn theme(&self) -> &Theme;
}

impl ActiveTheme for AppContext {
    fn theme(&self) -> &Theme {
        self.global()
    }
}

impl<'a, V> ActiveTheme for ViewContext<'a, V> {
    fn theme(&self) -> &Theme {
        self.deref().theme()
    }
}

impl<'a, V> ActiveTheme for ModelContext<'a, V> {
    fn theme(&self) -> &Theme {
        self.deref().theme()
    }
}

impl<'a> ActiveTheme for WindowContext<'a> {
    fn theme(&self) -> &Theme {
        self.deref().theme()
    }
}

/// Make a [Hsla] color.
///
/// - h: 0..360.0
/// - s: 0.0..100.0
/// - l: 0.0..100.0
pub fn hsl(h: f32, s: f32, l: f32) -> Hsla {
    hsla(h / 360., s / 100.0, l / 100.0, 1.0)
}

pub trait Colorize {
    fn opacity(&self, opacity: f32) -> Hsla;
    fn divide(&self, divisor: f32) -> Hsla;
    fn invert(&self) -> Hsla;
    fn invert_l(&self) -> Hsla;
    fn lighten(&self, amount: f32) -> Hsla;
    fn darken(&self, amount: f32) -> Hsla;
}

impl Colorize for Hsla {
    /// Returns a new color with the given opacity.
    ///
    /// The opacity is a value between 0.0 and 1.0, where 0.0 is fully transparent and 1.0 is fully opaque.
    fn opacity(&self, factor: f32) -> Hsla {
        Hsla {
            a: self.a * factor.clamp(0.0, 1.0),
            ..*self
        }
    }

    /// Returns a new color with each channel divided by the given divisor.
    ///
    /// The divisor in range of 0.0 .. 1.0
    fn divide(&self, divisor: f32) -> Hsla {
        Hsla {
            a: divisor,
            ..*self
        }
    }

    /// Return inverted color
    fn invert(&self) -> Hsla {
        Hsla {
            h: (self.h + 1.8) % 3.6,
            s: 1.0 - self.s,
            l: 1.0 - self.l,
            a: self.a,
        }
    }

    /// Return inverted lightness
    fn invert_l(&self) -> Hsla {
        Hsla {
            l: 1.0 - self.l,
            ..*self
        }
    }

    /// Return a new color with the lightness increased by the given factor.
    fn lighten(&self, factor: f32) -> Hsla {
        let l = (self.l * (1.0 + factor.clamp(0.0, 1.0))).min(1.0);
        Hsla { l, ..*self }
    }

    /// Return a new color with the darkness increased by the given factor.
    fn darken(&self, factor: f32) -> Hsla {
        let l = (self.l * (1.0 - factor.clamp(0.0, 1.0))).max(0.0);
        Hsla { l, ..*self }
    }
}

pub struct Theme {
    // FIXME: These should not be in the theme.
    pub font_size: Pixels,
    pub font_family: SharedString,
    pub radius: Pixels,
    pub input_height: Pixels,
    pub cursor_width: Pixels,
    pub cursor_blink_interval_ms: u64,
    pub cursor_blink_pause_delay_ms: u64,

    /// Accent color. Used as a highlight color.
    pub accent: Hsla,

    /// Border color. Used for most borders, is usually a high contrast color.
    pub border: Hsla,
    /// Border color. Used for deemphasized borders, like a visual divider between two sections
    pub border_variant: Hsla,
    /// Border color. Used for focused elements, like keyboard focused list item.
    pub border_focused: Hsla,
    /// Border color. Used for selected elements, like an active search filter or selected checkbox.
    pub border_selected: Hsla,
    /// Border color. Used for transparent borders. Used for placeholder borders when an element gains a border on state change.
    pub border_transparent: Hsla,
    /// Border color. Used for disabled elements, like a disabled input or button.
    pub border_disabled: Hsla,
    /// Border color. Used for elevated surfaces, like a context menu, popup, or dialog.
    pub elevated_surface_background: Hsla,
    /// Background Color. Used for grounded surfaces like a panel or tab.
    pub surface_background: Hsla,
    /// Background Color. Used for the app background and blank panels or windows.
    pub background: Hsla,
    /// Background Color. Used for the background of an element that should have a different background than the surface it's on.
    ///
    /// Elements might include: Buttons, Inputs, Checkboxes, Radio Buttons...
    ///
    /// For an element that should have the same background as the surface it's on, use `ghost_element_background`.
    pub element_background: Hsla,
    /// Background Color. Used for the hover state of an element that should have a different background than the surface it's on.
    ///
    /// Hover states are triggered by the mouse entering an element, or a finger touching an element on a touch screen.
    pub element_hover: Hsla,
    /// Background Color. Used for the active state of an element that should have a different background than the surface it's on.
    ///
    /// Active states are triggered by the mouse button being pressed down on an element, or the Return button or other activator being pressd.
    pub element_active: Hsla,
    /// Background Color. Used for the selected state of an element that should have a different background than the surface it's on.
    ///
    /// Selected states are triggered by the element being selected (or "activated") by the user.
    ///
    /// This could include a selected checkbox, a toggleable button that is toggled on, etc.
    pub element_selected: Hsla,
    /// Background Color. Used for the disabled state of an element that should have a different background than the surface it's on.
    ///
    /// Disabled states are shown when a user cannot interact with an element, like a disabled button or input.
    pub element_disabled: Hsla,

    /// Text Color. Default text color used for most text.
    pub text: Hsla,
    /// Text Color. Color of muted or deemphasized text. It is a subdued version of the standard text color.
    pub text_muted: Hsla,
    /// Text Color. Color of the placeholder text typically shown in input fields to guide the user to enter valid data.
    pub text_placeholder: Hsla,
    /// Text Color. Color used for text denoting disabled elements. Typically, the color is faded or grayed out to emphasize the disabled state.
    pub text_disabled: Hsla,
    /// Text Color. Color used for emphasis or highlighting certain text, like an active filter or a matched character in a search.
    pub text_accent: Hsla,
    /// Fill Color. Used for the default fill color of an icon.
    pub icon: Hsla,
    /// Fill Color. Used for the muted or deemphasized fill color of an icon.
    ///
    /// This might be used to show an icon in an inactive pane, or to demphasize a series of icons to give them less visual weight.
    pub icon_muted: Hsla,
    /// Fill Color. Used for the disabled fill color of an icon.
    ///
    /// Disabled states are shown when a user cannot interact with an element, like a icon button.
    pub icon_disabled: Hsla,
    /// Fill Color. Used for the placeholder fill color of an icon.
    ///
    /// This might be used to show an icon in an input that disappears when the user enters text.
    pub icon_placeholder: Hsla,
    /// Fill Color. Used for the accent fill color of an icon.
    ///
    /// This might be used to show when a toggleable icon button is selected.
    pub icon_accent: Hsla,

    /// Background color of a frame header.
    pub frame_header_background: Hsla,
    /// Border color of a frame header.
    pub frame_header_border: Hsla,
    /// Text color of a frame header.
    pub frame_header_text_color: Hsla,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            // General
            radius: px(4.0),
            input_height: px(22.0),
            cursor_width: px(2.0),
            cursor_blink_interval_ms: 750,
            cursor_blink_pause_delay_ms: 500,

            // Fonts
            font_size: px(12.0),
            font_family: "IBM Plex Mono".into(),

            // Colors
            accent: hsl(44.0, 98.0, 50.0),
            border: hsl(210.0, 15.0, 70.0),
            border_variant: hsl(210.0, 15.0, 85.0),
            border_focused: hsl(44.0, 98.0, 50.0),
            border_selected: hsl(44.0, 98.0, 50.0),
            border_transparent: hsla(0.0, 0.0, 0.0, 0.0),
            border_disabled: hsl(214.0, 32.0, 82.0),
            elevated_surface_background: hsl(210.0, 10.0, 95.0),
            surface_background: hsl(210.0, 10.0, 90.0),
            background: hsl(210.0, 15.0, 98.0),
            element_background: hsl(210.0, 15.0, 95.0),
            element_hover: hsl(210.0, 15.0, 97.0),
            element_active: hsl(210.0, 15.0, 100.0),
            element_selected: hsl(210.0, 15.0, 98.0),
            element_disabled: hsl(0.0, 0.0, 92.0),
            text: hsl(210.0, 15.0, 20.0),
            text_muted: hsl(210.0, 10.0, 50.0),
            text_placeholder: hsl(210.0, 10.0, 50.0),
            text_disabled: hsl(210.0, 10.0, 60.0),
            text_accent: hsl(44.0, 98.0, 50.0),
            icon: hsl(210.0, 15.0, 30.0),
            icon_muted: hsl(210.0, 10.0, 50.0),
            icon_disabled: hsl(210.0, 10.0, 60.0),
            icon_placeholder: hsl(210.0, 10.0, 50.0),
            icon_accent: hsl(44.0, 98.0, 50.0),

            frame_header_background: hsl(0.0, 0.0, 7.0),
            frame_header_border: hsl(0.0, 0.0, 0.0),
            frame_header_text_color: hsl(210.0, 10.0, 98.0),
        }
    }
}

impl Global for Theme {}
