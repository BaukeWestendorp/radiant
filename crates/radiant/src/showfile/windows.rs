use gpui::{AnyView, WindowContext};

use crate::workspace::frame::{EffectGraphEditorFrameDelegate, Frame};

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Windows {
    pub main_window: Window,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Window {
    pub frames: Vec<BoundedFrame>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BoundedFrame {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub kind: FrameKind,
}

impl BoundedFrame {
    pub fn to_frame_view(&self, cx: &mut WindowContext) -> AnyView {
        match self.kind {
            FrameKind::EffectGraphEditor => {
                Frame::build(self.clone(), EffectGraphEditorFrameDelegate::new(cx), cx)
            }
        }
        .into()
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum FrameKind {
    EffectGraphEditor,
}
