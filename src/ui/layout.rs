use gpui::{
    div, rgb, white, IntoElement, ParentElement, Render, SharedString, Styled, ViewContext,
};
use serde::{Deserialize, Serialize};

use super::window::Window;

#[derive(Clone, Serialize, Deserialize)]
pub struct Layout {
    label: SharedString,
    windows: Vec<Window>,
}

impl Layout {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string().into(),
            windows: Vec::new(),
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn set_label(&mut self, label: &str) {
        self.label = label.to_string().into();
    }

    pub fn add_window(&mut self, window: Window) {
        self.windows.push(window);
    }

    pub fn windows(&self) -> &Vec<Window> {
        &self.windows
    }

    pub fn render_header(&self) -> impl IntoElement {
        div()
            .child(self.label.to_string())
            .flex()
            .items_center()
            .w_full()
            .px_3()
            .h_10()
    }
}

impl Render for Layout {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        let content = div().size_full().bg(rgb(0x202020));

        div()
            .child(self.render_header())
            .child(content)
            .flex()
            .flex_col()
            .size_full()
            .text_color(white())
            .font("Zed Sans")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LayoutId(pub(crate) usize);
