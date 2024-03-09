use gpui::{
    div, rgb, AnyView, Context, IntoElement, Model, ParentElement, Render, Styled, View,
    ViewContext, VisualContext, WindowContext,
};

use crate::{layout::GridBounds, ui::grid_div};

use self::pool_window::{PoolWindow, PoolWindowView};

pub mod pool_window;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Window {
    pub kind: WindowKind,
    pub bounds: GridBounds,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum WindowKind {
    Pool(PoolWindow),
}

impl WindowKind {
    pub fn window_title(&self) -> &str {
        match self {
            WindowKind::Pool(pool_window) => pool_window.window_title(),
        }
    }

    pub fn show_header(&self) -> bool {
        match self {
            WindowKind::Pool(_) => false,
        }
    }
}

pub struct WindowView {
    pub window: Model<Window>,
    content: AnyView,
}

impl WindowView {
    pub fn build(window: Model<Window>, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let content = Self::render_content(window.clone(), cx);

            Self { window, content }
        })
    }

    fn render_header(&self, cx: &mut ViewContext<Self>) -> Option<impl IntoElement> {
        let window = self.window.read(cx);

        if !window.kind.show_header() {
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
            .child(window.kind.window_title().to_string());

        Some(header)
    }

    fn render_content(window: Model<Window>, cx: &mut ViewContext<Self>) -> AnyView {
        let window = window.read(cx);

        match window.kind.clone() {
            WindowKind::Pool(pool_window) => {
                PoolWindowView::build(cx.new_model(|_cx| pool_window), cx).into()
            }
        }
    }
}

impl Render for WindowView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let window = self.window.read(cx);

        let content = div()
            .bg(rgb(0x202020))
            .rounded_b_md()
            .child(self.content.clone());

        grid_div(window.bounds.size, Some(window.bounds.origin))
            .flex()
            .flex_col()
            .children(self.render_header(cx))
            .child(content)
    }
}
