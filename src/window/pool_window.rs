use gpui::{
    div, px, rgb, InteractiveElement, IntoElement, Model, ParentElement, Render, Rgba,
    ScrollWheelEvent, Styled, View, ViewContext, VisualContext, WindowContext,
};

use crate::{
    layout::LAYOUT_CELL_SIZE,
    presets::{ColorPresetId, Preset},
    show::Show,
    ui::uniform_grid::uniform_grid,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PoolWindow {
    pub kind: PoolWindowKind,
    row_scroll_offset: usize,
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
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum PoolWindowKind {
    Color,
}

pub struct PoolWindowView {
    pool_window: Model<PoolWindow>,
}

impl PoolWindowView {
    pub fn build(pool_window: Model<PoolWindow>, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| Self { pool_window })
    }

    fn handle_scroll(&mut self, event: &ScrollWheelEvent, cx: &mut ViewContext<Self>) {
        let scroll_delta_x = event.delta.pixel_delta(px(1.0)).x;
        dbg!(&scroll_delta_x);
        self.pool_window.update(cx, |pool_window, _cx| {
            pool_window.row_scroll_offset = scroll_delta_x.0 as usize;
        });
    }
}

impl Render for PoolWindowView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let pool_window = self.pool_window.read(cx);
        let pool_window_kind = pool_window.kind.clone();

        uniform_grid(
            cx.view().clone(),
            "pool_items",
            pool_window.cols,
            pool_window.rows,
            move |view, range, cx| {
                let pool_window = view.pool_window.read(cx);
                let row_scroll_offset = pool_window.row_scroll_offset;

                let mut items = Vec::new();
                for pool_item_id in range {
                    let pool_item = div()
                        .child(PoolItemView::build(
                            pool_window_kind,
                            pool_item_id + row_scroll_offset,
                            cx,
                        ))
                        .w(px(LAYOUT_CELL_SIZE as f32))
                        .h(px(LAYOUT_CELL_SIZE as f32));

                    items.push(pool_item);
                }
                items
            },
        )
        .on_scroll_wheel(cx.listener(Self::handle_scroll))
        .on_mouse_down(
            gpui::MouseButton::Left,
            cx.listener(|_this, _event, _cx| println!("Hello world")),
        )
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
                            .w_full()
                            .h_full()
                            .flex()
                            .flex_col()
                            .justify_center()
                            .items_center()
                            .text_xs()
                            .child(label)
                            .child(div().size_1_3().bg(color))
                    }
                    None => div().w_full().h_full(),
                }
            }
        };

        div()
            .bg(rgb(0x202020))
            .border_color(rgb(0x303030))
            .border_1()
            .rounded_md()
            .w_full()
            .h_full()
            .relative()
            .child(
                div()
                    .w_full()
                    .h_full()
                    .absolute()
                    .text_sm()
                    .text_color(rgb(0x808080))
                    .pl(px(4.0))
                    .child(format!("{}", self.pool_item_id)),
            )
            .child(div().w_full().h_full().absolute().child(content))
    }
}
