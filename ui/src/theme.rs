use gpui::{Hsla, Pixels};

pub struct Theme {
    pub background: Hsla,
    pub background_focused: Hsla,

    pub text_primary: Hsla,
    pub text_muted: Hsla,

    pub radius: Pixels,
    pub border_color: Hsla,
    pub border_color_muted: Hsla,
    pub border_color_focused: Hsla,

    pub accent: Hsla,

    pub cursor: Hsla,
    pub cursor_width: Pixels,
    pub highlight: Hsla,

    pub dot_grid_color: Hsla,
    pub line_grid_color: Hsla,
}

impl Theme {
    pub fn init(cx: &mut gpui::App) {
        cx.set_global(Theme::default());
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            background: gpui::hsla(0.0, 0.0, 0.0, 1.0),
            background_focused: gpui::hsla(0.0, 0.0, 0.1, 1.0),

            text_primary: gpui::hsla(0.0, 0.0, 1.0, 1.0),
            text_muted: gpui::hsla(0.0, 0.0, 0.75, 1.0),

            radius: gpui::px(4.0),
            border_color: gpui::hsla(0.0, 0.0, 0.5, 1.0),
            border_color_muted: gpui::hsla(0.0, 0.0, 0.50, 1.0),
            border_color_focused: gpui::rgb(0xffc416).into(),

            accent: gpui::rgb(0xffc416).into(),

            cursor: gpui::rgb(0xffc416).into(),
            cursor_width: gpui::px(1.0),
            highlight: gpui::rgba(0xffc41640).into(),

            dot_grid_color: gpui::rgba(0xffc41680).into(),
            line_grid_color: gpui::hsla(0.0, 0.0, 0.15, 1.0),
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
