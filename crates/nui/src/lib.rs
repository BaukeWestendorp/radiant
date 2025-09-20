pub mod error;

mod app_ext;
mod wm;

pub use app_ext::*;
pub use wm::*;

use gpui::App;

pub fn init(cx: &mut App) {
    let wm = WindowManager::new(cx);
    cx.set_global(wm);
}
