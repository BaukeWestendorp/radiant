use gpui::{AppContext, Context, Model};

#[derive(Debug, Clone, PartialEq)]
pub struct Layout {
    pub main_window: Model<Window>,
    pub secondary_window: Model<Window>,
}

impl Layout {
    pub fn from_showfile(layout: showfile::Layout, cx: &mut AppContext) -> Self {
        Self {
            main_window: cx.new_model(|_| layout.main_window.into()),
            secondary_window: cx.new_model(|_| layout.secondary_window.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Window {
    pub frames: Vec<Frame>,
}

impl From<showfile::Window> for Window {
    fn from(window: showfile::Window) -> Self {
        Self {
            frames: window.frames.into_iter().map(Frame::from).collect(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Frame {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub kind: FrameKind,
}

impl From<showfile::Frame> for Frame {
    fn from(frame: showfile::Frame) -> Self {
        Self {
            x: frame.x,
            y: frame.y,
            width: frame.width,
            height: frame.height,
            kind: frame.kind.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameKind {
    EffectGraphEditor,
    Pool(PoolKind),
}

impl From<showfile::FrameKind> for FrameKind {
    fn from(kind: showfile::FrameKind) -> Self {
        match kind {
            showfile::FrameKind::EffectGraphEditor => Self::EffectGraphEditor,
            showfile::FrameKind::Pool(kind) => Self::Pool(kind.into()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PoolKind {
    EffectGraph,
    Effect,
    Group,
}

impl From<showfile::PoolKind> for PoolKind {
    fn from(kind: showfile::PoolKind) -> Self {
        match kind {
            showfile::PoolKind::EffectGraph => Self::EffectGraph,
            showfile::PoolKind::Effect => Self::Effect,
            showfile::PoolKind::Group => Self::Group,
        }
    }
}
