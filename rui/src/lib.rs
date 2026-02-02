mod root;

pub use root::Root;

pub fn init(cx: &mut gpui::App) {
    root::action::init(cx);
}
