use crate::show::{AssetId, effect_graph::EffectGraph};
use gpui::Bounds;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FrameKind {
    Window(WindowFrameKind),
    Pool(PoolFrameKind),
}

impl FrameKind {
    pub fn all() -> [Self; 15] {
        [
            // Presets
            Self::Pool(PoolFrameKind::DimmerPresets),
            Self::Pool(PoolFrameKind::PositionPresets),
            Self::Pool(PoolFrameKind::GoboPresets),
            Self::Pool(PoolFrameKind::ColorPresets),
            Self::Pool(PoolFrameKind::BeamPresets),
            Self::Pool(PoolFrameKind::FocusPresets),
            Self::Pool(PoolFrameKind::ControlPresets),
            Self::Pool(PoolFrameKind::ShapersPresets),
            Self::Pool(PoolFrameKind::VideoPresets),
            // Pools
            Self::Pool(PoolFrameKind::EffectGraphs),
            Self::Pool(PoolFrameKind::FixtureGroups),
            Self::Pool(PoolFrameKind::Cues),
            Self::Pool(PoolFrameKind::Sequences),
            Self::Pool(PoolFrameKind::Executors),
            // Editors
            Self::Window(WindowFrameKind::EffectGraphEditor(None)),
        ]
    }
}

impl std::fmt::Display for FrameKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FrameKind::Window(kind) => kind.fmt(f),
            FrameKind::Pool(kind) => kind.fmt(f),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WindowFrameKind {
    EffectGraphEditor(Option<AssetId<EffectGraph>>),
}

impl std::fmt::Display for WindowFrameKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EffectGraphEditor(_) => write!(f, "Effect Graph Editor"),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
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

impl std::fmt::Display for PoolFrameKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EffectGraphs => write!(f, "Effect Graphs"),
            Self::FixtureGroups => write!(f, "Fixture Groups"),

            Self::Cues => write!(f, "Cues"),
            Self::Sequences => write!(f, "Sequences"),
            Self::Executors => write!(f, "Executors"),

            Self::DimmerPresets => write!(f, "Dimmer Presets"),
            Self::PositionPresets => write!(f, "Position Presets"),
            Self::GoboPresets => write!(f, "Gobo Presets"),
            Self::ColorPresets => write!(f, "Color Presets"),
            Self::BeamPresets => write!(f, "Beam Presets"),
            Self::FocusPresets => write!(f, "Focus Presets"),
            Self::ControlPresets => write!(f, "Control Presets"),
            Self::ShapersPresets => write!(f, "Shapers Presets"),
            Self::VideoPresets => write!(f, "Video Presets"),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone)]
pub struct Frame {
    pub bounds: Bounds<u32>,
    pub kind: FrameKind,
}
