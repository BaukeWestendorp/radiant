use gpui::{
    div, rgba, IntoElement, Model, ParentElement, Render, Styled, View, ViewContext, VisualContext,
    WindowContext,
};

use crate::color;
use crate::show::{self, Show};
use crate::ui::{grid_div, uniform_grid::uniform_grid};
use crate::workspace::layout::LayoutSize;

use super::pool_item::PoolItem;
use super::Window;

pub struct PoolWindow {
    show_pool_window: show::PoolWindow,
    show_window: show::Window,
    pool_items: Vec<View<PoolItem>>,
}

impl PoolWindow {
    const ROW_SCROLL_OFFSET_MAX: i16 = 10000;
    const SCROLL_SENSITIVITY: f32 = 0.5;

    pub fn build(
        show_pool_window: &show::PoolWindow,
        show_window: &show::Window,
        show: Model<Show>,
        cx: &mut ViewContext<Window>,
    ) -> View<Self> {
        let pool_items = create_pool_items(
            show_window.bounds.size.rows * show_window.bounds.size.cols - 1,
            show_pool_window,
            show,
            cx,
        );

        cx.new_view(|_cx| Self {
            show_pool_window: show_pool_window.clone(),
            show_window: show_window.clone(),
            pool_items,
        })
    }

    /// Number of pool items shown in the window.
    ///
    /// This does not include the header cell.
    pub fn item_count(&self) -> usize {
        self.show_window.bounds.size.rows * self.show_window.bounds.size.cols - 1
    }

    pub fn render_header_cell(&mut self) -> impl IntoElement {
        let title = self.show_pool_window.kind.window_title().to_string();
        let color = color::darken(self.show_pool_window.kind.color(), 0.1);
        let border_color = self.show_pool_window.kind.color();

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

    // fn handle_scroll(&mut self, event: &ScrollWheelEvent, cx: &mut ViewContext<Self>) {
    //     let pool_window = self.pool_window.read(cx);

    //     let scroll_delta_y = event.delta.pixel_delta(px(Self::SCROLL_SENSITIVITY)).y;
    //     self.scroll_offset += scroll_delta_y.0 as i16 * pool_window.size.cols as i16;
    //     self.scroll_offset = self.scroll_offset.clamp(
    //         0,
    //         Self::ROW_SCROLL_OFFSET_MAX - pool_window.item_count() as i16,
    //     );

    //     let row_offset = self.scroll_offset as usize;
    //     self.update_pool_items(cx, |ix, pool_item| {
    //         pool_item.id = ix + row_offset;
    //     });

    //     cx.notify();
    // }
}

fn create_pool_items(
    item_count: usize,
    show_pool_window: &show::PoolWindow,
    show: Model<Show>,
    cx: &mut WindowContext,
) -> Vec<View<PoolItem>> {
    (0..item_count)
        .map(|ix| {
            PoolItem::build(
                // We subtract 1 to the id because we have to skip the header cell.
                ix + show_pool_window.scroll_offset - 1,
                show_pool_window.kind,
                show.clone(),
                cx,
            )
        })
        .collect()
}

impl Render for PoolWindow {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        grid_div(self.show_window.bounds.size, None)
            .size_full()
            .child(uniform_grid(
                cx.view().clone(),
                "pool_items",
                self.show_window.bounds.size.cols,
                self.show_window.bounds.size.rows,
                move |view, _range, _cx| {
                    let header_cell = view.render_header_cell();
                    let mut cells = vec![div().child(header_cell)];

                    cells.extend(view.pool_items.iter().map(|pool_item| {
                        grid_div(LayoutSize::new(1, 1), None).child(pool_item.clone())
                    }));

                    cells
                },
            ))
        // .on_scroll_wheel(cx.listener(Self::handle_scroll))
    }
}
