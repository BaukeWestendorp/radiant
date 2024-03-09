use gpui::{div, prelude::FluentBuilder, px, Div, Styled};

use crate::layout::{GridPoint, GridSize, LAYOUT_CELL_SIZE};

pub mod uniform_grid;

pub fn grid_div(size: GridSize, origin: Option<GridPoint>) -> Div {
    div()
        .w(px(size.cols as f32 * LAYOUT_CELL_SIZE as f32))
        .h(px(size.rows as f32 * LAYOUT_CELL_SIZE as f32))
        .when_some(origin, |this, origin| {
            this.absolute()
                .top(px(origin.y as f32 * LAYOUT_CELL_SIZE as f32))
                .left(px(origin.x as f32 * LAYOUT_CELL_SIZE as f32))
        })
}
