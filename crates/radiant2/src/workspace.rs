use gpui::{
    div, IntoElement, ParentElement, Render, View, ViewContext, VisualContext, WindowContext,
};

pub struct Workspace {}

impl Workspace {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| Self {})
    }
}

impl Render for Workspace {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div().child("child")
    }
}
