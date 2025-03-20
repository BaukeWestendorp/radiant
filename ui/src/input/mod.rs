mod text_input;

mod number_field;
mod text_field;

pub use text_input::*;

pub use number_field::*;
pub use text_field::*;

pub fn init(cx: &mut gpui::App) {
    text_input::init(cx);
}
