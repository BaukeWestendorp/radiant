use crate::show::asset::{AssetId, effect_graph::EffectGraph};
use gpui::Bounds;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, Eq)]
pub enum FrameKind {
    Window(WindowFrameKind),
    Pool(PoolFrameKind),
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, Eq)]
pub enum WindowFrameKind {
    EffectGraphEditor(AssetId<EffectGraph>),
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, Eq)]
pub enum PoolFrameKind {
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
#[derive(Clone)]
pub struct Frame {
    pub bounds: Bounds<u32>,
    pub kind: FrameKind,
}
