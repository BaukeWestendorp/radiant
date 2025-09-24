use std::borrow::Cow;

use gpui::{App, AssetSource, Result, SharedString};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "./assets"]
#[include = "fonts/*"]
#[exclude = "*.DS_Store"]
pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        Ok(Self::get(path)
            .map(|f| Some(f.data))
            .expect(&format!("could not find asset at path \"{}\"", path)))
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(Self::iter()
            .filter_map(|p| if p.starts_with(path) { Some(p.into()) } else { None })
            .collect())
    }
}

pub(crate) fn load_fonts(cx: &mut App) -> gpui::Result<()> {
    let font_paths = cx.asset_source().list("fonts")?;
    let mut embedded_fonts = Vec::new();
    for font_path in font_paths {
        if font_path.ends_with(".ttf") {
            if let Some(font_bytes) = cx.asset_source().load(&font_path)? {
                embedded_fonts.push(font_bytes);
            }
        }
    }
    cx.text_system().add_fonts(embedded_fonts)?;
    Ok(())
}
