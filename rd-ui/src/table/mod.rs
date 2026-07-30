use gpui::{
    App, ElementId, Entity, Focusable, FontWeight, MouseButton, Pixels, Window, div, prelude::*, px,
};

mod column;
mod delegate;
mod state;

pub use column::*;
pub use delegate::*;
pub use state::*;

use crate::{
    ActiveTheme, Button, ButtonVariant, HslaExt, IconVariant, StatefulInteractiveElementExt,
    h_flex, todo, v_flex,
};

const ROW_HEIGHT: Pixels = px(24.0);

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
        let active_sort = state.sorted_column() == Some(column.id());
        let selected = self.state.read(cx).sorted_column() == Some(column.id());

        let sort_icon = match (active_sort, state.sort_direction()) {
            (true, Some(TableSortDirection::Ascending)) => IconVariant::ArrowDownAZ,
            (true, Some(TableSortDirection::Descending)) => IconVariant::ArrowUpZA,
            _ => IconVariant::ArrowDownUp,
        };

        let bg = cx.theme().bg_secondary;

        h_flex()
            .id(format!("header-cell-{}", column.id()))
            .justify_between()
            .gap_1()
            .w_full()
            .h(ROW_HEIGHT)
            .px_1()
            .when(column_ix != 0, |e| e.border_l_1())
            .border_color(cx.theme().border_secondary)
            .child(div().font_weight(FontWeight::BOLD).child(column.name().to_string()))
            .bg(bg)
            .hover(|e| e.bg(bg.hover()))
            .active(|e| e.bg(bg.active()))
            .when(column.sortable(), |e| {
                e.child(
                    Button::new(format!("{}-sort", column.id()), cx.focus_handle())
                        .icon(sort_icon)
                        .variant(if selected {
                            ButtonVariant::Primary
                        } else {
                            ButtonVariant::Secondary
                        })
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
            .on_mouse_down(MouseButton::Left, {
                let column_id = column.id().to_string();
                let state = self.state.clone();
                move |_, _, cx| {
                    state.update(cx, |state, cx| {
                        state.select_all_in_column(&column_id, cx);
                        cx.notify();
                    });
                }
            })
            .on_mouse_up(MouseButton::Right, {
                let column_id = column.id().to_string();
                let state = self.state.clone();
                move |_, window, cx| {
                    state.update(cx, |state, cx| {
                        state.select_all_in_column(&column_id, cx);
                        cx.notify();
                    });

                    window.dispatch_action(Box::new(crate::action::Edit), cx);
                }
            })
            .on_double_click(|_, window, cx| {
                window.dispatch_action(Box::new(crate::action::Edit), cx);
            })
    }

    fn render_body(&self, window: &Window, cx: &App) -> impl IntoElement {
        let state = self.state().read(cx);
        let rows = state.sorted_rows();

        let cells = rows
            .into_iter()
            .enumerate()
            .map(|(row_ix, (row_id, row))| self.render_row(row, row_id, row_ix, window, cx));

        div().id("body").overflow_scroll().size_full().child(
            div()
                .flex()
                .flex_col()
                .children(cells)
                .on_mouse_up_out(MouseButton::Left, {
                    let state = self.state().clone();
                    move |_, _, cx| {
                        state.update(cx, |state, cx| {
                            if let Some(last_row_ix) = state.delegate().row_count().checked_sub(1) {
                                state.stop_selection_drag(last_row_ix, cx);
                                cx.notify();
                            }
                        });
                    }
                })
                .on_mouse_up_out(MouseButton::Right, {
                    let state = self.state().clone();
                    move |_, _, cx| {
                        state.update(cx, |state, cx| {
                            if let Some(last_row_ix) = state.delegate().row_count().checked_sub(1) {
                                state.stop_selection_drag(last_row_ix, cx);
                                cx.notify();
                            }
                        });
                    }
                }),
        )
    }

    fn render_row(
        &self,
        row: &D::Row,
        row_id: &D::RowId,
        row_ix: usize,
        window: &Window,
        cx: &App,
    ) -> impl IntoElement {
        let columns = self.state().read(cx).delegate().columns();

        let cells = columns.iter().enumerate().map(|(column_ix, column)| {
            self.render_cell(row, row_id, row_ix, column_ix, column, window, cx)
        });

        div()
            .id(format!("row-{}", row_ix))
            .flex()
            .flex_row()
            .h(ROW_HEIGHT)
            .bg(cx.theme().bg_table_odd)
            .border_t_1()
            .border_color(cx.theme().border_secondary)
            .cursor_crosshair()
            .when(row_ix.is_multiple_of(2), |e| e.bg(cx.theme().bg_table))
            .children(cells)
            .on_double_click(|_, window, cx| {
                window.dispatch_action(Box::new(crate::action::Edit), cx);
            })
    }

    fn render_cell(
        &self,
        row: &D::Row,
        row_id: &D::RowId,
        row_ix: usize,
        column_ix: usize,
        column: &Column<D>,
        window: &Window,
        cx: &App,
    ) -> impl IntoElement {
        let content = match &column.cell_builder {
            Some(cell_builder) => div()
                .absolute()
                .inset_0()
                .flex()
                .justify_between()
                .gap_1()
                .px_1()
                .child((cell_builder)(row, window, cx))
                .into_any_element(),
            None => todo(cx).into_any_element(),
        };

        let is_selected =
            self.state.read(cx).selection().read(cx).is_cell_selected(column.id(), row_id);
        let is_editable = column.editable();

        let selection_overlay = is_selected.then(|| {
            div()
                .absolute()
                .inset_0()
                .bottom_px()
                .border_1()
                .border_color(cx.theme().border_selected)
        });

        div()
            .relative()
            .w_full()
            .h(ROW_HEIGHT)
            .when(column_ix != 0, |e| e.border_l_1())
            .border_color(cx.theme().border_secondary)
            .bg(if is_selected { cx.theme().bg_selected } else { gpui::transparent_black() })
            .child(content)
            .children(selection_overlay)
            .when(!is_editable, |e| e.opacity(0.75))
            .on_mouse_down(MouseButton::Left, {
                let state = self.state().clone();
                let column_id = column.id().to_string();
                move |_, _window, cx| {
                    state.update(cx, |state, cx| {
                        if !is_selected {
                            state.start_selection_drag(column_id.clone(), row_ix, cx);
                            cx.notify();
                        }
                    });
                }
            })
            .on_mouse_down(MouseButton::Right, {
                let state = self.state().clone();
                let column_id = column.id().to_string();
                move |_, _window, cx| {
                    state.update(cx, |state, cx| {
                        if !is_selected {
                            state.start_selection_drag(column_id.clone(), row_ix, cx);
                            cx.notify();
                        }
                    });
                }
            })
            .on_mouse_move({
                let state = self.state().clone();
                move |_, _, cx| {
                    if state.read(cx).is_dragging_selection() {
                        state.update(cx, |state, cx| {
                            let Some((_, first_row_ix)) = &state.selection_drag else {
                                return;
                            };

                            if *first_row_ix == row_ix {
                                return;
                            }

                            state.update_selection_drag(row_ix, cx);

                            cx.notify();
                        });
                    }
                }
            })
            .on_mouse_up(MouseButton::Left, {
                let state = self.state().clone();
                move |_, _, cx| {
                    state.update(cx, |state, cx| {
                        state.stop_selection_drag(row_ix, cx);
                        cx.notify();
                    });
                }
            })
            .on_mouse_up(MouseButton::Right, {
                let state = self.state().clone();
                move |_, window, cx| {
                    state.update(cx, |state, cx| {
                        state.stop_selection_drag(row_ix, cx);
                        cx.notify();
                    });

                    window.dispatch_action(Box::new(crate::action::Edit), cx);
                }
            })
    }

    fn render_action_bar(&self, _window: &Window, cx: &App) -> impl IntoElement {
        h_flex()
            .justify_between()
            .h(px(32.0))
            .bg(cx.theme().bg_secondary)
            .border_t_1()
            .border_color(cx.theme().border_secondary)
            .px_2()
            .child(div().text_color(cx.theme().fg_secondary).child(format!(
                "{}/{} row{} selected",
                self.state.read(cx).selection().read(cx).count(),
                self.state.read(cx).delegate().row_count(),
                if self.state.read(cx).delegate().row_count() == 1 { "" } else { "s" }
            )))
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(
                        Button::new("edit-selection", cx.focus_handle())
                            .label("Edit")
                            .variant(ButtonVariant::Secondary)
                            .icon(IconVariant::SquarePen)
                            .disabled(!self.state.read(cx).can_edit(cx))
                            .action_for(crate::action::Edit, self.state.focus_handle(cx)),
                    )
                    .child(
                        Button::new("delete-selection", cx.focus_handle())
                            .label("Delete")
                            .disabled(self.state.read(cx).selection().read(cx).is_empty())
                            .variant(ButtonVariant::Danger)
                            .icon(IconVariant::Trash2)
                            .action_for(crate::action::Delete, self.state.focus_handle(cx)),
                    ),
            )
    }
}

