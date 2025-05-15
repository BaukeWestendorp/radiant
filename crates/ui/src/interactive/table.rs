use std::collections::HashMap;

use gpui::{App, Div, ElementId, Pixels, Window, div, prelude::*, px, uniform_list};

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
        let cells = D::Column::all().iter().map(|col| {
            div()
                .w(self.width_for_column(col))
                .h_full()
                .px_1()
                .border_r_1()
                .border_color(cx.theme().colors.border)
                .text_ellipsis()
                .child(col.label().to_string())
        });

        div()
            .w_full()
            .when_some(self.row_height(w, cx), |e, h| e.h(h))
            .flex()
            .bg(cx.theme().colors.bg_tertiary)
            .border_color(cx.theme().colors.border)
            .border_b_1()
            .children(cells)
    }
}

impl<D: TableDelegate + 'static> Render for Table<D> {
    fn render(&mut self, w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let header_row = self.render_header_row(w, cx);

        let rows = self.rows(cx);
        let data_rows = uniform_list(
            cx.entity(),
            self.id.clone(),
            rows.len(),
            move |this, visible_range, w, cx| {
                rows[visible_range]
                    .iter()
                    .enumerate()
                    .map(|(row_ix, row)| {
                        let alternating_color = cx.theme().colors.bg_alternating;

                        let cells = D::Column::all().iter().map(|col| {
                            div()
                                .w(this.width_for_column(col))
                                .when_some(this.row_height(w, cx), |e, h| e.h(h))
                                .border_b_1()
                                .border_r_1()
                                .border_color(cx.theme().colors.border)
                                .child(row.render_cell(col, w, cx))
                        });

                        div()
                            .w_full()
                            .when(row_ix % 2 == 0, |e| e.bg(alternating_color))
                            .flex()
                            .children(cells)
                    })
                    .collect()
            },
        )
        .size_full();

        div().h_full().bg(cx.theme().colors.bg_secondary).child(header_row).child(data_rows)
    }
}

pub trait TableDelegate: Sized {
    type Row: TableRow<Self>;

    type Column: TableColumn + std::hash::Hash + Eq;

    fn rows(&self, cx: &App) -> Vec<Self::Row>;

    fn row_height(&self, _w: &Window, _cx: &Context<Table<Self>>) -> Option<Pixels>
    where
        Self: Sized,
    {
        None
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
