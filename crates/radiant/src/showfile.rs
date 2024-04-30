use anyhow::{anyhow, Result};
use gpui::{AppContext, Global, SharedString};
use std::{fmt::Display, path::PathBuf, str::FromStr};

use backstage::show::{PresetFilter, Show};

use crate::geo::{Bounds, Size};

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Layouts {
    pub selected_layout_id: usize,
    pub layouts: Vec<Layout>,
}

impl Layouts {
    pub fn current_layout(&self) -> Option<&Layout> {
        self.layouts.get(self.selected_layout_id)
    }
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

#[derive(Debug, Clone, PartialEq, Default, serde::Deserialize, serde::Serialize)]
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

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Window {
    pub id: usize,
    pub bounds: Bounds,
    pub kind: WindowKind,
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum WindowKind {
    Pool(PoolWindow),
    AttributeEditor,
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PoolWindow {
    pub kind: PoolWindowKind,
    pub scroll_offset: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PoolWindowKind {
    Group,
    Preset(PresetKind),
}

impl serde::Serialize for PoolWindowKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        match self {
            PoolWindowKind::Group => serializer.serialize_str("Group"),
            PoolWindowKind::Preset(kind) => format!("Preset({})", kind).serialize(serializer),
        }
    }
}

impl<'de> serde::Deserialize<'de> for PoolWindowKind {
    fn deserialize<D>(deserializer: D) -> Result<PoolWindowKind, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        if s == "Group" {
            Ok(PoolWindowKind::Group)
        } else {
            let kind = s
                .strip_prefix("Preset(")
                .and_then(|s| s.strip_suffix(")"))
                .ok_or_else(|| serde::de::Error::custom("Invalid PoolWindowKind"))?;
            Ok(PoolWindowKind::Preset(
                kind.parse().map_err(serde::de::Error::custom)?,
            ))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum PresetKind {
    Dimmer,
    Position,
    Gobo,
    Color,
    Beam,
    Focus,
    Control,
    Shapers,
    Video,
    All,
}

impl Display for PresetKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PresetKind::Dimmer => write!(f, "Dimmer"),
            PresetKind::Position => write!(f, "Position"),
            PresetKind::Gobo => write!(f, "Gobo"),
            PresetKind::Color => write!(f, "Color"),
            PresetKind::Beam => write!(f, "Beam"),
            PresetKind::Focus => write!(f, "Focus"),
            PresetKind::Control => write!(f, "Control"),
            PresetKind::Shapers => write!(f, "Shapers"),
            PresetKind::Video => write!(f, "Video"),
            PresetKind::All => write!(f, "All"),
        }
    }
}

impl FromStr for PresetKind {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        match s {
            "Dimmer" => Ok(PresetKind::Dimmer),
            "Position" => Ok(PresetKind::Position),
            "Gobo" => Ok(PresetKind::Gobo),
            "Color" => Ok(PresetKind::Color),
            "Beam" => Ok(PresetKind::Beam),
            "Focus" => Ok(PresetKind::Focus),
            "Control" => Ok(PresetKind::Control),
            "Shapers" => Ok(PresetKind::Shapers),
            "Video" => Ok(PresetKind::Video),
            "All" => Ok(PresetKind::All),
            other => Err(anyhow!("Invalid PresetKind: '{other}'")),
        }
    }
}

impl From<PresetKind> for PresetFilter {
    fn from(kind: PresetKind) -> Self {
        PresetFilter::FeatureGroup(kind.to_string())
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Io {
    pub artnet: Vec<ArtnetProtocol>,
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ArtnetProtocol {
    pub target_ip: String,
}
