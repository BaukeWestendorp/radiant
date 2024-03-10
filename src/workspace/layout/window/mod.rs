use gpui::{
    div, rgb, AnyView, IntoElement, Model, ParentElement, Render, Styled, View, ViewContext,
    VisualContext, WindowContext,
};

use crate::show::{self, Show, WindowKind};
use crate::ui::grid_div;

use self::pool_window::PoolWindow;

pub mod pool_item;
pub mod pool_window;

pub struct Window {
    show: Model<Show>,
    show_window: show::Window,
}

impl Window {
    pub fn build(
        show_window: &show::Window,
        show: Model<Show>,
        cx: &mut WindowContext,
    ) -> View<Self> {
        cx.new_view(|_cx| Self {
            show,
            show_window: show_window.clone(),
        })
    }

    fn render_header(&self) -> Option<impl IntoElement> {
        if !self.show_window.kind.show_header() {
            return None;
        }

        let window_title = self.show_window.kind.window_title().to_string();

        let header = div()
            .flex()
            .items_center()
            .px_3()
            .h_10()
            .bg(rgb(0x222280))
            .border_color(rgb(0x0000ff))
            .border_1()
            .rounded_t_md()
            .child(window_title);

        Some(header)
    }

    fn render_content(&self, cx: &mut ViewContext<Self>) -> AnyView {
        match &self.show_window.kind {
            WindowKind::Pool(pool_window) => {
                PoolWindow::build(pool_window, &self.show_window, self.show.clone(), cx).into()
            }
        }
    }
}

impl Render for Window {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let content = div()
            .bg(rgb(0x202020))
            .size_full()
            .child(self.render_content(cx));

        grid_div(
            self.show_window.bounds.size,
            Some(self.show_window.bounds.origin),
        )
        .flex()
        .flex_col()
        .children(self.render_header())
        .child(content)
    }
}
