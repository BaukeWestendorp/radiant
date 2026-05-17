use gpui::prelude::*;
use gpui::{App, Entity, FontWeight, Pixels, Size, Window, div, px, size};

mod delegate;
mod state;

pub use delegate::*;
pub use state::*;

use crate::{ActiveTheme, dot_grid, h_flex, v_flex};

#[derive(IntoElement)]
pub struct TileGrid {
    state: Entity<TileGridState>,

    grid_size: Size<u32>,
    cell_size: Size<Pixels>,
}

impl TileGrid {
    pub fn new(state: Entity<TileGridState>) -> Self {
        Self { state, grid_size: size(18, 12), cell_size: size(px(80.0), px(80.0)) }
    }

    pub fn grid_size(&self) -> Size<u32> {
        self.grid_size
    }

    pub fn with_grid_size(mut self, grid_size: Size<u32>) -> Self {
        self.grid_size = grid_size;
        self
    }

    pub fn with_cell_size(mut self, cell_size: Size<Pixels>) -> Self {
        self.cell_size = cell_size;
        self
    }

    pub fn cell_size(&self) -> Size<Pixels> {
        self.cell_size
    }
}

impl RenderOnce for TileGrid {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let tiles = self.state.update(cx, |state, cx| {
            state
                .tiles()
                .iter()
                .map(|tile| {
                    let title = tile.delegate().title(cx);

                    let origin = tile.bounds().origin;
                    let size = tile.bounds().size;
                    let show_header = tile.delegate().show_header(cx);
                    let content = tile.delegate().render_content(tile.bounds(), window, cx);

                    let header = show_header.then(|| {
                        h_flex()
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
                            .child(title)
                    });

                    div()
                        .absolute()
                        .bg(cx.theme().bg_primary)
                        .left(origin.x as f32 * self.cell_size().width)
                        .top(origin.y as f32 * self.cell_size().height)
                        .w(size.width as f32 * self.cell_size().width)
                        .h(size.height as f32 * self.cell_size().height)
                        .occlude()
                        .overflow_hidden()
                        .child(
                            v_flex().size_full().children(header).child(
                                div()
                                    .relative()
                                    .size_full()
                                    .child(
                                        div()
                                            .absolute()
                                            .size_full()
                                            .overflow_hidden()
                                            .child(content),
                                    )
                                    .child(
                                        div()
                                            .absolute()
                                            .size_full()
                                            .border_1()
                                            .border_color(cx.theme().border_primary)
                                            .when(!show_header, |e| e.rounded_t(cx.theme().radius))
                                            .rounded_b(cx.theme().radius),
                                    ),
                            ),
                        )
                        .into_any_element()
                })
                .collect::<Vec<_>>()
        });

        div()
            .absolute()
            .size_full()
            .overflow_hidden()
            .bg(cx.theme().bg_primary)
            .w(self.grid_size.width as f32 * self.cell_size().width)
            .h(self.grid_size.height as f32 * self.cell_size().height)
            .relative()
            .child(
                dot_grid(self.cell_size().width, self.cell_size().height, cx.theme().accent)
                    .absolute()
                    .size_full(),
            )
            .child(div().absolute().size_full().child(div().size_full().relative().children(tiles)))
    }
}
