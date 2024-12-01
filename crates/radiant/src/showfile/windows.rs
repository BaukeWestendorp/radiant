use gpui::{AnyView, WindowContext};

use crate::workspace::{
    EffectGraphEditorFrameDelegate, EffectGraphPoolFrameDelegate, EffectPoolFrameDelegate,
    FrameView, GroupPoolFrameDelegate, PoolFrameDelegate,
};

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Windows {
    pub main_window: Window,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Window {
    pub frames: Vec<Frame>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Frame {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub kind: FrameKind,
}

impl Frame {
    pub fn to_frame_view(&self, cx: &mut WindowContext) -> AnyView {
        match self.kind {
            FrameKind::EffectGraphEditor => {
                FrameView::build(self.clone(), EffectGraphEditorFrameDelegate::new(cx), cx).into()
            }
            FrameKind::Pool(kind) => match kind {
                PoolKind::EffectGraph => FrameView::build(
                    self.clone(),
                    PoolFrameDelegate::new(
                        self.width,
                        self.height,
                        EffectGraphPoolFrameDelegate::new(),
                    ),
                    cx,
                )
                .into(),
                PoolKind::Effect => FrameView::build(
                    self.clone(),
                    PoolFrameDelegate::new(self.width, self.height, EffectPoolFrameDelegate::new()),
                    cx,
                )
                .into(),
                PoolKind::Group => FrameView::build(
                    self.clone(),
                    PoolFrameDelegate::new(self.width, self.height, GroupPoolFrameDelegate::new()),
                    cx,
                )
                .into(),
            },
        }
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum FrameKind {
    EffectGraphEditor,
    Pool(PoolKind),
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum PoolKind {
    EffectGraph,
    Effect,
    Group,
}
