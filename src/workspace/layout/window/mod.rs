use gpui::{
    div, rgb, AnyView, IntoElement, ParentElement, Render, Styled, ViewContext, WindowContext,
};

use crate::{layout::GridBounds, ui::grid_div};

pub mod pool_item;
pub mod pool_window;

pub struct WindowView {
    bounds: GridBounds,
    content: AnyView,
}

impl WindowView {
    pub fn new(bounds: GridBounds, content: AnyView, cx: &mut WindowContext) -> Self {
        Self { bounds, content }
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

    fn render_content(&self, cx: &mut ViewContext<Self>) -> AnyView {
        todo!();
    }
}

impl Render for WindowView {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        let content = div()
            .bg(rgb(0x202020))
            .rounded_b_md()
            .child(self.content.clone());

        grid_div(self.bounds.size, Some(self.bounds.origin))
            .flex()
            .flex_col()
            .children(self.render_header())
            .child(content)
    }
}
