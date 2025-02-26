use frames::{Frame, FrameWrapper};
use gpui::*;

pub enum MainFrame {
    Example,
}

impl Frame for MainFrame {
    fn render_content(&mut self, _cx: &mut Context<FrameWrapper<Self>>) -> impl IntoElement {
        match self {
            MainFrame::Example => div().child("Example"),
        }
    }
}
