use std::path::PathBuf;

use crate::showfile::{self, Showfile};
use assets::Assets;
use dmx_io::DmxIoSettings;

pub mod assets;
pub mod dmx_io;
pub mod layout;

#[derive(Clone, Default)]
pub struct Show {
    pub path: Option<PathBuf>,
    pub dmx_io_settings: dmx_io::DmxIoSettings,
    pub assets: assets::Assets,
    pub layout: showfile::Layout,
}

impl Show {
    pub(crate) fn from_showfile(
        path: Option<PathBuf>,
        showfile: Showfile,
        cx: &mut gpui::App,
    ) -> Self {
        Show {
            path,
            assets: Assets::from_showfile(&showfile, cx),
            dmx_io_settings: DmxIoSettings::from_showfile(showfile.dmx_io_settings, cx),
            layout: showfile.layout,
        }
    }

    pub fn open_from_file(path: PathBuf, cx: &mut gpui::App) -> ron::Result<Show> {
        let showfile = Showfile::open_from_file(&path)?;
        Ok(Show::from_showfile(Some(path), showfile, cx))
    }

    pub fn save_to_file(&mut self, path: &PathBuf, cx: &gpui::App) -> Result<(), std::io::Error> {
        let showfile = Showfile {
            dmx_io_settings: self.dmx_io_settings.to_showfile(cx),
            layout: self.layout.clone(),
            assets: self.assets.to_showfile(cx),
        };

        self.path = Some(path.clone());

        showfile.save_to_file(path)
    }
}

impl gpui::Global for Show {}
