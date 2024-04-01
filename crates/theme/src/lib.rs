use std::sync::Arc;

use gpui::{AppContext, Global};

use self::colors::ThemeColors;
use self::default::ThemeStyles;

pub mod colors;
pub mod default;

#[derive(Debug, Clone)]
pub struct Theme {
    pub styles: ThemeStyles,
}

impl Theme {
    pub fn colors(&self) -> &ThemeColors {
        &self.styles.colors
    }
}

pub trait ActiveTheme {
    fn theme(&self) -> &Arc<Theme>;
}

impl ActiveTheme for AppContext {
    fn theme(&self) -> &Arc<Theme> {
        &self.global::<ThemeSettings>().active_theme
    }
}

pub struct ThemeSettings {
    pub active_theme: Arc<Theme>,
}

impl ThemeSettings {
    pub fn init(cx: &mut AppContext) {
        cx.set_global(Self::default());
    }
}

impl Default for ThemeSettings {
    fn default() -> Self {
        Self {
            active_theme: Arc::new(Theme::default()),
        }
    }
}

impl Global for ThemeSettings {}
