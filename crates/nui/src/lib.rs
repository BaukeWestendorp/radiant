pub mod assets;
pub mod error;

mod app_ext;
mod wm;

pub use app_ext::*;
pub use wm::*;

use eyre::ContextCompat;
use gpui::App;

use crate::error::Result;

pub fn init(cx: &mut App) -> Result<()> {
    assets::load_fonts(cx).ok().wrap_err("failed to load fonts")?;

    let wm = WindowManager::new(cx);
    cx.set_global(wm);

    Ok(())
}
