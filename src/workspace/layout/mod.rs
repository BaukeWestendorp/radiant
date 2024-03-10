use gpui::{
    div, rgb, IntoElement, ParentElement, Render, Styled, View, ViewContext, WindowContext,
};

pub mod window;

use window::WindowView;

pub const LAYOUT_CELL_SIZE: usize = 80;

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct GridBounds {
    pub origin: GridPoint,
    pub size: GridSize,
}

impl GridBounds {
    pub fn new(origin: GridPoint, size: GridSize) -> Self {
        Self { origin, size }
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct GridPoint {
    pub x: usize,
    pub y: usize,
}

impl GridPoint {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct GridSize {
    pub cols: usize,
    pub rows: usize,
}

impl GridSize {
    pub fn new(cols: usize, rows: usize) -> Self {
        Self { cols, rows }
    }
}

pub struct LayoutView {
    windows: Vec<View<WindowView>>,
}

impl LayoutView {
    pub fn new(cx: &mut WindowContext) -> Self {
        Self {
            windows: Vec::new(),
        }
    }
}

impl Render for LayoutView {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(rgb(0x181818))
            .flex()
            .flex_shrink()
            .children(self.windows.clone())
    }
}
