use gpui::{
    div, px, rgb, size, InteractiveElement, IntoElement, Model, ParentElement, Render, Rgba,
    ScrollWheelEvent, Styled, View, ViewContext, VisualContext, WindowContext,
};

use crate::{
    presets::{ColorPresetId, Preset},
    show::Show,
    ui::{grid_div, uniform_grid::uniform_grid},
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PoolWindow {
    pub kind: PoolWindowKind,
    row_scroll_offset: i16,
    cols: usize,
    rows: usize,
}

impl PoolWindow {
    pub fn new(kind: PoolWindowKind, cols: usize, rows: usize) -> Self {
        Self {
            kind,
            row_scroll_offset: 0,
            cols,
            rows,
        }
    }

    pub fn window_title(&self) -> &str {
        match &self.kind {
            PoolWindowKind::Color => "Color Pool",
        }
    }

    pub fn cell_count(&self) -> usize {
        self.cols * self.rows
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum PoolWindowKind {
    Color,
}

pub struct PoolWindowView {
    pool_window: Model<PoolWindow>,
    row_scroll_offset: i16,
    pool_items: Vec<View<PoolItemView>>,
}

impl PoolWindowView {
    const ROW_SCROLL_OFFSET_MAX: i16 = 10000;
    const SCROLL_SENSITIVITY: f32 = 0.5;

    pub fn build(pool_window: Model<PoolWindow>, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| Self {
            row_scroll_offset: pool_window.read(cx).row_scroll_offset,
            pool_items: Self::get_pool_items(pool_window.clone(), cx),
            pool_window,
        })
    }

    fn get_pool_items(
        pool_window: Model<PoolWindow>,
        cx: &mut ViewContext<Self>,
    ) -> Vec<View<PoolItemView>> {
        let total_cells = pool_window.read(cx).cell_count();
        let range = 0..total_cells;
        range
            .map(|ix| {
                let pool_window = pool_window.read(cx);
                let id = ix + pool_window.row_scroll_offset as usize * pool_window.cols;

                let pool_item = PoolItemView::build(pool_window.kind, id, cx);
                pool_item
            })
            .collect()
    }

    fn update_pool_items(&mut self, cx: &mut WindowContext, f: impl Fn(usize, &mut PoolItemView)) {
        for (ix, pool_item) in self.pool_items.iter_mut().enumerate() {
            pool_item.update(cx, |pool_item, _cx| f(ix, pool_item));
        }
    }

    fn handle_scroll(&mut self, event: &ScrollWheelEvent, cx: &mut ViewContext<Self>) {
        let pool_window = self.pool_window.read(cx);

        let scroll_delta_y = event.delta.pixel_delta(px(Self::SCROLL_SENSITIVITY)).y;
        self.row_scroll_offset += scroll_delta_y.0 as i16 * pool_window.cols as i16;
        self.row_scroll_offset = self.row_scroll_offset.clamp(
            0,
            Self::ROW_SCROLL_OFFSET_MAX - pool_window.cell_count() as i16,
        );

        let row_offset = self.row_scroll_offset as usize;
        self.update_pool_items(cx, |ix, pool_item| {
            pool_item.pool_item_id = ix + row_offset;
        });

        cx.notify();
    }
}

impl Render for PoolWindowView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let pool_window = self.pool_window.read(cx);

        grid_div(size(pool_window.cols, pool_window.rows), None)
            .child(uniform_grid(
                cx.view().clone(),
                "pool_items",
                pool_window.cols,
                pool_window.rows,
                |view, _range, _cx| {
                    view.pool_items
                        .iter()
                        .map(|pool_item| grid_div(size(1, 1), None).child(pool_item.clone()))
                        .collect::<Vec<_>>()
                },
            ))
            .on_scroll_wheel(cx.listener(Self::handle_scroll))
    }
}

pub struct PoolItemView {
    pool_window_kind: PoolWindowKind,
    pool_item_id: usize,
}

impl PoolItemView {
    pub fn build(
        pool_window_kind: PoolWindowKind,
        pool_item_id: usize,
        cx: &mut ViewContext<PoolWindowView>,
    ) -> View<Self> {
        cx.new_view(|_cx| Self {
            pool_window_kind,
            pool_item_id,
        })
    }
}

impl Render for PoolItemView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let content = match self.pool_window_kind {
            PoolWindowKind::Color => {
                let color_preset = Show::global(cx)
                    .presets
                    .color_preset(ColorPresetId(self.pool_item_id));

                match color_preset {
                    Some(color_preset) => {
                        let color: Rgba = color_preset.color.clone().into();
                        let label = color_preset.label().to_string();

                        div()
                            .size_full()
                            .flex()
                            .flex_col()
                            .justify_center()
                            .items_center()
                            .text_xs()
                            .child(label)
                            .child(div().size_1_3().bg(color))
                    }
                    None => div().size_full(),
                }
            }
        };

        div()
            .bg(rgb(0x202020))
            .border_color(rgb(0x303030))
            .border_1()
            .rounded_md()
            .size_full()
            .relative()
            .child(
                div()
                    .size_full()
                    .absolute()
                    .text_sm()
                    .text_color(rgb(0x808080))
                    .pl(px(4.0))
                    .child(format!("{}", self.pool_item_id)),
            )
            .child(div().size_full().absolute().child(content))
    }
}
