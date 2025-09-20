pub mod assets;
pub mod error;
pub mod wm;

mod app_ext;

pub use app_ext::*;

use eyre::ContextCompat;
use gpui::App;

use crate::error::Result;
use crate::wm::WindowManager;

pub fn init(cx: &mut App) -> Result<()> {
    assets::load_fonts(cx).ok().wrap_err("failed to load fonts")?;

    let wm = WindowManager::new(cx);
    cx.set_global(wm);

    Ok(())
}
