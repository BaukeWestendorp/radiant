use anyhow::anyhow;
use gpui::SharedString;
use std::{fmt::Display, str::FromStr};

use backstage::show::PresetFilter;

use crate::geo::{Bounds, Size};

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
    GraphEditor(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PoolWindow {
    pub kind: PoolWindowKind,
    pub scroll_offset: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum PoolWindowKind {
    Group,
    Preset(PresetKind),
    Effect,
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
