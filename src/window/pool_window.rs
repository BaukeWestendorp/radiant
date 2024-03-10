use gpui::{
    div, prelude::FluentBuilder, px, rgb, rgba, InteractiveElement, IntoElement, Model,
    ParentElement, Render, Rgba, ScrollWheelEvent, Styled, View, ViewContext, VisualContext,
    WindowContext,
};

use crate::{
    color,
    layout::GridSize,
    presets::{ColorPreset, ColorPresetId, Preset},
    show::Show,
    ui::{grid_div, uniform_grid::uniform_grid},
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PoolWindow {
    pub kind: PoolWindowKind,
    size: GridSize,
    scroll_offset: i16,
}

impl PoolWindow {
    pub fn new(kind: PoolWindowKind, size: GridSize, scroll_offset: i16) -> Self {
        Self {
            kind,
            scroll_offset,
            size,
        }
    }

    pub fn window_title(&self) -> &str {
        match &self.kind {
            PoolWindowKind::Color => "Color",
        }
    }

    pub fn color(&self) -> Rgba {
        match &self.kind {
            PoolWindowKind::Color => rgb(0x27D0CD),
        }
    }

    pub fn item_count(&self) -> usize {
        self.size.rows * self.size.cols - 1
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum PoolWindowKind {
    Color,
}

pub struct PoolWindowView {
    pool_window: Model<PoolWindow>,
    scroll_offset: i16,
    header_cell: View<HeaderCellView>,
    pool_items: Vec<View<PoolItemView>>,
}

impl PoolWindowView {
    const ROW_SCROLL_OFFSET_MAX: i16 = 10000;
    const SCROLL_SENSITIVITY: f32 = 0.5;

    pub fn build(pool_window: Model<PoolWindow>, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| Self {
            scroll_offset: pool_window.read(cx).scroll_offset,
            pool_items: Self::get_pool_items(pool_window.clone(), cx),
            header_cell: HeaderCellView::build(pool_window.clone(), cx),
            pool_window,
        })
    }

    fn get_pool_items(
        pool_window: Model<PoolWindow>,
        cx: &mut ViewContext<Self>,
    ) -> Vec<View<PoolItemView>> {
        let total_items = pool_window.read(cx).item_count();
        let range = 0..total_items;
        range
            .map(|ix| {
                let pool_window_model = pool_window.clone();
                let pool_window = pool_window.read(cx);
                let id = ix + pool_window.scroll_offset as usize * pool_window.size.cols;

                let pool_item = PoolItemView::build(pool_window_model, id, cx);
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
        self.scroll_offset += scroll_delta_y.0 as i16 * pool_window.size.cols as i16;
        self.scroll_offset = self.scroll_offset.clamp(
            0,
            Self::ROW_SCROLL_OFFSET_MAX - pool_window.item_count() as i16,
        );

        let row_offset = self.scroll_offset as usize;
        self.update_pool_items(cx, |ix, pool_item| {
            pool_item.id = ix + row_offset;
        });

        cx.notify();
    }
}

impl Render for PoolWindowView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let pool_window = self.pool_window.read(cx);
        let header_cell = self.header_cell.clone();

        grid_div(pool_window.size, None)
            .child(uniform_grid(
                cx.view().clone(),
                "pool_items",
                pool_window.size.cols,
                pool_window.size.rows,
                move |view, _range, _cx| {
                    let mut cells = vec![div().child(header_cell.clone())];

                    cells.extend(view.pool_items.iter().map(|pool_cell| {
                        grid_div(GridSize::new(1, 1), None).child(pool_cell.clone())
                    }));

                    cells
                },
            ))
            .on_scroll_wheel(cx.listener(Self::handle_scroll))
    }
}

struct HeaderCellView {
    pool_window: Model<PoolWindow>,
}

impl HeaderCellView {
    pub fn build(pool_window: Model<PoolWindow>, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| Self { pool_window })
    }
}

impl Render for HeaderCellView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let pool_window = self.pool_window.read(cx);
        let title = pool_window.window_title().to_string();
        let color = color::darken(pool_window.color(), 0.1);
        let border_color = pool_window.color();

        grid_div(GridSize::new(1, 1), None)
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
}

struct PoolItemView {
    pool_window: Model<PoolWindow>,
    id: usize,
}

impl PoolItemView {
    pub fn build(
        pool_window: Model<PoolWindow>,
        id: usize,
        cx: &mut ViewContext<PoolWindowView>,
    ) -> View<Self> {
        cx.new_view(|_cx| Self { pool_window, id })
    }

    fn render_color_pool_item(&self, color_preset: &ColorPreset) -> impl IntoElement {
        let color: Rgba = color_preset.color.clone().into();
        let label = color_preset.label().to_string();

        div()
            .size_full()
            .flex()
            .flex_col_reverse()
            .bg(color::opacify(color, 0.8))
            .rounded_md()
            .child(
                div()
                    .bg(color)
                    .h_1_3()
                    .border_t()
                    .rounded_b_md()
                    .border_color(color::darken(color, 0.4)),
            )
            .child(div().flex().justify_center().text_xs().child(label))
    }

    fn render_empty_pool_item(&self) -> impl IntoElement {
        div().size_full()
    }
}

impl Render for PoolItemView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let pool_window = self.pool_window.read(cx);

        let mut has_content = false;
        let content = match &pool_window.kind {
            PoolWindowKind::Color => {
                let color_preset = Show::global(cx)
                    .presets
                    .color_preset(ColorPresetId(self.id));

                match color_preset {
                    Some(color_preset) => {
                        has_content = true;
                        self.render_color_pool_item(color_preset).into_any_element()
                    }
                    None => self.render_empty_pool_item().into_any_element(),
                }
            }
        };

        div()
            .bg(rgb(0x202020))
            .border_color(color::darken(pool_window.color(), 0.7))
            .border_1()
            .rounded_md()
            .size_full()
            .relative()
            .child(div().size_full().absolute().inset_0().child(content))
            .child(
                div()
                    .absolute()
                    .size_full()
                    .text_sm()
                    .text_color(rgb(0x808080))
                    .when(has_content, |this| this.text_color(rgb(0xffffff)))
                    .pl(px(4.0))
                    .child(format!("{}", self.id)),
            )
    }
}
