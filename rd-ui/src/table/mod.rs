use gpui::{App, ElementId, Entity, FontWeight, MouseButton, Pixels, Window, div, prelude::*, px};

mod column;
mod delegate;
mod state;

pub use column::*;
pub use delegate::*;
pub use state::*;

use crate::{ActiveTheme, Button, Icon, IconSize, IconVariant, h_flex, todo, v_flex};

const ROW_HEIGHT: Pixels = px(24.0);
const EDIT_MOUSE_BUTTON: MouseButton = MouseButton::Right;

#[derive(IntoElement)]
pub struct Table<D: TableDelegate + 'static> {
    id: ElementId,

    state: Entity<TableState<D>>,
}

impl<D: TableDelegate> Table<D> {
    pub fn new(
        id: impl Into<ElementId>,
        state: Entity<TableState<D>>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self {
        Self { id: id.into(), state }
    }

    pub fn state(&self) -> &Entity<TableState<D>> {
        &self.state
    }

    fn render_header(&self, window: &Window, cx: &App) -> impl IntoElement {
        let columns = self.state().read(cx).delegate().columns();

        let cells = columns
            .iter()
            .enumerate()
            .map(|(ix, column)| self.render_header_cell(column, ix, window, cx));

        h_flex()
            .bg(cx.theme().bg_secondary)
            .border_b_1()
            .border_color(cx.theme().border_secondary)
            .children(cells)
    }

    fn render_header_cell(
        &self,
        column: &Column<D>,
        column_ix: usize,
        _window: &Window,
        cx: &App,
    ) -> impl IntoElement {
        let state = self.state.read(cx);
        let is_active_sort = state.sorted_column() == Some(column.id());

        let sort_icon = match (is_active_sort, state.sort_direction()) {
            (true, Some(TableSortDirection::Ascending)) => IconVariant::ArrowDownAZ,
            (true, Some(TableSortDirection::Descending)) => IconVariant::ArrowUpZA,
            _ => IconVariant::ArrowDownUp,
        };

        h_flex()
            .justify_between()
            .gap_1()
            .w_full()
            .h(ROW_HEIGHT)
            .px_1()
            .border_b_1()
            .when(column_ix != 0, |e| e.border_l_1())
            .border_color(cx.theme().border_secondary)
            .child(div().font_weight(FontWeight::BOLD).child(column.name().to_string()))
            .when(column.sortable(), |e| {
                e.child(
                    Button::new(format!("{}-sort", column.id()))
                        .icon(Icon::new(sort_icon, IconSize::ExtraSmall))
                        .selected(self.state.read(cx).sorted_column() == Some(column.id()))
                        .on_click({
                            let column_id = column.id().to_string();
                            let state = self.state.clone();
                            move |_, _, cx| {
                                state.update(cx, |state, cx| {
                                    match state.sort_direction() {
                                        Some(TableSortDirection::Descending) | None => state
                                            .sort_by_column(
                                                &column_id,
                                                TableSortDirection::Ascending,
                                            ),
                                        Some(TableSortDirection::Ascending) => state
                                            .sort_by_column(
                                                &column_id,
                                                TableSortDirection::Descending,
                                            ),
                                    }

                                    cx.notify();
                                })
                            }
                        }),
                )
            })
            .on_mouse_down(EDIT_MOUSE_BUTTON, {
                let state = self.state.clone();
                let edit_handler = column.edit_handler.clone();
                move |_, window, cx| {
                    if let Some(edit_handler) = &edit_handler {
                        let row_ids = state
                            .read(cx)
                            .sorted_rows()
                            .iter()
                            .map(|(id, _)| (*id).clone())
                            .collect();
                        (edit_handler)(state.clone(), row_ids, window, cx);
                    }
                }
            })
    }

    fn render_body(&self, window: &Window, cx: &App) -> impl IntoElement {
        let state = self.state().read(cx);
        let rows = state.sorted_rows();

        let cells =
            rows.into_iter().enumerate().map(|(ix, (_, row))| self.render_row(ix, row, window, cx));

        div().flex().flex_col().children(cells)
    }

    fn render_row(
        &self,
        row_ix: usize,
        row: &D::Row,
        window: &Window,
        cx: &App,
    ) -> impl IntoElement {
        let columns = self.state().read(cx).delegate().columns();

        let cells = columns
            .iter()
            .enumerate()
            .map(|(ix, column)| self.render_cell(row, ix, column, window, cx));

        div()
            .flex()
            .flex_row()
            .h(ROW_HEIGHT)
            .bg(cx.theme().bg_table_odd)
            .when(row_ix.is_multiple_of(2), |e| e.bg(cx.theme().bg_table))
            .when(row_ix != 0, |e| e.border_t_1())
            .border_color(cx.theme().border_secondary)
            .children(cells)
    }

    fn render_cell(
        &self,
        row: &D::Row,
        column_ix: usize,
        column: &Column<D>,
        window: &Window,
        cx: &App,
    ) -> impl IntoElement {
        let content = match &column.cell_builder {
            Some(cell_builder) => (cell_builder)(row, window, cx),
            None => todo(cx).into_any_element(),
        };

        div()
            .flex()
            .justify_between()
            .gap_1()
            .w_full()
            .h(ROW_HEIGHT)
            .px_1()
            .when(column_ix != 0, |e| e.border_l_1())
            .border_color(cx.theme().border_secondary)
            .child(content)
    }
}

impl<D: TableDelegate + 'static> RenderOnce for Table<D> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        v_flex().id(self.id.clone()).size_full().child(self.render_header(window, cx)).child(
            div().id("body").overflow_scroll().size_full().child(self.render_body(window, cx)),
        )
    }
}
