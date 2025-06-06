use gpui::{Window, div, prelude::*};

pub struct AttributeEditorFrame {}

impl AttributeEditorFrame {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {}
    }
}

impl Render for AttributeEditorFrame {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().child("Hell World")
    }
}
