use crate::showfile::AssetId;
use gpui::{Bounds, Size};

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Default)]
pub struct Layout {
    pub main_window: MainWindow,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone)]
pub struct MainWindow {
    pub size: Size<u32>,
    pub frames: Vec<Frame<MainFrameKind>>,
}

impl Default for MainWindow {
    fn default() -> Self {
        Self { size: Size { width: 20, height: 12 }, frames: Vec::default() }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, Eq)]
pub enum MainFrameKind {
    EffectGraphEditor(AssetId),
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Default)]
pub struct Frame<K> {
    pub bounds: Bounds<u32>,
    pub kind: K,
}
