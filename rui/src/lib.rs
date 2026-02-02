mod root;
mod styled_ext;
mod theme;
mod title_bar;

pub use root::Root;
pub use styled_ext::{StyledExt, h_flex, v_flex};
pub use theme::ActiveTheme;
pub use title_bar::TitleBar;

pub fn init(cx: &mut gpui::App) {
    root::action::init(cx);
    theme::init(cx);
}
