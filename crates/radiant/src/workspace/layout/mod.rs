use gpui::{
    div, rgb, AnyView, IntoElement, Model, ParentElement, Render, Styled, View, ViewContext,
    VisualContext, WindowContext,
};

pub mod window;

use window::Window;

use crate::show::{
    self,
    layout::{PoolWindowKind, WindowKind},
    Show,
};

use self::window::{
    color_picker::ColorPickerWindowDelegate,
    fixture_sheet::FixtureSheetWindowDelegate,
    pool::{color::ColorPoolWindowDelegate, group::GroupPoolWindowDelegate},
};

use super::screen::Screen;

pub const LAYOUT_CELL_SIZE: usize = 80;

pub struct Layout {
    windows: Vec<AnyView>,
}

impl Layout {
    pub fn build(show: Model<Show>, cx: &mut ViewContext<Screen>) -> View<Self> {
        cx.new_view(|cx| {
            cx.observe(&show, |this: &mut Self, show, cx| {
                this.windows = show
                    .read(cx)
                    .layout
                    .windows()
                    .clone()
                    .into_iter()
                    .map(|(id, window)| build_window_view(id, window, show.clone(), cx))
                    .collect();
                cx.notify();
            })
            .detach();

            Self {
                windows: Vec::new(),
            }
        })
    }
}

pub fn build_window_view(
    id: usize,
    window: show::Window,
    show: Model<Show>,
    cx: &mut WindowContext,
) -> AnyView {
    match &window.kind {
        WindowKind::Pool(pool_window) => match &pool_window.kind {
            PoolWindowKind::Color => {
                let delegate = ColorPoolWindowDelegate::new(
                    id,
                    pool_window.scroll_offset,
                    window.bounds,
                    show.clone(),
                );
                Window::build(delegate, id, show.clone(), cx).into()
            }
            PoolWindowKind::Group => {
                let delegate = GroupPoolWindowDelegate::new(
                    id,
                    pool_window.scroll_offset,
                    window.bounds,
                    show.clone(),
                );
                Window::build(delegate, id, show.clone(), cx).into()
            }
        },
        WindowKind::ColorPicker => {
            let delegate = ColorPickerWindowDelegate::new(cx);
            Window::build(delegate, id, show.clone(), cx).into()
        }
        WindowKind::FixtureSheet => {
            let delegate = FixtureSheetWindowDelegate::new(show.clone(), cx);
            Window::build(delegate, id, show.clone(), cx).into()
        }
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

    pub fn cell_count(&self) -> usize {
        self.size.cols * self.size.rows
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
