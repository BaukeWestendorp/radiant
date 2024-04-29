use std::os::unix::raw::nlink_t;

use gpui::{
    div, prelude::FluentBuilder, px, Global, InteractiveElement, IntoElement, MouseButton,
    ParentElement, SharedString, Styled, ViewContext, WindowContext,
};

use crate::{
    app::GRID_SIZE,
    showfile::{Showfile, Window},
};

use super::{WindowDelegate, WindowView};

pub trait PoolDelegate {
    fn title(&self, cx: &mut WindowContext) -> SharedString;

    fn has_content(&self, id: usize, cx: &mut WindowContext) -> bool;

    fn render_pool_item(&mut self, id: usize, cx: &mut WindowContext) -> impl IntoElement;

    fn on_click_item(&mut self, id: usize, cx: &mut WindowContext);
}

pub struct PoolWindowDelegate<D: PoolDelegate> {
    pub pool_delegate: D,
    window: Window,
}

impl<D: PoolDelegate> PoolWindowDelegate<D> {
    pub fn new(pool_delegate: D, window: Window) -> Self {
        Self {
            pool_delegate,
            window,
        }
    }
}

impl<D: PoolDelegate + 'static> WindowDelegate for PoolWindowDelegate<D> {
    fn title(&mut self, cx: &mut ViewContext<WindowView<Self>>) -> Option<SharedString> {
        Some(self.pool_delegate.title(cx))
    }

    fn render_content(&mut self, cx: &mut ViewContext<WindowView<Self>>) -> impl IntoElement {
        let header_cell = div()
            .size(GRID_SIZE)
            .border()
            .border_color(gpui::blue())
            .bg(gpui::rgb(0x000088))
            .rounded_md()
            .flex()
            .justify_center()
            .items_center()
            .child(self.pool_delegate.title(cx));

        let items = (0..self.window.bounds.area()).map(|id| {
            div()
                .size(GRID_SIZE)
                .relative()
                .child(
                    div()
                        .absolute()
                        .size_full()
                        .border()
                        .border_color(gpui::white())
                        .rounded_md()
                        .text_sm()
                        .when(!self.pool_delegate.has_content(id, cx), |this| {
                            this.text_color(gpui::rgb(0xaaaaaa))
                        })
                        .child(id.to_string())
                        .pl_1(),
                )
                .child(
                    div()
                        .absolute()
                        .size_full()
                        .child(self.pool_delegate.render_pool_item(id, cx)),
                )
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, _event, cx| {
                        this.delegate.pool_delegate.on_click_item(id, cx)
                    }),
                )
        });

        div()
            .w(self.window.bounds.size.width as f32 * GRID_SIZE)
            .h(self.window.bounds.size.height as f32 * GRID_SIZE)
            .overflow_hidden()
            .flex()
            .flex_wrap()
            .child(header_cell)
            .children(items)
    }

    fn render_header(
        &mut self,
        _cx: &mut ViewContext<WindowView<Self>>,
    ) -> Option<impl IntoElement> {
        Option::<gpui::Empty>::None
    }
}

pub struct GroupPoolWindowDelegate {}

impl GroupPoolWindowDelegate {
    pub fn new() -> Self {
        Self {}
    }
}

impl PoolDelegate for GroupPoolWindowDelegate {
    fn title(&self, _cx: &mut WindowContext) -> SharedString {
        "Group".into()
    }

    fn has_content(&self, _id: usize, _cx: &mut WindowContext) -> bool {
        false
    }

    fn render_pool_item(&mut self, _id: usize, cx: &mut WindowContext) -> impl IntoElement {
        div()
            .size_full()
            .child(
                div()
                    .h_6()
                    .border_b()
                    .border_color(gpui::white())
                    .flex()
                    .items_center()
                    .justify_end()
                    .pr_1()
                    .text_sm()
                    .child("3 fxt."),
            )
            .child(
                div()
                    .my_auto()
                    .child("Item name long y sdfjl sdlkdf sdjfkl aya"),
            )
            .overflow_hidden()
            .line_height(cx.line_height() * 0.5)
    }

    fn on_click_item(&mut self, id: usize, _cx: &mut WindowContext) {
        log::info!("Clicked pool item {id}");
    }
}
