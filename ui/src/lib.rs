mod container;
mod element;
mod grid;
mod input;
mod styled_ext;
mod theme;

pub mod utils;

pub use container::*;
pub use element::*;
pub use grid::*;
pub use input::*;
pub use styled_ext::*;
pub use theme::*;

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
