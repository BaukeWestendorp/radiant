use anyhow::Result;
use gpui::{AppContext, Global, SharedString};
use std::path::PathBuf;

use backstage::show::Show;

use crate::geo::{Bounds, Size};

#[derive(Debug, Clone)]
pub struct Showfile {
    pub show: Show,
    pub layouts: Layouts,
}

impl Showfile {
    pub fn init(showfile_path: Option<PathBuf>, cx: &mut AppContext) -> Result<()> {
        let mut show: Show = match &showfile_path {
            Some(showfile_path) => {
                let file = std::fs::File::open(showfile_path.join("show.json"))?;
                serde_json::from_reader(file)?
            }
            None => {
                log::info!("No showfile path provided. Using the default show.");
                Show::default()
            }
        };

        let layouts: Layouts = match &showfile_path {
            Some(showfile_path) => {
                let file = std::fs::File::open(showfile_path.join("layout.json"))?;
                serde_json::from_reader(file)?
            }
            None => {
                log::info!("No showfile path provided. Using the default layouts.");
                Layouts::default()
            }
        };

        smol::block_on(async {
            match show
                .initialize(
                    std::env::var("GDTF_SHARE_USER").unwrap(),
                    std::env::var("GDTF_SHARE_PASSWORD").unwrap(),
                )
                .await
            {
                Ok(_) => {
                    log::info!("Show has been initialized")
                }
                Err(err) => {
                    log::error!("Failed to initialize show: {err}")
                }
            }
        });

        cx.set_global(Showfile { show, layouts });

        Ok(())
    }
}

impl Global for Showfile {}

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
    AttributeEditor,
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
    Preset,
}

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct Io {
    pub artnet: Vec<ArtnetProtocol>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ArtnetProtocol {
    pub target_ip: String,
}
