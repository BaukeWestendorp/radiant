pub mod assets;
pub mod error;
pub mod interactive;
pub mod misc;
pub mod nav;
pub mod org;
pub mod theme;
pub mod typo;

pub mod utils;

use eyre::Context;

use crate::error::Result;

pub fn init(cx: &mut gpui::App) -> Result<()> {
    assets::load_fonts(cx).map_err(|err| eyre::eyre!(err)).wrap_err("failed to load fonts")?;

    theme::Theme::init(cx);
    interactive::init(cx);

    Ok(())
}

pub trait Disableable {
    fn disabled(self, disabled: bool) -> Self;
}

pub trait Selectable {
    fn selected(self, selected: bool) -> Self;
}
