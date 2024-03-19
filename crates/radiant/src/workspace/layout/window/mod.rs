use gpui::{
    div, rgb, IntoElement, Model, ParentElement, Render, Styled, View, ViewContext, VisualContext,
    WindowContext,
};

use crate::show::Show;
use crate::ui::grid_div;

use super::LayoutBounds;

pub mod color_picker;
pub mod fixture_sheet;
pub mod pool;

pub struct Window<D: WindowDelegate> {
    delegate: D,
    window_id: usize,
    show: Model<Show>,
}

impl<D: WindowDelegate + 'static> Window<D> {
    pub fn build(
        delegate: D,
        window_id: usize,
        show: Model<Show>,
        cx: &mut WindowContext,
    ) -> View<Self> {
        cx.new_view(|_cx| Self {
            delegate,
            window_id,
            show,
        })
    }

    fn bounds(&self, cx: &mut WindowContext) -> LayoutBounds {
        self.show
            .read(cx)
            .layout
            .window(self.window_id)
            .unwrap()
            .bounds
    }
}

impl<D: WindowDelegate + 'static> Render for Window<D> {
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

    fn render_header(&self, _cx: &mut ViewContext<Window<Self>>) -> Option<impl IntoElement>
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

    fn render_content(&self, cx: &mut ViewContext<Window<Self>>) -> impl IntoElement
    where
        Self: Sized;
}
