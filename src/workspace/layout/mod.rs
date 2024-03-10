use gpui::{
    div, rgb, IntoElement, Model, ParentElement, Render, Styled, View, ViewContext, VisualContext,
};

pub mod window;

use window::Window;

use crate::show::Show;

use super::screen::Screen;

pub const LAYOUT_CELL_SIZE: usize = 80;

pub struct Layout {
    windows: Vec<View<Window>>,
}

impl Layout {
    pub fn build(show: Model<Show>, cx: &mut ViewContext<Screen>) -> View<Self> {
        cx.new_view(|cx| {
            cx.observe(&show, |this: &mut Self, show, cx| {
                this.windows = show
                    .read(cx)
                    .layout
                    .windows
                    .clone()
                    .iter()
                    .map(|window| Window::build(window, cx))
                    .collect();
            })
            .detach();

            Self {
                windows: Vec::new(),
            }
        })
    }
}

impl Render for Layout {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(rgb(0x181818))
            .children(self.windows.clone())
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct LayoutBounds {
    pub origin: LayoutPoint,
    pub size: LayoutSize,
}

impl LayoutBounds {
    pub fn new(origin: LayoutPoint, size: LayoutSize) -> Self {
        Self { origin, size }
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct LayoutPoint {
    pub x: usize,
    pub y: usize,
}

impl LayoutPoint {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct LayoutSize {
    pub cols: usize,
    pub rows: usize,
}

impl LayoutSize {
    pub fn new(cols: usize, rows: usize) -> Self {
        Self { cols, rows }
    }
}
