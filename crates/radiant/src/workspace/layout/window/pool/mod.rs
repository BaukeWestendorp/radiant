use gpui::{
    div, prelude::FluentBuilder, px, rgb, AnyElement, IntoElement, ParentElement, RenderOnce,
    Styled, ViewContext, WindowContext,
};
use smallvec::SmallVec;

use crate::workspace::layout::{LayoutBounds, LAYOUT_CELL_SIZE};

use super::{Window, WindowDelegate};

pub mod color;
pub mod group;

pub trait PoolWindowDelegate {
    fn label(&self) -> String;

    fn bounds(&self, _cx: &mut WindowContext) -> &LayoutBounds;

    fn scroll_offset(&self, _cx: &mut WindowContext) -> usize;

    fn render_item_for_id(&self, id: usize, cx: &mut WindowContext) -> Option<impl IntoElement>;
}

impl<T: PoolWindowDelegate> WindowDelegate for T {
    fn title(&self) -> String {
        self.label()
    }

    fn render_header(&self, _cx: &mut ViewContext<Window<Self>>) -> Option<impl IntoElement> {
        Option::<gpui::Empty>::None
    }

    fn render_content(&self, cx: &mut ViewContext<Window<Self>>) -> impl IntoElement
    where
        Self: Sized,
    {
        let mut grid = vec![];
        for i in 0..self.bounds(cx).cell_count() {
            let id = i + self.scroll_offset(cx);
            let content = self.render_item_for_id(id, cx);
            let item = PoolCell::new(id).children(content);
            grid.push(item);
        }

        div().size_full().flex().flex_wrap().children(grid)
    }
}

#[derive(IntoElement)]
pub struct PoolCell {
    id: usize,
    children: SmallVec<[AnyElement; 1]>,
}

impl PoolCell {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            children: SmallVec::new(),
        }
    }
}

impl ParentElement for PoolCell {
    fn extend(&mut self, elements: impl Iterator<Item = AnyElement>) {
        self.children.extend(elements)
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
