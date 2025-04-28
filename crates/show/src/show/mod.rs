use std::path::PathBuf;

use crate::showfile::{self, Showfile};
use assets::Assets;
use dmx_io::DmxIoSettings;
use gpui::{AppContext, Entity};

pub mod assets;
pub mod dmx_io;
pub mod layout;

#[derive(Clone)]
pub struct Show {
    pub path: Option<PathBuf>,
    pub dmx_io_settings: dmx_io::DmxIoSettings,
    pub assets: assets::Assets,
    pub layout: Entity<showfile::Layout>,
}

impl Show {
    pub fn new(cx: &mut gpui::App) -> Self {
        Self::from_showfile(None, Showfile::default(), cx)
    }

    pub fn init(cx: &mut gpui::App, showfile_path: Option<&PathBuf>) {
        let show = match showfile_path {
            Some(path) => match Show::open_from_file(path.clone(), cx) {
                Ok(show) => show,
                Err(err) => {
                    log::error!("Error opening showfile: '{}'", err);
                    std::process::exit(1);
                }
            },
            None => Show::new(cx),
        };

        cx.set_global(show);
    }

    pub(crate) fn from_showfile(
        path: Option<PathBuf>,
        showfile: Showfile,
        cx: &mut gpui::App,
    ) -> Self {
        Show {
            path,
            assets: Assets::from_showfile(&showfile, cx),
            dmx_io_settings: DmxIoSettings::from_showfile(showfile.dmx_io_settings, cx),
            layout: cx.new(|_| showfile.layout),
        }
    }

    pub fn open_from_file(path: PathBuf, cx: &mut gpui::App) -> ron::Result<Show> {
        let showfile = Showfile::open_from_file(&path)?;
        Ok(Show::from_showfile(Some(path), showfile, cx))
    }

    pub fn save_to_file(&mut self, path: &PathBuf, cx: &gpui::App) -> Result<(), std::io::Error> {
        let showfile = Showfile {
            dmx_io_settings: self.dmx_io_settings.to_showfile(cx),
            layout: self.layout.read(cx).clone(),
            assets: self.assets.to_showfile(cx),
        };

        self.path = Some(path.clone());

        showfile.save_to_file(path)
    }
}

impl gpui::Global for Show {}
