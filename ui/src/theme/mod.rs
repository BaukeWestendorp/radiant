use gpui::Pixels;

mod color;

pub use color::*;

pub struct Theme {
    pub colors: Colors,

    pub radius: Pixels,

    pub cursor_width: Pixels,
}

impl Theme {
    pub fn init(cx: &mut gpui::App) {
        cx.set_global(Theme::default());
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self { colors: Colors::default(), radius: gpui::px(3.0), cursor_width: gpui::px(1.0) }
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
