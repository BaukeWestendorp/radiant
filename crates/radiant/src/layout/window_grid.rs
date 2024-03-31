use std::collections::HashMap;

use backstage::show::Show;
use gpui::prelude::FluentBuilder;
use gpui::{
    div, px, AnyView, Div, IntoElement, Model, ParentElement, Render, Styled, View, ViewContext,
    VisualContext, WindowContext,
};
use theme::ActiveTheme;

use super::{
    ColorPoolWindowDelegate, ExecutorsWindowDelegate, FixtureSheetWindowDelegate,
    GroupPoolWindowDelegate, PoolWindowKind, Screen, Window, WindowKind, WindowView,
};

pub const GRID_CELL_SIZE: usize = 80;

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct WindowGrid {
    id: usize,
    windows: HashMap<usize, Window>,
}

impl WindowGrid {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn windows(&self) -> &HashMap<usize, Window> {
        &self.windows
    }

    pub fn window(&self, id: usize) -> Option<&Window> {
        self.windows.get(&id)
    }

    pub fn window_mut(&mut self, id: usize) -> Option<&mut Window> {
        self.windows.get_mut(&id)
    }

    pub fn add_window(&mut self, window: Window) -> usize {
        let id = self.new_window_id();
        self.windows.insert(id, window);
        id
    }

    fn new_window_id(&self) -> usize {
        // TODO: This is not a good way to get a new id. This only works if you can't
        // remove colors.
        self.windows.len()
    }
}

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
            PoolWindowKind::ColorPreset => {
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
        WindowKind::FixtureSheet => {
            let delegate = FixtureSheetWindowDelegate::new(show, cx);
            WindowView::build(delegate, id, window_grid.clone(), cx).into()
        }
    }
}

impl Render for WindowGridView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let mut grid_dots = vec![];
        for y in 0..20 {
            for x in 0..20 {
                let dot = div()
                    .absolute()
                    .size(px(2.0))
                    .rounded_full()
                    .bg(cx.theme().colors().border)
                    .top(px(GRID_CELL_SIZE as f32 * y as f32 - 1.0))
                    .left(px(GRID_CELL_SIZE as f32 * x as f32 - 1.0));

                grid_dots.push(dot);
            }
        }

        let grid = div().size_full().absolute().children(grid_dots);
        let windows = div().size_full().absolute().children(self.windows.clone());

        div()
            .size_full()
            .relative()
            .bg(cx.theme().colors().background)
            .child(grid)
            .child(windows)
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
