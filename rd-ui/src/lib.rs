mod app;
mod asset;
mod keymap;
mod popup;
mod root;
mod settings;
mod theme;
mod util;

pub mod comp;

pub use app::*;
pub use asset::*;
pub use keymap::*;
pub use popup::*;
pub use root::*;
pub use settings::*;
pub use theme::*;
pub use util::*;

pub fn init(cx: &mut gpui::App) {
    theme::init(cx);
    settings::init(cx);
    root::init(cx);
    popup::init(cx);
}

pub use ::gpui;
