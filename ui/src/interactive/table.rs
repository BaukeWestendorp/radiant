use std::collections::HashMap;
use std::ops::Range;

use gpui::prelude::*;
use gpui::{App, ClickEvent, Div, ElementId, Pixels, Window, div, px, uniform_list};

use crate::{ActiveTheme, InteractiveColor};

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
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Self {
        Self { delegate, id: id.into(), column_widths: HashMap::new() }
    }

    pub fn set_column_width(&mut self, column: D::Column, width: Pixels) {
        self.column_widths.insert(column, width);
    }

    pub fn column_width(&self, column: &D::Column) -> Pixels {
        *self.column_widths.get(column).unwrap_or(&px(120.0))
    }
}

impl<D: TableDelegate> Table<D> {
    fn render_header_row(&self, window: &mut Window, cx: &mut Context<Table<D>>) -> Div {
        let row_height = self.row_height(window, cx);

        let cells = D::Column::all().iter().map(|col| {
            div()
                .w(self.column_width(col))
                .h_full()
                .px_1()
                .border_r_1()
                .border_color(cx.theme().colors.border)
                .text_ellipsis()
                .child(col.label().to_string())
        });

        div()
            .w_full()
            .when_some(row_height, |e, h| e.h(h))
            .flex()
            .bg(cx.theme().colors.bg_tertiary)
            .border_color(cx.theme().colors.border)
            .border_b_1()
            .children(cells)
    }
}

impl<D: TableDelegate + 'static> Render for Table<D>
where
    D::Row: Clone,
{
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let header_row = self.render_header_row(window, cx);

        let rows = self.rows(cx);
        let data_rows = uniform_list(
            self.id.clone(),
            rows.len(),
            cx.processor(move |this, visible_range: Range<usize>, window, cx| {
                rows[visible_range]
                    .iter()
                    .enumerate()
                    .map(|(row_ix, row)| {
                        let alternating_color = cx.theme().colors.bg_alternating;

                        let row_id = row.id(cx);
                        let row_height = this.row_height(window, cx);

                        let cells = D::Column::all()
                            .iter()
                            .map(|col| {
                                div()
                                    .w(this.column_width(col))
                                    .when_some(row_height, |e, h| e.h(h))
                                    .border_b_1()
                                    .border_r_1()
                                    .border_color(cx.theme().colors.border)
                                    .child(row.render_cell(col, window, cx))
                            })
                            .collect::<Vec<_>>();

                        div()
                            .id(row_id)
                            .w_full()
                            .when(row_ix.is_multiple_of(2), |e| e.bg(alternating_color))
                            .hover(|e| e.bg(cx.theme().colors.bg_tertiary.hovered()))
                            .flex()
                            .when(row.selected(cx), |e| e.bg(cx.theme().colors.bg_selected))
                            .on_click({
                                let row = row.clone();
                                cx.listener(move |this, event, w, cx| {
                                    this.delegate.handle_on_click_row(row.clone(), event, w, cx)
                                })
                            })
                            .children(cells)
                    })
                    .collect()
            }),
        )
        .size_full();

        div().h_full().bg(cx.theme().colors.bg_secondary).child(header_row).child(data_rows)
    }
}

pub trait TableDelegate: Sized {
    type Row: TableRow<Self>;

    type Column: TableColumn + std::hash::Hash + Eq;

    fn rows(&mut self, cx: &mut App) -> Vec<Self::Row>;

    fn row_height(&self, _window: &mut Window, _cx: &mut Context<Table<Self>>) -> Option<Pixels>
    where
        Self: Sized,
    {
        None
    }

    fn handle_on_click_row(
        &mut self,
        _row: Self::Row,
        _event: &ClickEvent,
        _window: &mut Window,
        _cx: &mut Context<Table<Self>>,
    ) {
    }
}

pub trait TableRow<D: TableDelegate> {
    fn id(&self, cx: &mut Context<Table<D>>) -> ElementId;

    fn selected(&self, _cx: &mut Context<Table<D>>) -> bool {
        false
    }

    fn render_cell(
        &self,
        column: &D::Column,
        window: &mut Window,
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
