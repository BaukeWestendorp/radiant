use gpui::*;
use prelude::FluentBuilder;

use crate::{ActiveTheme, StyledExt};

pub const ROW_HEIGHT: Pixels = px(24.0);

pub struct Table<D: TableDelegate> {
    pub delegate: D,
}

impl<D: TableDelegate + 'static> Table<D> {
    pub fn new(delegate: D) -> Self {
        Self { delegate }
    }

    pub fn render_header_row(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let column_count = self.delegate.column_count();
        let cells = (0..column_count)
            .map(|col_ix| {
                div()
                    .w(self.delegate.col_width(col_ix))
                    .h_full()
                    .child(self.delegate.render_header_cell(col_ix, cx))
            })
            .collect::<Vec<_>>();

        div()
            .h_flex()
            .h(ROW_HEIGHT)
            .border_b_1()
            .border_color(cx.theme().border)
            .children(cells)
    }

    pub fn render_body(&self, cx: &ViewContext<Self>) -> impl IntoElement {
        let row_count = self.delegate.row_count();
        div()
            .h_flex()
            .id("table-content")
            .flex_grow()
            .size_full()
            .child(
                uniform_list(cx.view().clone(), "table-content-list", row_count, {
                    move |this, visible_range, cx| {
                        visible_range
                            .map(|row_ix| this.delegate.render_row(row_ix, cx).h(ROW_HEIGHT))
                            .collect::<Vec<_>>()
                    }
                })
                .flex_grow()
                .size_full()
                .with_sizing_behavior(gpui::ListSizingBehavior::Auto),
            )
    }
}

impl<D: TableDelegate + 'static> Render for Table<D> {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .v_flex()
            .flex_none()
            .size_full()
            .child(self.render_header_row(cx))
            .child(self.render_body(cx))
    }
}

pub trait TableDelegate: Sized {
    fn column_count(&self) -> usize;

    fn row_count(&self) -> usize;

    fn column_label(&self, col_ix: usize, cx: &mut ViewContext<Table<Self>>) -> SharedString;

    fn col_width(&self, col_ix: usize) -> Pixels;

    fn render_header_cell(
        &self,
        col_ix: usize,
        cx: &mut ViewContext<Table<Self>>,
    ) -> impl IntoElement {
        div()
            .h_full()
            .border_r_1()
            .px_1()
            .when(col_ix == 0 && col_ix != 0, |this| this.border_l_1())
            .font_weight(FontWeight::BOLD)
            .border_color(cx.theme().border)
            .child(self.column_label(col_ix, cx))
    }

    fn render_row(&self, row_ix: usize, cx: &mut ViewContext<Table<Self>>) -> Div {
        let cells = (0..self.column_count())
            .map(|col_ix| {
                div()
                    .w(self.col_width(col_ix))
                    .h(ROW_HEIGHT)
                    .h_flex()
                    .overflow_hidden()
                    .text_ellipsis()
                    .border_r_1()
                    .when(col_ix == 0 && col_ix != 0, |this| this.border_l_1())
                    .border_color(cx.theme().border_variant)
                    .child(self.render_cell(row_ix, col_ix, cx))
            })
            .collect::<Vec<_>>();

        let bg = if row_ix % 2 == 0 {
            cx.theme().background
        } else {
            cx.theme().background.opacity(0.05)
        };

        div()
            .h_flex()
            .w_full()
            .children(cells)
            .bg(bg)
            .border_b_1()
            .border_color(cx.theme().border_variant)
    }

    fn render_cell(
        &self,
        row_ix: usize,
        col_ix: usize,
        cx: &ViewContext<Table<Self>>,
    ) -> impl IntoElement;
}
