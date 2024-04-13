use std::io::BufReader;
use std::path::Path;
use std::{fs::File, io::Write};

use backstage::Show;
use gpui::{AppContext, BorrowAppContext, Global, SharedString};

use crate::geometry::{Bounds, Size};

const LAYOUT_PATH: &str = "layouts.json";
const SHOW_PATH: &str = "show.json";
const IO_PATH: &str = "io.json";

pub struct ShowfileManager {
    showfile: Showfile,
    path: Option<String>,
}

impl ShowfileManager {
    pub fn init(showfile_path: Option<String>, cx: &mut AppContext) {
        let showfile = match showfile_path.clone() {
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

        cx.set_global(ShowfileManager {
            showfile,
            path: showfile_path.map(|path| path.into()),
        });
    }

    pub fn save(cx: &AppContext) -> anyhow::Result<()> {
        let manager = cx.global::<Self>();
        let path = manager.path.as_ref().unwrap();
        let showfile = manager.showfile.clone();
        let showfile_path = Path::new(path.as_str());
        std::fs::create_dir_all(showfile_path)?;
        let layout_file = File::create(showfile_path.join(LAYOUT_PATH))?;
        serde_json::to_writer(layout_file, &showfile.layouts)?;

        let show_json = showfile.show.to_json()?;
        let mut show_file = File::create(showfile_path.join(SHOW_PATH))?;
        show_file.write(show_json.as_bytes())?;

        let io_file = File::create(showfile_path.join(IO_PATH))?;
        serde_json::to_writer(io_file, &showfile.io)?;

        log::info!("Showfile saved to '{}'", showfile_path.display());

        Ok(())
    }

    pub fn update<C: BorrowAppContext, R>(
        cx: &mut C,
        f: impl FnOnce(&mut Showfile, &mut C) -> R,
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

#[derive(Default, Clone)]
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Layouts {
    pub selected_layout_id: usize,
    pub layouts: Vec<Layout>,
}

impl Default for Layouts {
    fn default() -> Self {
        Self {
            selected_layout_id: 1,
            layouts: vec![Layout {
                id: 1,
                label: "Layout 1".into(),
                size: Size {
                    width: 20,
                    height: 12,
                },
                windows: Vec::new(),
            }],
        }
    }
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
    Group,
    Sequence,
    BeamPreset,
    ColorPreset,
    DimmerPreset,
    FocusPreset,
    GoboPreset,
    PositionPreset,
    AllPreset,
}

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct Io {
    pub artnet: Vec<ArtnetProtocol>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ArtnetProtocol {
    pub target_ip: String,
}
