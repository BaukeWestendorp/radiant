mod container;
mod element;
mod input;
pub mod styled_ext;
pub mod theme;

mod grid;
mod utils;

pub use container::*;
pub use element::*;
pub use grid::*;
pub use input::*;
pub use utils::*;

pub fn init(cx: &mut gpui::App) {
    theme::Theme::init(cx);
    input::init(cx);
}

pub trait Disableable {
    fn disabled(self, disabled: bool) -> Self;
}
