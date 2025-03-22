use gpui::{Hsla, Pixels};

pub struct Theme {
    pub background: Hsla,

    pub element_background: Hsla,
    pub element_background_focused: Hsla,
    pub element_background_hover: Hsla,
    pub element_background_muted: Hsla,

    pub text_primary: Hsla,
    pub text_muted: Hsla,

    pub radius: Pixels,
    pub border: Hsla,
    pub border_muted: Hsla,
    pub border_focused: Hsla,
    pub border_muted_focused: Hsla,

    pub accent: Hsla,

    pub cursor: Hsla,
    pub cursor_width: Pixels,
    pub highlight: Hsla,

    pub dot_grid_color: Hsla,
    pub line_grid_color: Hsla,

    pub input_height: Pixels,
    pub input_slider_bar_color: Hsla,
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

            element_background: gpui::hsla(0.0, 0.0, 0.0, 1.0),
            element_background_focused: gpui::hsla(0.0, 0.0, 0.1, 1.0),
            element_background_hover: gpui::hsla(0.0, 0.0, 0.075, 1.0),
            element_background_muted: gpui::hsla(0.0, 0.0, 0.0, 1.0),

            text_primary: gpui::hsla(0.0, 0.0, 1.0, 1.0),
            text_muted: gpui::hsla(0.0, 0.0, 0.75, 1.0),

            radius: gpui::px(4.0),
            border: gpui::hsla(0.0, 0.0, 0.5, 1.0),
            border_muted: gpui::hsla(0.0, 0.0, 0.50, 1.0),
            border_focused: gpui::rgb(0xffc416).into(),
            border_muted_focused: gpui::rgb(0x6d5b25).into(),

            accent: gpui::rgb(0xffc416).into(),

            cursor: gpui::rgb(0xffc416).into(),
            cursor_width: gpui::px(1.0),
            highlight: gpui::rgba(0xffc41640).into(),

            dot_grid_color: gpui::rgba(0xffc41680).into(),
            line_grid_color: gpui::hsla(0.0, 0.0, 0.15, 1.0),

            input_height: gpui::px(28.0),
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
