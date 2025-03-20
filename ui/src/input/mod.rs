mod text_field;

pub use text_field::*;

pub fn init(cx: &mut gpui::App) {
    text_field::init(cx);
}
