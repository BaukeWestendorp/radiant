use std::path::PathBuf;

use crate::showfile::{self, Showfile};
use anyhow::Context;
use assets::Assets;
use dmx_io::DmxIoSettings;
use gpui::{AppContext, Entity};

pub mod assets;
pub mod dmx_io;
pub mod layout;
pub mod patch;

#[derive(Clone)]
pub struct Show {
    pub path: Option<PathBuf>,
    pub dmx_io_settings: dmx_io::DmxIoSettings,
    pub assets: assets::Assets,
    pub layout: Entity<showfile::Layout>,
    pub patch: Entity<patch::Patch>,
}

impl Show {
    pub fn new(cx: &mut gpui::App) -> Self {
        Self::try_from_showfile(None, Showfile::default(), cx)
            .expect("should create show from default showfile")
    }

    pub fn init(cx: &mut gpui::App, showfile_path: Option<&PathBuf>) -> anyhow::Result<()> {
        let show = match showfile_path {
            Some(path) => Show::open_from_file(path.clone(), cx).expect("should open showfile"),
            None => Show::new(cx),
        };

        cx.set_global(show);

        Ok(())
    }

    pub(crate) fn try_from_showfile(
        path: Option<PathBuf>,
        showfile: Showfile,
        cx: &mut gpui::App,
    ) -> anyhow::Result<Self> {
        let patch = patch::Patch::try_from_showfile(showfile.patch.clone())?;

        Ok(Show {
            path,
            assets: Assets::from_showfile(&showfile, cx),
            dmx_io_settings: DmxIoSettings::from_showfile(showfile.dmx_io_settings, cx),
            layout: cx.new(|_| showfile.layout),
            patch: cx.new(|_| patch),
        })
    }

    pub fn open_from_file(path: PathBuf, cx: &mut gpui::App) -> anyhow::Result<Show> {
        let showfile = Showfile::open_from_file(&path).context("open showfile")?;
        Show::try_from_showfile(Some(path), showfile, cx)
    }

    pub fn save_to_file(&mut self, path: &PathBuf, cx: &gpui::App) -> Result<(), std::io::Error> {
        let showfile = Showfile {
            dmx_io_settings: self.dmx_io_settings.to_showfile(cx),
            layout: self.layout.read(cx).clone(),
            assets: self.assets.to_showfile(cx),
            patch: self.patch.read(cx).to_showfile(),
        };

        self.path = Some(path.clone());

        showfile.save_to_file(path)
    }
}

impl gpui::Global for Show {}
