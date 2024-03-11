use gpui::{
    div, px, rgba, InteractiveElement, IntoElement, Model, ParentElement, Render, ScrollWheelEvent,
    Styled, View, ViewContext, VisualContext, WindowContext,
};

use crate::color;
use crate::show::{self, Show};
use crate::ui::{grid_div, uniform_grid::uniform_grid};
use crate::workspace::layout::LayoutSize;

use super::pool_item::PoolItem;
use super::Window;

pub struct PoolWindow {
    window_id: usize,
    pool_items: Vec<View<PoolItem>>,
    show: Model<Show>,
}

impl PoolWindow {
    const ROW_SCROLL_OFFSET_MAX: i32 = 10000;
    const SCROLL_SENSITIVITY: f32 = 0.5;

    pub fn build(window_id: usize, show: Model<Show>, cx: &mut ViewContext<Window>) -> View<Self> {
        cx.new_view(|cx| {
            cx.observe(&show, move |this: &mut Self, show, cx| {
                dbg!(&show);
                this.pool_items = create_pool_items(window_id, &show, cx)
            })
            .detach();

            Self {
                window_id,
                pool_items: create_pool_items(window_id, &show, cx),
                show,
            }
        })
    }

    /// Number of pool items shown in the window.
    ///
    /// This does not include the header cell.
    pub fn item_count(&self, cx: &mut WindowContext) -> usize {
        show_window(&self.show, self.window_id, cx)
            .bounds
            .cell_count()
            - 1
    }

    pub fn render_header_cell(&mut self, cx: &mut WindowContext) -> impl IntoElement {
        let show_pool_window = show_pool_window(&self.show, self.window_id, cx);
        let title = show_pool_window.kind.window_title().to_string();
        let color = color::darken(show_pool_window.kind.color(), 0.1);
        let border_color = show_pool_window.kind.color();

        grid_div(LayoutSize::new(1, 1), None)
            .bg(color)
            .flex()
            .justify_center()
            .rounded_md()
            .border()
            .border_color(border_color)
            .items_center()
            .child(
                div()
                    .bg(rgba(0x00000040))
                    .px_1()
                    .rounded_sm()
                    .text_sm()
                    .child(title),
            )
    }

    fn handle_scroll(&mut self, event: &ScrollWheelEvent, cx: &mut ViewContext<Self>) {
        let item_count = self.item_count(cx);
        let show_window = show_window(&self.show, self.window_id, cx).clone();
        let show_pool_window = show_pool_window(&self.show, self.window_id, cx);

        let scroll_delta_y = event.delta.pixel_delta(px(Self::SCROLL_SENSITIVITY)).y;

        let mut row_offset = show_pool_window.scroll_offset;
        row_offset += scroll_delta_y.0 as i32 * show_window.bounds.size.cols as i32;
        row_offset = row_offset.clamp(0, Self::ROW_SCROLL_OFFSET_MAX - item_count as i32);

        self.show.update(cx, |show, cx| {
            show.layout
                .pool_window_mut(self.window_id)
                .unwrap()
                .scroll_offset = row_offset;
            cx.notify();
        })
    }
}

fn create_pool_items(
    window_id: usize,
    show: &Model<Show>,
    cx: &mut WindowContext,
) -> Vec<View<PoolItem>> {
    let show_window = show_window(show, window_id, cx).clone();
    let show_pool_window = show_pool_window(show, window_id, cx).clone();
    let item_count = show_window.bounds.cell_count() - 1;

    (0..item_count)
        .map(|ix| {
            PoolItem::build(
                ix + show_pool_window.scroll_offset as usize,
                show_pool_window.kind,
                show.clone(),
                cx,
            )
        })
        .collect()
}

impl Render for PoolWindow {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let show_window = show_window(&self.show, self.window_id, cx).clone();

        grid_div(show_window.bounds.size, None)
            .size_full()
            .child(uniform_grid(
                cx.view().clone(),
                "pool_items",
                show_window.bounds.size.cols,
                show_window.bounds.size.rows,
                move |view, _range, cx| {
                    let header_cell = view.render_header_cell(cx);
                    let mut cells = vec![div().child(header_cell)];

                    cells.extend(view.pool_items.iter().map(|pool_item| {
                        grid_div(LayoutSize::new(1, 1), None).child(pool_item.clone())
                    }));

                    cells
                },
            ))
            .on_scroll_wheel(cx.listener(Self::handle_scroll))
    }
}

fn show_window<'a>(
    show: &Model<Show>,
    window_id: usize,
    cx: &'a mut WindowContext,
) -> &'a show::Window {
    show.read(cx).layout.window(window_id).unwrap()
}

fn show_pool_window<'a>(
    show: &Model<Show>,
    window_id: usize,
    cx: &'a mut WindowContext,
) -> &'a show::PoolWindow {
    show.read(cx).layout.pool_window(window_id).unwrap()
}
