use gpui::*;
use ui::theme::ActiveTheme;

use super::{Frame, FrameDelegate};

pub struct TestFrameDelegate {}

impl FrameDelegate for TestFrameDelegate {
    fn title(&mut self, _cx: &mut ViewContext<Frame<Self>>) -> &str
    where
        Self: Sized,
    {
        "Test Frame"
    }

    fn render(&mut self, cx: &mut ViewContext<Frame<Self>>) -> impl IntoElement
    where
        Self: Sized,
    {
        div()
            .size_full()
            .child("Test Frame")
            .bg(white())
            .border_1()
            .border_color(red())
            .rounded(cx.theme().radius)
    }
}
