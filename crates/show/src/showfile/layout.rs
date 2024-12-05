use flow::Point;

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct Layout {
    pub main_window: Window,
    pub secondary_window: Window,
}

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct Window {
    pub selected_effect_graph: Option<super::AssetId>,
    pub frames: Vec<Frame>,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct Frame {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub kind: FrameKind,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub enum FrameKind {
    EffectGraphEditor {
        auto_save: bool,
        graph_offset: Point,
    },
    Pool(PoolKind),
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub enum PoolKind {
    EffectGraph,
    Effect,
    Group,
}
