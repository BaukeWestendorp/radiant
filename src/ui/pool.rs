use gpui::{
    div, px, rgb, IntoElement, ParentElement, Render, RenderOnce, Styled, View, ViewContext,
    VisualContext, WindowContext,
};

use super::{
    grid::{Grid, GridDelegate},
    pool_grid::PoolGrid,
};

pub struct Pool {
    cols: usize,
    rows: usize,
    grid: View<Grid<PoolDelegate>>,
}

impl Pool {
    pub fn new(cx: &mut WindowContext, rows: usize, cols: usize) -> Self {
        let delegate = PoolDelegate::new(rows, cols);
        let grid = cx.new_view(|_| Grid::new(delegate));

        Pool { cols, rows, grid }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
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
}

impl PoolDelegate {
    pub fn new(rows: usize, cols: usize) -> Self {
        PoolDelegate { rows, cols }
    }
}

impl GridDelegate for PoolDelegate {
    type Cell = GridCell;

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

    fn render_cell(&self, row: usize, col: usize) -> Self::Cell {
        GridCell::new(row * self.cols + col)
    }
}

#[derive(Debug, Clone, IntoElement)]
struct GridCell {
    ix: usize,
}

impl GridCell {
    pub fn new(ix: usize) -> Self {
        GridCell { ix }
    }
}

impl RenderOnce for GridCell {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
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
            .border_color(rgb(0x303030))
            .rounded_md()
    }
}
