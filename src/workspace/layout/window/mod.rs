use gpui::{
    div, rgb, AnyView, IntoElement, ParentElement, Render, Styled, View, ViewContext,
    VisualContext, WindowContext,
};

use crate::show::{self, WindowKind};
use crate::ui::grid_div;

use super::LayoutBounds;

// pub mod pool_item;
// pub mod pool_window;

pub struct Window {
    bounds: LayoutBounds,
    kind: WindowKind,
}

impl Window {
    pub fn build(show_window: &show::Window, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| Self {
            bounds: show_window.bounds,
            kind: show_window.kind,
        })
    }

    fn render_header(&self) -> Option<impl IntoElement> {
        if !self.kind.show_header() {
            return None;
        }

        let header = div()
            .flex()
            .items_center()
            .px_3()
            .h_10()
            .bg(rgb(0x222280))
            .border_color(rgb(0x0000ff))
            .border_1()
            .rounded_t_md()
            .child(self.kind.window_title().to_string());

        Some(header)
    }

    fn render_content(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        match self.kind {
            WindowKind::Pool => div().child("PoolWindow::build()"),
        }
    }
}

impl Render for Window {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let content = grid_div(self.bounds.size, Some(self.bounds.origin))
            .bg(rgb(0x202020))
            .child(self.render_content(cx));

        grid_div(self.bounds.size, Some(self.bounds.origin))
            .flex()
            .flex_col()
            .children(self.render_header())
            .child(content)
    }
}

pub trait WindowContent: Render {
    fn render_test(&mut self) {}
}
