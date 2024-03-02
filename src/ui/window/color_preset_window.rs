use gpui::{div, rgb, IntoElement, ParentElement, Render, Styled, ViewContext};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ColorPresetWindow {}

impl ColorPresetWindow {
    pub fn new() -> Self {
        Self {}
    }
}

impl Render for ColorPresetWindow {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .child(format!("ColorPresetWindow"))
            .size_full()
            .bg(rgb(0x303030))
    }
}
