use gpui::{
    div, rgb, white, IntoElement, ParentElement, Render, RenderOnce, Styled, View, ViewContext,
    VisualContext, WindowContext,
};

use super::{
    grid::{Grid, GridDelegate},
    pool::Pool,
};

pub struct PoolGrid {
    pub pools: Vec<View<Pool>>,
    cols: usize,
    rows: usize,
    grid: View<Grid<PoolGridDelegate>>,
}

impl PoolGrid {
    pub const GRID_SIZE: usize = 80;
    pub const GRID_GAP: usize = 1;

    pub fn new(cx: &mut WindowContext, rows: usize, cols: usize) -> Self {
        let delegate = PoolGridDelegate::new(rows, cols);
        let grid = cx.new_view(|_| Grid::new(delegate));

        PoolGrid {
            pools: Vec::new(),
            cols,
            rows,
            grid,
        }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn add_pool(&mut self, pool: View<Pool>) {
        self.pools.push(pool);
    }
}

impl Render for PoolGrid {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        let pool_views = self.pools.iter().cloned();

        div()
            .relative()
            .child(div().child(self.grid.clone()).absolute())
            .child(div().children(pool_views).absolute())
            .font("Zed Sans")
            .text_color(white())
    }
}

struct PoolGridDelegate {
    rows: usize,
    cols: usize,
}

impl PoolGridDelegate {
    pub fn new(rows: usize, cols: usize) -> Self {
        PoolGridDelegate { rows, cols }
    }
}

impl GridDelegate for PoolGridDelegate {
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

    fn render_cell(&self, _row: usize, _col: usize) -> Self::Cell {
        GridCell
    }
}

#[derive(Debug, Clone, IntoElement)]
struct GridCell;

impl RenderOnce for GridCell {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        div()
            .size_full()
            .bg(rgb(0x101010))
            .border_1()
            .border_color(rgb(0x202020))
            .rounded_md()
    }
}
