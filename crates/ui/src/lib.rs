mod container;
mod element;
mod grid;
mod input;
mod styled_ext;
mod tabs_view;
mod theme;
mod toggle_button;

pub use container::*;
pub use element::*;
pub use grid::*;
pub use input::*;
pub use styled_ext::*;
pub use tabs_view::*;
pub use theme::*;
pub use toggle_button::*;

pub mod utils;

pub mod actions {
    use gpui::App;

    pub fn init(cx: &mut App) {
        super::input::actions::init(cx);
    }
}

pub fn init(cx: &mut gpui::App) {
    theme::Theme::init(cx);
}

pub fn root(cx: &mut gpui::App) -> gpui::Div {
    use gpui::Styled as _;
    gpui::div().text_color(cx.theme().text_primary)
}

pub trait Disableable {
    fn disabled(self, disabled: bool) -> Self;
}
