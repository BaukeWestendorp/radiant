use crate::showfile::{self, Showfile};
use assets::Assets;

pub mod assets;
pub mod dmx_io;
pub mod layout;

#[derive(Clone, Default)]
pub struct Show {
    pub dmx_io_settings: dmx_io::DmxIoSettings,
    pub assets: assets::Assets,
    pub layout: showfile::Layout,
}

impl Show {
    pub(crate) fn from_showfile(showfile: Showfile, cx: &mut gpui::App) -> Self {
        Show {
            assets: Assets::from_showfile(&showfile, cx),
            dmx_io_settings: showfile.dmx_io_settings,
            layout: showfile.layout,
        }
    }
}

impl gpui::Global for Show {}

pub fn open_from_file(path: &std::path::PathBuf, cx: &mut gpui::App) -> ron::Result<Show> {
    let showfile = showfile::open_from_file(path)?;
    Ok(Show::from_showfile(showfile, cx))
}
