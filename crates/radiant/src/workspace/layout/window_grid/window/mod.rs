use gpui::{
    div, rgb, IntoElement, Model, ParentElement, Render, Styled, View, ViewContext, VisualContext,
    WindowContext,
};

use crate::workspace::layout::WindowGrid;

use super::{grid_div, GridBounds};

pub mod executors;

pub struct WindowView<D: WindowDelegate> {
    delegate: D,
    window_id: usize,
    window_grid: Model<WindowGrid>,
}

impl<D: WindowDelegate + 'static> WindowView<D> {
    pub fn build(
        delegate: D,
        window_id: usize,
        window_grid: Model<WindowGrid>,
        cx: &mut WindowContext,
    ) -> View<Self> {
        cx.new_view(|_cx| Self {
            delegate,
            window_id,
            window_grid,
        })
    }

    fn bounds(&self, cx: &mut WindowContext) -> GridBounds {
        self.window_grid
            .read(cx)
            .window(self.window_id)
            .unwrap()
            .bounds
    }
}

impl<D: WindowDelegate + 'static> Render for WindowView<D> {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let bounds = self.bounds(cx);

        let content = div()
            .bg(rgb(0x202020))
            .size_full()
            .rounded_b_md()
            .child(self.delegate.render_content(cx));

        grid_div(bounds.size, Some(bounds.origin))
            .flex()
            .flex_col()
            .children(self.delegate.render_header(cx))
            .child(content)
    }
}

pub trait WindowDelegate {
    fn title(&self) -> String;

    fn render_header(&self, _cx: &mut ViewContext<WindowView<Self>>) -> Option<impl IntoElement>
    where
        Self: Sized,
    {
        let header = div()
            .flex()
            .items_center()
            .px_3()
            .h_10()
            .bg(rgb(0x222280))
            .border_color(rgb(0x0000ff))
            .border_1()
            .rounded_t_md()
            .child(self.title());

        Some(header)
    }

    fn render_content(&self, cx: &mut ViewContext<WindowView<Self>>) -> impl IntoElement
    where
        Self: Sized;
}
