use gpui::{
    div, px, AnyElement, InteractiveElement, Interactivity, IntoElement, Model, ParentElement,
    RenderOnce, ScrollWheelEvent, Styled, ViewContext, WindowContext,
};
use smallvec::SmallVec;

use crate::workspace::layout::window_grid::{grid_div, GridBounds, GridSize};
use crate::workspace::layout::{PoolWindow, Window, WindowGrid, WindowKind};
use theme::ActiveTheme;

use super::{WindowDelegate, WindowView};

pub mod color;
pub mod group;

const ROW_SCROLL_OFFSET_MAX: i32 = 10000;
const SCROLL_SENSITIVITY: f32 = 0.5;

pub trait PoolWindowDelegate
where
    Self: 'static,
{
    fn label(&self) -> String;

    fn bounds(&self) -> &GridBounds;

    fn window_id(&self) -> usize;

    fn scroll_offset(&self) -> i32;

    fn render_item_for_id(&self, id: usize, cx: &mut WindowContext) -> Option<impl IntoElement>;

    fn handle_scroll(
        &mut self,
        event: &ScrollWheelEvent,
        window_grid: Model<WindowGrid>,
        cx: &mut WindowContext,
    ) where
        Self: Sized + 'static,
    {
        let item_count = self.bounds().cell_count();
        let cols = self.bounds().size.cols;

        let scroll_delta_y = event.delta.pixel_delta(px(SCROLL_SENSITIVITY)).y;

        let mut row_offset = self.scroll_offset();
        row_offset += scroll_delta_y.0 as i32 * cols as i32;
        row_offset = row_offset.clamp(0, ROW_SCROLL_OFFSET_MAX - item_count as i32);
        let id = self.window_id();
        window_grid.update(cx, |window_grid, cx| {
            if let Some(Window {
                kind: WindowKind::Pool(PoolWindow { scroll_offset, .. }),
                ..
            }) = window_grid.window_mut(id)
            {
                *scroll_offset = row_offset;
                cx.notify();
            }
        });
    }

    fn handle_click_item(&mut self, _id: usize, _cx: &mut ViewContext<WindowView<Self>>)
    where
        Self: Sized,
    {
    }
}

impl<T: PoolWindowDelegate + 'static> WindowDelegate for T {
    fn title(&self) -> String {
        self.label()
    }

    fn render_header(&self, _cx: &mut ViewContext<WindowView<Self>>) -> Option<impl IntoElement> {
        Option::<gpui::Empty>::None
    }

    fn render_content(&self, cx: &mut ViewContext<WindowView<Self>>) -> impl IntoElement
    where
        Self: Sized,
    {
        let mut grid = vec![];
        for i in 0..self.bounds().cell_count() {
            let id = i + self.scroll_offset() as usize;
            let content = self.render_item_for_id(id, cx);
            let item = div()
                .child(PoolCell::new(id).children(content))
                .on_mouse_down(
                    gpui::MouseButton::Left,
                    cx.listener(move |this, _event, cx| {
                        this.delegate.handle_click_item(id, cx);
                    }),
                );
            grid.push(item);
        }

        div()
            .size_full()
            .flex()
            .flex_wrap()
            .children(grid)
            .on_scroll_wheel(cx.listener(|this, event, cx| {
                this.delegate
                    .handle_scroll(event, this.window_grid.clone(), cx)
            }))
    }
}

#[derive(IntoElement)]
pub struct PoolCell {
    id: usize,
    children: SmallVec<[AnyElement; 1]>,
    interactivity: Interactivity,
}

impl PoolCell {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            children: SmallVec::new(),
            interactivity: Interactivity::default(),
        }
    }
}

impl ParentElement for PoolCell {
    fn extend(&mut self, elements: impl Iterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl InteractiveElement for PoolCell {
    fn interactivity(&mut self) -> &mut gpui::Interactivity {
        &mut self.interactivity
    }
}

impl RenderOnce for PoolCell {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let has_content = !self.children.is_empty();

        grid_div(GridSize::new(1, 1), None)
            .border_color(cx.theme().colors().border_variant)
            .border_1()
            .rounded_md()
            .relative()
            .child(
                div()
                    .size_full()
                    .absolute()
                    .inset_0()
                    .children(self.children),
            )
            .child(
                div()
                    .absolute()
                    .size_full()
                    .text_sm()
                    .text_color(match has_content {
                        true => cx.theme().colors().text,
                        false => cx.theme().colors().text_muted,
                    })
                    .pl(px(4.0))
                    .child(format!("{}", self.id)),
            )
    }
}
