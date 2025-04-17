use showfile::Showfile;
use std::path::PathBuf;

pub mod assets;
pub(crate) mod showfile;

pub mod layout {
    pub use crate::showfile::layout::*;
}

#[derive(Clone, Default)]
pub struct Show {
    pub assets: assets::Assets,
    pub layout: showfile::layout::Layout,
}

impl Show {
    pub(crate) fn from_showfile(showfile: Showfile, cx: &mut gpui::App) -> Self {
        Show { assets: assets::Assets::from_showfile(&showfile, cx), layout: showfile.layout }
    }
}

impl gpui::Global for Show {}

pub fn open_from_file(path: &PathBuf, cx: &mut gpui::App) -> Result<Show, std::io::Error> {
    let showfile = showfile::open_from_file(path)?;
    Ok(Show::from_showfile(showfile, cx))
}
