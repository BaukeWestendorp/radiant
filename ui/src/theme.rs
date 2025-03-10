use gpui::{Hsla, Pixels};

pub struct Theme {
    pub background: Hsla,
    pub text_primary: Hsla,

    pub radius: Pixels,
    pub border_color: Hsla,
    pub border_color_muted: Hsla,
    pub border_color_focused: Hsla,

    pub accent: Hsla,

    pub grid_color: Hsla,
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
            text_primary: gpui::hsla(0.0, 0.0, 1.0, 1.0),

            radius: gpui::px(4.0),
            border_color: gpui::hsla(0.0, 0.0, 0.5, 1.0),
            border_color_muted: gpui::hsla(0.0, 0.0, 0.15, 1.0),
            border_color_focused: gpui::rgb(0xffc416).into(),

            accent: gpui::rgb(0xffc416).into(),

            grid_color: gpui::rgba(0xffc41680).into(),
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
