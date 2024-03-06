use gpui::{
    div, IntoElement, Model, ParentElement, Render, Styled, View, ViewContext, VisualContext,
    WindowContext,
};

use crate::show::Window;

pub struct WindowView {}

impl WindowView {
    pub fn build(window: Model<Window>, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| {
            let this = Self {};

            this
        })
    }
}

impl Render for WindowView {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div().size_full().child("Window")
    }
}
