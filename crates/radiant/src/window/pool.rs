use gpui::{
    div, InteractiveElement, IntoElement, MouseButton, ParentElement, SharedString, Styled,
    ViewContext, WindowContext,
};

use crate::{app::GRID_SIZE, showfile::Window};

use super::{WindowDelegate, WindowView};

pub trait PoolDelegate {
    fn title(&mut self, cx: &mut WindowContext) -> SharedString;

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
        let items = (0..self.window.bounds.area()).map(|id| {
            div()
                .size(GRID_SIZE)
                .child(self.pool_delegate.render_pool_item(id, cx))
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
    fn title(&mut self, _cx: &mut WindowContext) -> SharedString {
        "Group".into()
    }

    fn render_pool_item(&mut self, id: usize, _cx: &mut WindowContext) -> impl IntoElement {
        id.to_string()
    }

    fn on_click_item(&mut self, id: usize, _cx: &mut WindowContext) {
        log::info!("Clicked pool item {id}");
    }
}
