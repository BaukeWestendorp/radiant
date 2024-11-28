use gpui::{AnyView, WindowContext};

use crate::workspace::frame::{Frame, TestFrameDelegate};

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
    Test,
}

impl FrameKind {
    pub fn to_frame_view(&self, cx: &mut WindowContext) -> AnyView {
        match self {
            FrameKind::Test => Frame::build(TestFrameDelegate {}, cx),
        }
        .into()
    }
}
