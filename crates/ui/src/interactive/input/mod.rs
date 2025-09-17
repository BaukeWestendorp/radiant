mod checkbox;
mod text_field;
// mod number_field;
mod text_input;

pub use checkbox::*;
pub use text_field::*;
// pub use number_field::*;
pub use text_input::*;

pub(super) fn init(cx: &mut gpui::App) {
    text_input::actions::init(cx);
}
