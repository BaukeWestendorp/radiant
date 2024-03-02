use gpui::{div, rgb, white, IntoElement, ParentElement, Render, Styled, ViewContext};
use serde::{Deserialize, Serialize};

use super::window::Window;

#[derive(Clone, Serialize, Deserialize)]
pub struct Layout {
    windows: Vec<Window>,
}

impl Layout {
    pub fn new() -> Self {
        Self {
            windows: Vec::new(),
        }
    }

    pub fn add_window(&mut self, window: Window) {
        self.windows.push(window);
    }

    pub fn windows(&self) -> &Vec<Window> {
        &self.windows
    }

    pub fn render_content(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .children(self.windows.iter().map(|window| window.get_view(cx)))
            .size_full()
            .bg(rgb(0x202020))
            .p_2()
    }
}

impl Render for Layout {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .child(self.render_content(cx))
            .flex()
            .flex_col()
            .size_full()
            .text_color(white())
            .font("Zed Sans")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LayoutId(pub(crate) usize);
