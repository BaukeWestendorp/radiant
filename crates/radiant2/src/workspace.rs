use gpui::{
    div, IntoElement, ParentElement, Render, Styled, View, ViewContext, VisualContext,
    WindowContext,
};

use crate::window::{Window, WindowDelegate};

pub struct Workspace {}

impl Workspace {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| Self {})
    }
}

impl Render for Workspace {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let delegate = TestWindowDelegate {};
        Window::build(delegate, cx)
    }
}

struct TestWindowDelegate {}

impl WindowDelegate for TestWindowDelegate {
    fn render_content(&mut self, _cx: &mut ViewContext<Window<Self>>) -> impl IntoElement
    where
        Self: Sized,
    {
        div().text_color(gpui::red()).child("helo world")
    }
}
