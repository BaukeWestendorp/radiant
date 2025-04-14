use gpui::{Bounds, Size};

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Layout {
    pub main_window: MainWindow,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MainWindow {
    pub size: Size<u32>,
    pub frames: Vec<Frame<MainFrameKind>>,
}

impl Default for MainWindow {
    fn default() -> Self {
        Self { size: Size::new(20, 12), frames: Vec::default() }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MainFrameKind {
    Debugger,
    EffectGraphEditor,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Frame<K> {
    pub bounds: Bounds<u32>,
    pub kind: K,
}
