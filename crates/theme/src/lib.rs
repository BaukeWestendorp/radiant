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
        load_fonts(cx).expect("Failed to load fonts");
        cx.set_global(Self::default());
    }
}

fn load_fonts(cx: &mut AppContext) -> gpui::Result<()> {
    let font_paths = cx.asset_source().list("fonts")?;
    let mut embedded_fonts = Vec::new();
    for font_path in font_paths {
        if font_path.ends_with(".ttf") {
            let font_bytes = cx.asset_source().load(&font_path)?;
            embedded_fonts.push(font_bytes);
        }
    }

    cx.text_system().add_fonts(embedded_fonts)
}

impl Default for ThemeSettings {
    fn default() -> Self {
        Self {
            active_theme: Arc::new(Theme::default()),
        }
    }
}

impl Global for ThemeSettings {}
