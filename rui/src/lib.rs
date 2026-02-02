mod root;
mod theme;
mod title_bar;

pub use root::Root;
pub use theme::ActiveTheme;
pub use title_bar::TitleBar;

pub fn init(cx: &mut gpui::App) {
    root::action::init(cx);
    theme::init(cx);
}
