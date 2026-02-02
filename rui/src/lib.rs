mod root;
mod theme;

pub use root::Root;
pub use theme::ActiveTheme;

pub fn init(cx: &mut gpui::App) {
    root::action::init(cx);
    theme::init(cx);
}
