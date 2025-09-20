use std::ops::{Deref, DerefMut};

use gpui::{App, Context, Global, Pixels, px};

mod colors;

pub use colors::*;

/// Stores theme colors and UI style properties.
pub struct Theme {
    colors: ThemeColors,

    /// Default border radius for UI elements.
    pub radius: Pixels,
    /// Cursor width.
    pub cursor_width: Pixels,
    /// Enable shadows.
    pub shadow: bool,
}

impl Deref for Theme {
    type Target = ThemeColors;

    fn deref(&self) -> &Self::Target {
        &self.colors
    }
}

impl DerefMut for Theme {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.colors
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            colors: ThemeColors::default(),
            radius: px(3.0),
            cursor_width: px(2.0),
            shadow: true,
        }
    }
}

impl Global for Theme {}

/// Trait for accessing the active theme.
pub trait ActiveTheme {
    fn theme(&self) -> &Theme;
}

impl<E> ActiveTheme for Context<'_, E> {
    fn theme(&self) -> &Theme {
        self.global()
    }
}

impl ActiveTheme for App {
    fn theme(&self) -> &Theme {
        self.global()
    }
}