impl<D: TableDelegate + 'static> RenderOnce for Table<D> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let focus_handle = self.state.focus_handle(cx);

        v_flex()
            .id(self.id.clone())
            .track_focus(&focus_handle)
            .size_full()
            .child(self.render_header(window, cx))
            .child(self.render_body(window, cx))
            .child(self.render_action_bar(window, cx))
            .on_action::<crate::action::Edit>({
                let state = self.state.clone();
                move |_, window, cx| {
                    let selection = state.read(cx).selection().read(cx);
                    let Some(column_id) = &selection.column_id else { return };
                    let Some(column) = state.read(cx).delegate().column(column_id) else { return };
                    let row_ids = selection.row_ids().cloned().collect();
                    if let Some(edit_handler) = column.edit_handler.clone() {
                        (edit_handler)(state.clone(), row_ids, window, cx);
                    }
                }
            })
            .on_action::<crate::action::Delete>({
                let state = self.state.clone();
                move |_, _, cx| {
                    state.update(cx, |state, cx| {
                        let selection = state.selection().read(cx);
                        state.delegate_mut().delete_rows(selection.row_ids());
                        state.selection().update(cx, |selection, cx| {
                            selection.clear();
                            cx.notify();
                        });
                    });
                }
            })
            .on_action::<crate::action::ClearSelection>({
                let state = self.state.clone();
                move |_, _, cx| {
                    state.update(cx, |state, cx| {
                        state.selection().update(cx, |selection, cx| {
                            selection.clear();
                            cx.notify();
                        });
                    });
                }
            })
            .on_action::<crate::action::SelectAll>({
                let state = self.state.clone();
                move |_, _, cx| {
                    state.update(cx, |state, cx| {
                        state.selection().update(cx, |selection, cx| {
                            let Some(column_id) = selection.column_id.clone().or_else(|| {
                                state.delegate().columns().first().map(|c| c.id().to_string())
                            }) else {
                                return;
                            };

                            for row_id in state.delegate().row_ids() {
                                selection.select_cell(column_id.clone(), row_id.clone());
                            }

                            cx.notify();
                        });
                    });
                }
            })
    }
}
