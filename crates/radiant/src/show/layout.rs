use gpui::{Bounds, Size};

use crate::show::asset::AssetId;

use super::asset::effect_graph::EffectGraph;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Default)]
pub struct Layout {
    pub main_window: MainWindow,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone)]
pub struct MainWindow {
    pub size: Size<u32>,
    pub pages: Vec<Page>,
    pub loaded_page: Page,
}

impl Default for MainWindow {
    fn default() -> Self {
        Self {
            size: Size { width: 20, height: 12 },
            pages: Vec::default(),
            loaded_page: Page::default(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Default)]
pub struct Page {
    pub label: String,
    pub frames: Vec<Frame<MainFrameKind>>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, Eq)]
pub enum MainFrameKind {
    EffectGraphEditor(AssetId<EffectGraph>),
    Pool(PoolKind),
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, Eq)]
pub enum PoolKind {
    EffectGraphs,
    FixtureGroups,

    Cues,
    Sequences,
    Executors,

    DimmerPresets,
    PositionPresets,
    GoboPresets,
    ColorPresets,
    BeamPresets,
    FocusPresets,
    ControlPresets,
    ShapersPresets,
    VideoPresets,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Default)]
pub struct Frame<K> {
    pub bounds: Bounds<u32>,
    pub kind: K,
}
