use gpui::{App, Entity, FontWeight, Pixels, Size, Window, div, prelude::*, px, size};

mod delegate;
mod state;

pub use delegate::*;
pub use state::*;

use crate::{ActiveTheme, dot_grid, h_flex, v_flex};

#[derive(IntoElement)]
pub struct TileGrid {
    state: Entity<TileGridState>,

    grid_size: Size<u32>,
    cell_size: Pixels,
}

impl TileGrid {
    pub fn new(state: Entity<TileGridState>) -> Self {
        Self { state, grid_size: size(18, 12), cell_size: px(80.0) }
    }

    pub fn grid_size(&self) -> Size<u32> {
        self.grid_size
    }

    pub fn with_grid_size(mut self, grid_size: Size<u32>) -> Self {
        self.grid_size = grid_size;
        self
    }

    pub fn with_cell_size(mut self, cell_size: Pixels) -> Self {
        self.cell_size = cell_size;
        self
    }

    pub fn cell_size(&self) -> Pixels {
        self.cell_size
    }
}

impl RenderOnce for TileGrid {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let tiles = self.state.read(cx).tiles().iter().map(|tile| {
            let title = tile.delegate().title().to_owned();

            let origin = tile.bounds().origin;
            let size = tile.bounds().size;
            let content = tile.delegate().render_content(window, cx);

            let header = h_flex()
                .px_2()
                .w_full()
                .min_h(window.line_height() * 1.5)
                .max_h(window.line_height() * 1.5)
                .bg(cx.theme().bg_tile_header)
                .border_1()
                .border_color(cx.theme().border_tile_header)
                .rounded_t(cx.theme().radius)
                .text_color(cx.theme().fg_tile_header)
                .font_weight(FontWeight::BOLD)
                .child(title);

            div()
                .absolute()
                .bg(cx.theme().bg_secondary)
                .left(origin.x as f32 * self.cell_size())
                .top(origin.y as f32 * self.cell_size())
                .w(size.width as f32 * self.cell_size)
                .h(size.height as f32 * self.cell_size)
                .occlude()
                .overflow_hidden()
                .child(
                    v_flex().size_full().child(header).child(
                        div()
                            .size_full()
                            .border_1()
                            .border_color(cx.theme().border_primary)
                            .rounded_b(cx.theme().radius)
                            .overflow_hidden()
                            .child(content),
                    ),
                )
        });

        div()
            .bg(cx.theme().bg_primary)
            .w(self.grid_size.width as f32 * self.cell_size)
            .h(self.grid_size.height as f32 * self.cell_size)
            .relative()
            .child(dot_grid(self.cell_size, cx.theme().accent).absolute().size_full())
            .child(div().absolute().size_full().child(div().size_full().relative().children(tiles)))
    }
}
