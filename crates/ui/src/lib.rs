mod element;
mod grid;
mod input;
mod styled_ext;
mod tabs_view;
mod theme;

mod org;
mod typo;
pub mod utils;

pub use element::*;
pub use grid::*;
pub use input::*;
pub use styled_ext::*;
pub use tabs_view::*;
pub use theme::*;

pub use org::*;
pub use typo::*;

pub mod actions {
    use gpui::App;

    pub fn init(cx: &mut App) {
        super::input::actions::init(cx);
    }
}

pub fn init(cx: &mut gpui::App) {
    theme::Theme::init(cx);
}

pub trait Disableable {
    fn disabled(self, disabled: bool) -> Self;
}

pub trait Selectable {
    fn selected(self, selected: bool) -> Self;
}
