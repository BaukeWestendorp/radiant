use backstage::show::Show;
use gpui::{
    div, prelude::FluentBuilder, px, rgb, AnyElement, InteractiveElement, Interactivity,
    IntoElement, Model, ParentElement, RenderOnce, ScrollWheelEvent, Styled, ViewContext,
    WindowContext,
};
use smallvec::SmallVec;

use crate::workspace::layout::window_grid::GridBounds;
use crate::workspace::layout::{Window, WindowKind};

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

    fn handle_scroll(&mut self, event: &ScrollWheelEvent, show: Model<Show>, cx: &mut WindowContext)
    where
        Self: Sized + 'static,
    {
        let item_count = self.bounds().cell_count();
        let cols = self.bounds().size.cols;

        let scroll_delta_y = event.delta.pixel_delta(px(SCROLL_SENSITIVITY)).y;

        let mut row_offset = self.scroll_offset();
        row_offset += scroll_delta_y.0 as i32 * cols as i32;
        row_offset = row_offset.clamp(0, ROW_SCROLL_OFFSET_MAX - item_count as i32);

        let id = self.window_id();
        show.update(cx, |show, cx| {
            if let Some(Window {
                kind: WindowKind::Pool(PoolWindow { scroll_offset, .. }),
                ..
            }) = show.layout.window_mut(id)
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
                this.delegate.handle_scroll(event, this.show.clone(), cx)
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
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let has_content = self.children.is_empty();

        div()
            .bg(rgb(0x202020))
            .border_color(rgb(0x404040))
            .border_1()
            .rounded_md()
            .size(px(LAYOUT_CELL_SIZE as f32))
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
                    .text_color(rgb(0xffffff))
                    .when(has_content, |this| this.text_color(rgb(0x808080)))
                    .pl(px(4.0))
                    .child(format!("{}", self.id)),
            )
    }
}
