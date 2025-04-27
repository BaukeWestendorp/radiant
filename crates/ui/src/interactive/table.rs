use std::collections::HashMap;

use gpui::{Div, ElementId, Pixels, Window, div, prelude::*, px, uniform_list};

use crate::ActiveTheme;

pub struct Table<D: TableDelegate>
where
    D::Column: std::hash::Hash,
{
    delegate: D,
    id: ElementId,
    column_widths: HashMap<D::Column, Pixels>,
}

impl<D: TableDelegate> std::ops::Deref for Table<D> {
    type Target = D;

    fn deref(&self) -> &Self::Target {
        &self.delegate
    }
}

impl<D: TableDelegate> std::ops::DerefMut for Table<D> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.delegate
    }
}

impl<D: TableDelegate> Table<D> {
    pub fn new(
        delegate: D,
        id: impl Into<ElementId>,
        _w: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Self {
        Self { delegate, id: id.into(), column_widths: HashMap::new() }
    }

    pub fn width_for_column(&self, column: &D::Column) -> Pixels {
        *self.column_widths.get(column).unwrap_or(&px(120.0))
    }
}

impl<D: TableDelegate> Table<D> {
    fn render_header_row(&self, w: &mut Window, cx: &mut Context<Table<D>>) -> Div {
        let cells = D::Column::all().iter().enumerate().map(|(ix, col)| {
            div()
                .w(self.width_for_column(col))
                .h_full()
                .border_1()
                .when(ix != 0, |e| e.border_l_0())
                .border_color(cx.theme().colors.border)
                .child(col.label().to_string())
        });

        let total_width = D::Column::all().iter().map(|c| self.width_for_column(c).0).sum();
        div()
            .w(px(total_width))
            .h(self.row_height(w, cx))
            .flex()
            .bg(cx.theme().colors.bg_tertiary)
            .children(cells)
    }
}

impl<D: TableDelegate + 'static> Render for Table<D> {
    fn render(&mut self, w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let header_row = self.render_header_row(w, cx);

        let data_rows = uniform_list(
            cx.entity(),
            self.id.clone(),
            self.rows().len(),
            |this, visible_range, w, cx| {
                this.rows()[visible_range]
                    .iter()
                    .enumerate()
                    .map(|(row_ix, row)| {
                        let alternating_color = cx.theme().colors.bg_alternating;

                        let cells = D::Column::all().iter().enumerate().map(|(col_ix, col)| {
                            div()
                                .w(this.width_for_column(col))
                                .h(this.row_height(w, cx))
                                .border_r_1()
                                .border_b_1()
                                .when(col_ix == 0, |e| e.border_l_1())
                                .border_color(cx.theme().colors.border)
                                .child(row.render_cell(col, w, cx))
                        });

                        div()
                            .when(row_ix % 2 == 0, |e| e.bg(alternating_color))
                            .flex()
                            .children(cells)
                    })
                    .collect()
            },
        )
        .h_full();

        div()
            .h_full()
            .bg(cx.theme().colors.bg_secondary)
            .child(header_row)
            .child(data_rows)
            .debug_below()
    }
}

pub trait TableDelegate: Sized {
    type Row: TableRow<Self>;
    type Column: TableColumn + std::hash::Hash + Eq;

    fn rows(&self) -> &[Self::Row];

    fn row_height(&self, w: &Window, _cx: &Context<Table<Self>>) -> Pixels
    where
        Self: Sized,
    {
        w.line_height()
    }
}

pub trait TableRow<D: TableDelegate> {
    fn render_cell(
        &self,
        column: &D::Column,
        w: &mut Window,
        cx: &mut Context<Table<D>>,
    ) -> impl IntoElement
    where
        Self: Sized;
}

pub trait TableColumn {
    fn label(&self) -> &str;

    fn all<'a>() -> &'a [Self]
    where
        Self: Sized;
}
