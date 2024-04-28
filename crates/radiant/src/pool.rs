use gpui::{
    div, px, IntoElement, ParentElement, Render, Styled, View, ViewContext, VisualContext,
    WindowContext,
};

use crate::app::GRID_SIZE;

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
