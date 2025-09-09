use gpui::{Hsla, rgb};

pub struct ThemeColors {
    /// Used for accents.
    pub accent: Hsla,
    /// Used for accent text color.
    pub accent_foreground: Hsla,
    /// Used for hovered accent text color.
    pub accent_foreground_hover: Hsla,

    /// Default background color.
    pub background: Hsla,

    /// Default border color.
    pub border: Hsla,

    /// Input background color.
    pub input: Hsla,
    /// Input secondary background color.
    pub input_secondary: Hsla,
    /// Input border color.
    pub input_border: Hsla,

    /// Input cursor color.
    pub cursor: Hsla,

    /// Default text color.
    pub foreground: Hsla,

    /// Link text color.
    pub link: Hsla,
    /// Active link text color.
    pub link_active: Hsla,
    /// Hover link text color.
    pub link_hover: Hsla,

    /// Modal background color.
    pub modal: Hsla,
    /// Modal border color.
    pub modal_border: Hsla,

    /// Muted background color.
    pub muted: Hsla,
    /// Muted text color.
    pub muted_foreground: Hsla,

    /// Used for focus ring.
    pub focus: Hsla,

    /// Secondary background color.
    pub secondary: Hsla,
    /// Active secondary background color.
    pub secondary_active: Hsla,
    /// Secondary text color, used for secondary button text color or secondary
    /// text.
    pub secondary_foreground: Hsla,
    /// Hover secondary background color.
    pub secondary_hover: Hsla,

    /// Input selection background color.
    pub selected: Hsla,
    /// Input selection background color.
    pub selected_border: Hsla,

    /// Table background color.
    pub table: Hsla,
    /// Stripe background color for even table row.
    pub table_even: Hsla,
    /// Table header background color.
    pub table_header: Hsla,
    /// Table header hover color.
    pub table_header_hover: Hsla,
    /// Table header text color.
    pub table_header_foreground: Hsla,
    /// Table header border color.
    pub table_header_border: Hsla,
    /// Table item hover background color.
    pub table_row_hover: Hsla,
    /// Table row border color.
    pub table_row_border: Hsla,
    /// Title bar background color, use for window title bar.
    pub title_bar: Hsla,
    /// Title bar border color.
    pub title_bar_border: Hsla,

    pub red: Hsla,
    pub green: Hsla,
    pub blue: Hsla,
    pub yellow: Hsla,
    pub magenta: Hsla,
    pub cyan: Hsla,
}

impl ThemeColors {
    pub fn dark() -> Self {
        Self {
            accent: rgb(0xee5622).into(),
            accent_foreground: rgb(0xee5622).into(),
            accent_foreground_hover: rgb(0xf16d41).into(),

            background: rgb(0x000000).into(),

            border: rgb(0x202020).into(),

            input: rgb(0x101010).into(),
            input_border: rgb(0x202020).into(),
            input_secondary: rgb(0x303030).into(),

            cursor: rgb(0xee5622).into(),

            foreground: rgb(0xffffff).into(),

            link: rgb(0xee5622).into(),
            link_active: Hsla::from(rgb(0xee5622)).lighten(0.2).into(),
            link_hover: Hsla::from(rgb(0xee5622)).with_opacity(0.8),

            modal: rgb(0x202020).into(),
            modal_border: rgb(0x282828).into(),

            muted: rgb(0x253340).into(),
            muted_foreground: rgb(0x626a73).into(),

            focus: Hsla::from(rgb(0x59c2ff)).with_opacity(0.5),

            secondary: rgb(0x101010).into(),
            secondary_active: Hsla::from(rgb(0x101010)).with_opacity(0.8),
            secondary_foreground: rgb(0xffffff).into(),
            secondary_hover: Hsla::from(rgb(0x101010)).with_opacity(0.6),

            selected: Hsla::from(rgb(0xee5622)).with_opacity(0.4),
            selected_border: rgb(0xee5622).into(),

            table: rgb(0x000000).into(),
            table_even: rgb(0x141414).into(),
            table_header: rgb(0x242424).into(),
            table_header_hover: rgb(0x303030).into(),
            table_header_foreground: rgb(0xffffff).into(),
            table_header_border: rgb(0x383838).into(),
            table_row_hover: rgb(0x202020).into(),
            table_row_border: rgb(0x242424).into(),

            title_bar: rgb(0x101010).into(),
            title_bar_border: rgb(0x202020).into(),

            red: rgb(0xff4040).into(),
            green: rgb(0xb8cc52).into(),
            blue: rgb(0x59c2ff).into(),
            yellow: rgb(0xee5622).into(),
            magenta: rgb(0xae81ff).into(),
            cyan: rgb(0x39bae6).into(),
        }
    }
}

impl Default for ThemeColors {
    fn default() -> Self {
        Self::dark()
    }
}

pub trait InteractiveColor {
    fn with_opacity(&self, factor: f32) -> Self;

    fn lighten(&self, factor: f32) -> Self;

    fn darken(&self, factor: f32) -> Self;
}

impl InteractiveColor for Hsla {
    fn with_opacity(&self, opacity: f32) -> Self {
        Self { a: opacity, ..*self }
    }

    fn lighten(&self, factor: f32) -> Self {
        let l = match self.l {
            0.0 => factor,
            l => l * (1.0 + factor),
        };
        Hsla { l, ..*self }
    }

    fn darken(&self, factor: f32) -> Self {
        let l = self.l * (1.0 - factor);
        Self { l, ..*self }
    }
}
