pub mod assets;
pub mod button;
pub mod container;
pub mod divider;
pub mod error;
pub mod event;
pub mod grid;
pub mod infobar;
pub mod input;
pub mod section;
pub mod table;
pub mod tabs;
pub mod theme;
pub mod typo;
pub mod utils;
pub mod wm;

mod app_ext;

pub use app_ext::*;

use eyre::ContextCompat;
use gpui::App;

use crate::error::Result;
use crate::theme::Theme;
use crate::wm::WindowManager;

pub fn init(cx: &mut App) -> Result<()> {
    assets::load_fonts(cx).ok().wrap_err("failed to load fonts")?;

    cx.set_global(Theme::default());

    let wm = WindowManager::new(cx);
    cx.set_global(wm);

    input::init(cx);
    table::init(cx);
    wm::init(cx);

    Ok(())
}
