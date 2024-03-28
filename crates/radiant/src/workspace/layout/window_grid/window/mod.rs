use gpui::{
    div, IntoElement, Model, ParentElement, Render, Styled, View, ViewContext, VisualContext,
    WindowContext,
};

use super::{grid_div, GridBounds};
use crate::workspace::layout::WindowGrid;
use theme::ActiveTheme;

pub mod executors;
pub mod fixture_sheet;
pub mod pool;

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

        let background = div()
            .absolute()
            .size_full()
            .rounded_md()
            .border()
            .border_color(cx.theme().colors().border_variant);

        let foreground = div()
            .absolute()
            .size_full()
            .child(self.delegate.render_content(cx));

        let content = div()
            .bg(cx.theme().colors().window_background)
            .size_full()
            .relative()
            .child(background)
            .child(foreground);

        grid_div(bounds.size, Some(bounds.origin))
            .flex()
            .flex_col()
            .shadow_lg()
            .children(self.delegate.render_header(cx))
            .child(content)
    }
}

pub trait WindowDelegate {
    fn title(&self) -> String;

    fn render_header(&self, cx: &mut ViewContext<WindowView<Self>>) -> Option<impl IntoElement>
    where
        Self: Sized,
    {
        let header = div()
            .flex()
            .items_center()
            .px_3()
            .h_10()
            .bg(cx.theme().colors().window_header)
            .border_color(cx.theme().colors().window_header_border)
            .border()
            .rounded_md()
            .child(self.title());

        Some(header)
    }

    fn render_content(&self, cx: &mut ViewContext<WindowView<Self>>) -> impl IntoElement
    where
        Self: Sized;
}
