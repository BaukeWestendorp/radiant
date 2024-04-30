pub mod button;
pub mod picker;
pub mod slider;

pub use button::*;
pub use picker::*;
pub use slider::*;

pub trait Selectable {
    fn selected(self, selected: bool) -> Self;
}

pub trait Disableable {
    fn disabled(self, disabled: bool) -> Self;
}
