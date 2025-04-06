mod text_input;

mod number_field;
mod text_field;

pub use text_input::*;

pub use number_field::*;
pub use text_field::*;

pub mod actions {
    use gpui::App;

    pub fn init(cx: &mut App) {
        super::text_input::actions::init(cx);
    }
}
