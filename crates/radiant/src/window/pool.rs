use gpui::{
    div, IntoElement, ParentElement, Render, SharedString, Styled, View, ViewContext,
    VisualContext, WindowContext,
};

use crate::{app::GRID_SIZE, showfile::Window};

use super::{WindowDelegate, WindowView};

pub struct PoolWindowDelegate {
    pool: View<Pool>,
}

impl PoolWindowDelegate {
    pub fn new(window: &Window, cx: &mut WindowContext) -> Self {
        Self {
            pool: Pool::build(window.bounds.size.width, window.bounds.size.height, cx),
        }
    }
}

impl WindowDelegate for PoolWindowDelegate {
    fn title(&mut self, _cx: &mut ViewContext<WindowView<Self>>) -> Option<SharedString> {
        Some("Pool".into())
    }

    fn render_content(&mut self, _cx: &mut ViewContext<WindowView<Self>>) -> impl IntoElement {
        div().size_full().child(self.pool.clone())
    }

    fn render_header(
        &mut self,
        _cx: &mut ViewContext<WindowView<Self>>,
    ) -> Option<impl IntoElement> {
        Option::<gpui::Empty>::None
    }
}

pub struct Pool {
    columns: usize,
    rows: usize,
}

impl Pool {
    pub fn build(columns: usize, rows: usize, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| Self { columns, rows })
    }
}

impl Render for Pool {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        let items = (0..self.columns * self.rows).map(|_ix| {
            div()
                .rounded_md()
                .border()
                .border_color(gpui::white())
                .size(GRID_SIZE)
        });

        div()
            .w(self.columns as f32 * GRID_SIZE)
            .h(self.rows as f32 * GRID_SIZE)
            .overflow_hidden()
            .flex()
            .flex_wrap()
            .children(items)
    }
}
