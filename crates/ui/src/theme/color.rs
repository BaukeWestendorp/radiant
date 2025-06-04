use gpui::Hsla;

pub struct Colors {
    pub bg_primary: Hsla,
    pub bg_secondary: Hsla,
    pub bg_tertiary: Hsla,
    pub bg_selected: Hsla,
    pub bg_selected_bright: Hsla,
    pub bg_focused: Hsla,
    pub bg_alternating: Hsla,
    pub bg_destructive: Hsla,
    pub bg_destructive_focused: Hsla,
    pub bg_destructive_selected: Hsla,

    pub text: Hsla,

    pub border: Hsla,
    pub border_focused: Hsla,
    pub border_selected: Hsla,
    pub border_destructive: Hsla,
    pub border_destructive_focused: Hsla,
    pub border_destructive_selected: Hsla,

    pub header_background: Hsla,
    pub header_border: Hsla,

    pub accent: Hsla,
    pub highlight: Hsla,
    pub cursor: Hsla,

    pub grid: Hsla,
}

impl Colors {
    pub fn light() -> Self {
        let accent: Hsla = gpui::rgb(0xffc416).into();
        let selected = accent;
        let destructive: Hsla = gpui::rgb(0xff4d4d).into();

        Self {
            bg_primary: gpui::hsla(0.0, 0.0, 1.0, 1.0),
            bg_secondary: gpui::hsla(0.0, 0.0, 0.9, 1.0),
            bg_tertiary: gpui::hsla(0.0, 0.0, 0.8, 1.0),
            bg_focused: gpui::hsla(0.0, 0.0, 0.85, 1.0),
            bg_selected: selected.darken(0.8),
            bg_selected_bright: selected.lighten(0.7),
            bg_alternating: gpui::hsla(0.0, 0.0, 0.95, 1.0),
            bg_destructive: destructive.lighten(0.2),
            bg_destructive_focused: destructive.lighten(0.2),
            bg_destructive_selected: destructive.lighten(0.1),

            text: gpui::hsla(0.0, 0.0, 0.0, 1.0),

            header_background: gpui::rgb(0xe0e4ff).into(),
            header_border: gpui::rgb(0xc0c4ff).into(),

            border: gpui::hsla(0.0, 0.0, 0.7, 1.0),
            border_focused: accent,
            border_selected: selected,
            border_destructive: destructive.lighten(0.2),
            border_destructive_focused: destructive,
            border_destructive_selected: destructive.darken(0.1),

            accent,
            highlight: accent.with_opacity(0.2),
            cursor: accent,

            grid: gpui::rgba(0xffc41680).into(),
        }
    }

    pub fn dark() -> Self {
        let accent: Hsla = gpui::rgb(0xffc416).into();
        let selected = accent;
        let destructive: Hsla = gpui::rgb(0xff4d4d).into();

        Self {
            bg_primary: gpui::hsla(0.0, 0.0, 0.0, 1.0),
            bg_secondary: gpui::hsla(0.0, 0.0, 0.0, 1.0),
            bg_tertiary: gpui::hsla(0.0, 0.0, 0.2, 1.0),
            bg_focused: gpui::hsla(0.0, 0.0, 0.1, 1.0),
            bg_selected: selected.darken(0.9),
            bg_selected_bright: selected.darken(0.2),
            bg_alternating: gpui::hsla(0.0, 0.0, 0.1, 1.0),
            bg_destructive: destructive.darken(0.8),
            bg_destructive_focused: destructive.darken(0.4),
            bg_destructive_selected: destructive.darken(0.2),

            text: gpui::hsla(0.0, 0.0, 1.0, 1.0),

            header_background: gpui::rgb(0x1317a0).into(),
            header_border: gpui::rgb(0x383eed).into(),

            border: gpui::hsla(0.0, 0.0, 0.5, 1.0),
            border_focused: accent,
            border_selected: selected,
            border_destructive: destructive.darken(0.2),
            border_destructive_focused: destructive,
            border_destructive_selected: destructive.lighten(0.1),

            accent,
            highlight: accent.with_opacity(0.2),
            cursor: accent,

            grid: gpui::rgba(0xffc41680).into(),
        }
    }
}

impl Default for Colors {
    fn default() -> Self {
        Self::dark()
    }
}

pub trait InteractiveColor {
    fn hovered(self) -> Self;

    fn muted(self) -> Self;

    fn active(self) -> Self;

    fn with_opacity(&self, factor: f32) -> Self;

    fn lighten(&self, factor: f32) -> Self;

    fn darken(&self, factor: f32) -> Self;
}

impl InteractiveColor for Hsla {
    fn hovered(self) -> Self {
        self.lighten(0.1).with_opacity(self.a + 0.1)
    }

    fn muted(self) -> Self {
        self.darken(0.4)
    }

    fn active(self) -> Self {
        self.lighten(0.2).with_opacity(self.a + 0.2)
    }

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
