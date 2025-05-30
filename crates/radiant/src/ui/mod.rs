//! UI elements specificaly for Radiant.

mod asset_table;
mod input;
mod vw;

pub use asset_table::*;
pub use input::*;
pub use vw::*;

pub const FRAME_CELL_SIZE: gpui::Pixels = gpui::px(80.0);

pub fn init(cx: &mut gpui::App) {
    input::init(cx);
}
