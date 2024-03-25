use backstage::show::Show;
use gpui::prelude::FluentBuilder;
use gpui::{
    div, px, AnyView, Div, IntoElement, Model, ParentElement, Render, Styled, View, ViewContext,
    VisualContext, WindowContext,
};

pub mod window;

use window::WindowView;

use self::window::executors::ExecutorsWindowDelegate;
use self::window::pool::color::ColorPoolWindowDelegate;
use self::window::pool::group::GroupPoolWindowDelegate;
use super::screen::Screen;
use super::{PoolWindowKind, Window, WindowGrid, WindowKind};
use crate::theme::ActiveTheme;

pub const GRID_CELL_SIZE: usize = 80;

pub struct WindowGridView {
    windows: Vec<AnyView>,
}

impl WindowGridView {
    pub fn build(
        window_grid: Model<WindowGrid>,
        show: Model<Show>,
        cx: &mut ViewContext<Screen>,
    ) -> View<Self> {
        cx.new_view(|cx| {
            cx.observe(&window_grid, {
                let show = show.clone();
                move |this: &mut Self, window_grid, cx| {
                    this.windows = build_windows(window_grid, show.clone(), cx);
                    cx.notify();
                }
            })
            .detach();

            Self {
                windows: build_windows(window_grid, show.clone(), cx),
            }
        })
    }
}

fn build_windows(
    window_grid: Model<WindowGrid>,
    show: Model<Show>,
    cx: &mut WindowContext,
) -> Vec<AnyView> {
    window_grid
        .read(cx)
        .windows()
        .clone()
        .into_iter()
        .map(|(id, window)| build_window_view(id, window, window_grid.clone(), show.clone(), cx))
        .collect()
}

fn build_window_view(
    id: usize,
    window: Window,
    window_grid: Model<WindowGrid>,
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
                WindowView::build(delegate, id, window_grid.clone(), cx).into()
            }
            PoolWindowKind::Group => {
                let delegate = GroupPoolWindowDelegate::new(
                    id,
                    pool_window.scroll_offset,
                    window.bounds,
                    show.clone(),
                );
                WindowView::build(delegate, id, window_grid.clone(), cx).into()
            }
        },
        WindowKind::Executors => {
            let delegate = ExecutorsWindowDelegate::new(show, cx);
            WindowView::build(delegate, id, window_grid.clone(), cx).into()
        }
    }
}

impl Render for WindowGridView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(cx.theme().colors().background)
            .children(self.windows.clone())
    }
}

pub fn grid_div(size: GridSize, origin: Option<GridPoint>) -> Div {
    div()
        .w(px(size.cols as f32 * GRID_CELL_SIZE as f32))
        .h(px(size.rows as f32 * GRID_CELL_SIZE as f32))
        .when_some(origin, |this, origin| {
            this.absolute()
                .top(px(origin.y as f32 * GRID_CELL_SIZE as f32))
                .left(px(origin.x as f32 * GRID_CELL_SIZE as f32))
        })
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct GridBounds {
    pub origin: GridPoint,
    pub size: GridSize,
}

impl GridBounds {
    pub fn new(origin: GridPoint, size: GridSize) -> Self {
        Self { origin, size }
    }

    pub fn cell_count(&self) -> usize {
        self.size.cols * self.size.rows
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
