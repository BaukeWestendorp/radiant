pub mod draggable;
pub mod event;
pub mod input;
pub mod modal;
pub mod pannable;
pub mod table;

pub(crate) fn init(cx: &mut gpui::App) {
    input::init(cx);
    modal::actions::init(cx);
    table::actions::init(cx);
}
