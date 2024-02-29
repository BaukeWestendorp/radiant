use gpui::{
    div, px, rgb, IntoElement, ParentElement, Render, RenderOnce, Styled, View, ViewContext,
    VisualContext, WindowContext,
};

use super::{
    grid::{Grid, GridDelegate},
    pool_grid::PoolGrid,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PoolKind {
    Color,
    Group,
}

impl PoolKind {
    pub fn color(&self) -> gpui::Rgba {
        match self {
            PoolKind::Color => rgb(0xDE3A7F),
            PoolKind::Group => rgb(0x5BB9C1),
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            PoolKind::Color => "Colors",
            PoolKind::Group => "Groups",
        }
    }
}

pub struct Pool {
    grid: View<Grid<PoolDelegate>>,
    x: usize,
    y: usize,
}

impl Pool {
    pub fn new(
        kind: PoolKind,
        rows: usize,
        cols: usize,
        x: usize,
        y: usize,
        cx: &mut WindowContext,
    ) -> Self {
        let delegate = PoolDelegate::new(kind, rows, cols);
        let grid = cx.new_view(|_| Grid::new(delegate));

        Pool { grid, x, y }
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }
}

impl Render for Pool {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div().child(self.grid.clone())
    }
}

struct PoolDelegate {
    rows: usize,
    cols: usize,
    kind: PoolKind,
}

impl PoolDelegate {
    pub fn new(kind: PoolKind, rows: usize, cols: usize) -> Self {
        PoolDelegate { kind, rows, cols }
    }
}

impl GridDelegate for PoolDelegate {
    type Cell = PoolItem;

    fn cell_size(&self) -> usize {
        PoolGrid::GRID_SIZE
    }

    fn grid_gap(&self) -> usize {
        PoolGrid::GRID_GAP
    }

    fn rows(&self) -> usize {
        self.rows
    }

    fn cols(&self) -> usize {
        self.cols
    }

    fn render_cell(&self, row: usize, col: usize, _cx: &mut ViewContext<Grid<Self>>) -> Self::Cell {
        PoolItem::new(self.kind, row * self.cols + col)
    }
}

#[derive(Debug, Clone, IntoElement)]
struct PoolItem {
    pool_kind: PoolKind,
    ix: usize,
}

impl PoolItem {
    pub fn new(pool_kind: PoolKind, ix: usize) -> Self {
        PoolItem { pool_kind, ix }
    }
}

impl RenderOnce for PoolItem {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        if self.ix == 0 {
            let border_color = self.pool_kind.color();
            let mut bg_color = self.pool_kind.color();
            bg_color.a = 0.7;

            div()
                .child(
                    div()
                        .child(self.pool_kind.label())
                        .size_full()
                        .flex()
                        .items_center()
                        .justify_center()
                        .font_weight(gpui::FontWeight::BOLD),
                )
                .size_full()
                .bg(bg_color)
                .border_color(border_color)
                .border_1()
                .rounded_md()
        } else {
            let mut border_color = self.pool_kind.color();
            border_color.a = 0.5;
            div()
                .child(
                    div()
                        .child(format!("{}", self.ix))
                        .pl(px(2.0))
                        .text_sm()
                        .text_color(rgb(0x808080)),
                )
                .size_full()
                .bg(rgb(0x202020))
                .border_1()
                .border_color(border_color)
                .rounded_md()
        }
    }
}
