use gpui::{AnyView, WindowContext};

use crate::workspace::frame::{EffectGraphEditorFrameDelegate, Frame};

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Windows {
    pub main_window: Window,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Window {
    pub frames: Vec<FrameKind>,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum FrameKind {
    EffectGraphEditor,
}

impl FrameKind {
    pub fn to_frame_view(&self, cx: &mut WindowContext) -> AnyView {
        match self {
            FrameKind::EffectGraphEditor => {
                Frame::build(EffectGraphEditorFrameDelegate::new(cx), cx)
            }
        }
        .into()
    }
}
