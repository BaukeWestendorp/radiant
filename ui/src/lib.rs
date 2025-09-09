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

/// Initializes the application by loading fonts, themes, and interactive
/// components.
///
/// # Errors
///
/// Returns an error if font loading fails.
pub fn init(cx: &mut gpui::App) -> Result<()> {
    assets::load_fonts(cx).map_err(|err| eyre::eyre!(err)).wrap_err("failed to load fonts")?;

    theme::init(cx);
    interactive::init(cx);

    Ok(())
}

/// A trait for types that can be disabled.
pub trait Disableable {
    /// Sets the disabled state.

    fn disabled(self, disabled: bool) -> Self;
}

/// A trait for types that can be selected.
pub trait Selectable {
    /// Sets the selected state.
    fn selected(self, selected: bool) -> Self;
}
