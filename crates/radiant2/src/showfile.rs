use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use backstage::Show;
use gpui::{AppContext, BorrowAppContext, Global, SharedString};

use crate::geometry::{Bounds, Size};

const LAYOUT_PATH: &str = "layouts.json";
const SHOW_PATH: &str = "show.json";
const IO_PATH: &str = "io.json";

pub struct ShowfileManager {
    showfile: Showfile,
}

impl ShowfileManager {
    pub fn init(showfile_path: Option<String>, cx: &mut AppContext) {
        let showfile = match showfile_path {
            Some(showfile_path) => {
                let showfile_path = Path::new(&showfile_path);
                match Showfile::from_dir(showfile_path) {
                    Ok(showfile) => showfile,
                    Err(error) => {
                        log::error!(
                            "Failed to load showfile at '{}': {error}. Loading empty show instead",
                            showfile_path.display()
                        );
                        // FIXME: Show error popup.
                        Showfile::default()
                    }
                }
            }
            None => Showfile::default(),
        };

        cx.set_global(ShowfileManager { showfile });
    }

    pub fn update<R>(
        cx: &mut AppContext,
        f: impl FnOnce(&mut Showfile, &mut AppContext) -> R,
    ) -> R {
        cx.update_global::<Self, R>(|this, cx| f(&mut this.showfile, cx))
    }

    pub fn show(cx: &AppContext) -> &Show {
        &cx.global::<Self>().showfile.show
    }

    pub fn layouts(cx: &AppContext) -> &Layouts {
        &cx.global::<Self>().showfile.layouts
    }

    pub fn io(cx: &AppContext) -> &Io {
        &cx.global::<Self>().showfile.io
    }
}

impl Global for ShowfileManager {}

#[derive(Default)]
pub struct Showfile {
    pub layouts: Layouts,
    pub show: Show,
    pub io: Io,
}

impl Showfile {
    pub fn from_dir(path: &Path) -> anyhow::Result<Self> {
        let layout_file = File::open(path.join(LAYOUT_PATH))?;
        let layout_reader = BufReader::new(layout_file);
        let layouts: Layouts = serde_json::from_reader(layout_reader)?;

        let show_file = File::open(path.join(SHOW_PATH))?;
        let show = futures_lite::future::block_on(Show::from_file(show_file))?;

        let io_file = File::open(path.join(IO_PATH))?;
        let io_reader = BufReader::new(io_file);
        let io: Io = serde_json::from_reader(io_reader)?;

        Ok(Showfile { layouts, show, io })
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Layouts {
    pub selected_layout_id: usize,
    pub layouts: Vec<Layout>,
}

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct Layout {
    pub id: usize,
    pub label: SharedString,
    pub size: Size,
    pub windows: Vec<Window>,
}

impl Layout {
    pub fn window(&self, id: usize) -> Option<&Window> {
        self.windows.iter().find(|w| w.id == id)
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Window {
    pub id: usize,
    pub bounds: Bounds,
    pub kind: WindowKind,
}

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub enum WindowKind {
    Pool(PoolWindow),
    Executors,
    FixtureSheet,
}

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct PoolWindow {
    pub kind: PoolWindowKind,
    pub scroll_offset: i32,
}

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub enum PoolWindowKind {
    ColorPreset,
    Group,
}

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct Io {
    pub artnet: Vec<ArtnetProtocol>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ArtnetProtocol {
    pub target_ip: String,
}
