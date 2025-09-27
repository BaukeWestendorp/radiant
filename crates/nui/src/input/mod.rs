mod checkbox;
mod number_field;
mod text_field;
mod text_input;

pub use checkbox::*;
pub use number_field::*;
pub use text_field::*;
pub use text_input::*;

pub(crate) fn init(cx: &mut gpui::App) {
    text_input::init(cx);
}
