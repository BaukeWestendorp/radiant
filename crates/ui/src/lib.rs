mod element;
mod input;
mod misc;
mod nav;
mod org;
mod theme;
mod typo;

pub mod utils;

pub use element::*;
pub use input::*;
pub use misc::*;
pub use nav::*;
pub use org::*;
pub use theme::*;
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
