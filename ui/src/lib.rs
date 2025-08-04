mod assets;
mod error;
mod interactive;
mod misc;
mod nav;
mod org;
mod theme;
mod typo;

pub mod utils;

pub use assets::*;
use eyre::Context;
pub use interactive::*;
pub use misc::*;
pub use nav::*;
pub use org::*;
pub use theme::*;
pub use typo::*;

use crate::error::Result;

pub mod actions {
    use gpui::App;

    pub fn init(cx: &mut App) {
        super::interactive::actions::init(cx);
    }
}

pub fn init(cx: &mut gpui::App) -> Result<()> {
    assets::load_fonts(cx).map_err(|err| eyre::eyre!(err)).wrap_err("failed to load fonts")?;
    theme::Theme::init(cx);
    Ok(())
}

pub trait Disableable {
    fn disabled(self, disabled: bool) -> Self;
}

pub trait Selectable {
    fn selected(self, selected: bool) -> Self;
}
