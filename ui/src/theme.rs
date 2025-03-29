use gpui::{Hsla, Pixels};

pub struct Theme {
    pub background: Hsla,

    pub element_background: Hsla,
    pub element_background_focused: Hsla,
    pub element_background_selected: Hsla,

    pub text_primary: Hsla,

    pub radius: Pixels,
    pub border: Hsla,
    pub border_focused: Hsla,
    pub border_selected: Hsla,

    pub accent: Hsla,

    pub cursor: Hsla,
    pub cursor_width: Pixels,
    pub highlight: Hsla,

    pub dot_grid_color: Hsla,
    pub line_grid_color: Hsla,

    pub input_slider_bar_color: Hsla,
}

impl Theme {
    pub fn init(cx: &mut gpui::App) {
        cx.set_global(Theme::default());
    }
}

impl Default for Theme {
    fn default() -> Self {
        let accent: Hsla = gpui::rgb(0xffc416).into();
        let selected = accent;

        Self {
            background: gpui::hsla(0.0, 0.0, 0.0, 1.0),

            element_background: gpui::hsla(0.0, 0.0, 0.0, 1.0),
            element_background_focused: gpui::hsla(0.0, 0.0, 0.1, 1.0),
            element_background_selected: selected.darken(0.5).with_opacity(0.1),

            text_primary: gpui::hsla(0.0, 0.0, 1.0, 1.0),

            radius: gpui::px(4.0),
            border: gpui::hsla(0.0, 0.0, 0.5, 1.0),
            border_focused: accent,
            border_selected: selected,

            accent,

            cursor: accent,
            cursor_width: gpui::px(1.0),
            highlight: accent.with_opacity(0.2),

            dot_grid_color: gpui::rgba(0xffc41680).into(),
            line_grid_color: gpui::hsla(0.0, 0.0, 0.15, 1.0),

            input_slider_bar_color: gpui::hsla(0.0, 0.0, 0.2, 1.0),
        }
    }
}

impl gpui::Global for Theme {}

pub trait ActiveTheme {
    fn theme(&self) -> &Theme;
}

impl<E> ActiveTheme for gpui::Context<'_, E> {
    fn theme(&self) -> &Theme {
        self.global()
    }
}

impl ActiveTheme for gpui::App {
    fn theme(&self) -> &Theme {
        self.global()
    }
}

pub trait InteractiveColor {
    fn hovered(self) -> Self;

    fn muted(self) -> Self;

    fn with_opacity(&self, factor: f32) -> Self;

    fn lighten(&self, factor: f32) -> Self;

    fn darken(&self, factor: f32) -> Self;
}

impl InteractiveColor for Hsla {
    fn hovered(self) -> Self {
        self.lighten(0.1).with_opacity(self.a + 0.1)
    }

    fn muted(self) -> Self {
        self.darken(0.1)
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
