use gpui::{div, prelude::FluentBuilder, px, Div, Point, Size, Styled};

use crate::layout::LAYOUT_CELL_SIZE;

pub mod uniform_grid;

pub fn grid_div(size: Size<usize>, origin: Option<Point<usize>>) -> Div {
    div()
        .w(px(size.width as f32 * LAYOUT_CELL_SIZE as f32))
        .h(px(size.height as f32 * LAYOUT_CELL_SIZE as f32))
        .when_some(origin, |this, origin| {
            this.absolute()
                .top(px(origin.y as f32 * LAYOUT_CELL_SIZE as f32))
                .left(px(origin.x as f32 * LAYOUT_CELL_SIZE as f32))
        })
}
