mod column;
mod delegate;

use std::ops::Range;

use gpui::prelude::*;
use gpui::{
    App, Context, FocusHandle, IntoElement, KeyBinding, MouseButton, Pixels, Window, div,
    uniform_list,
};

pub use column::*;
pub use delegate::*;

use crate::theme::ActiveTheme;
use crate::utils::z_stack;

pub mod actions {
    gpui::actions!(table, [ClearSelection, EditSelection]);
}

pub const TABLE_KEY_CONTEXT: &str = "Table";

pub(super) fn init(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("escape", actions::ClearSelection, Some(TABLE_KEY_CONTEXT)),
        KeyBinding::new("enter", actions::EditSelection, Some(TABLE_KEY_CONTEXT)),
    ]);
}

#[derive(Debug, Clone)]
pub struct Selection {
    pub column_id: String,
    pub start: usize,
    pub end: usize,
    pub inverted: bool,
}

impl Selection {
    pub fn contains(&self, row_ix: usize) -> bool {
        if self.inverted {
            row_ix >= self.end && row_ix <= self.start
        } else {
            row_ix >= self.start && row_ix <= self.end
        }
    }
}

pub struct Table<D: TableDelegate> {
    delegate: D,
    focus_handle: FocusHandle,
    selection: Option<Selection>,
    row_height: Pixels,

    is_selecting: bool,
}

impl<D: TableDelegate> Table<D> {
    pub fn new(delegate: D, window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            delegate,
            focus_handle: cx.focus_handle(),
            selection: None,
            row_height: window.line_height(),

            is_selecting: false,
        }
    }
}

impl<D: TableDelegate> Table<D> {
    pub fn delegate(&self) -> &D {
        &self.delegate
    }

    pub fn delegate_mut(&mut self) -> &mut D {
        &mut self.delegate
    }

    pub fn start_selection(
        &mut self,
        column_id: impl Into<String>,
        row_ix: usize,
        cx: &mut Context<Self>,
    ) {
        self.selection = Some(Selection {
            column_id: column_id.into(),
            start: row_ix,
            end: row_ix,
            inverted: false,
        });
        cx.notify();
    }

    pub fn end_selection(&mut self, row_ix: usize, cx: &mut Context<Self>) {
        if let Some(selection) = &mut self.selection {
            selection.end = row_ix;
            selection.inverted = row_ix <= selection.start;
        }
        cx.notify();
    }

    pub fn select_column(&mut self, column_id: impl Into<String>, cx: &mut Context<Self>) {
        let row_count = self.delegate().row_count(cx);
        if row_count != 0 {
            self.start_selection(column_id, 0, cx);
            self.end_selection(row_count - 1, cx);
        }
    }

    pub fn selection_contains(&self, row_ix: usize) -> bool {
        self.selection.as_ref().is_some_and(|selection| selection.contains(row_ix))
    }

    pub fn selected_column(&self) -> Option<&str> {
        self.selection.as_ref().map(|selection| selection.column_id.as_str())
    }

    pub fn clear_selection(&mut self, cx: &mut Context<Self>) {
        self.selection = None;
        cx.notify();
    }

    pub fn edit_selection(&mut self, cx: &mut App) {
        if let Some(selection) = self.selection.clone() {
            self.delegate_mut().edit_selection(selection, cx)
        }
    }

    fn render_header(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let cells = (0..self.delegate().column_count(cx)).into_iter().map(|col_ix| {
            let column = self.delegate().column(col_ix, cx);

            div()
                .flex()
                .items_center()
                .px_1()
                .w(column.width)
                .h(self.row_height)
                .bg(cx.theme().colors.bg_tertiary)
                .border_b_1()
                .border_r_1()
                .when(col_ix == self.delegate().column_count(cx) - 1, |e| e.border_r_0())
                .border_color(cx.theme().colors.border)
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener({
                        let column_id = column.id.to_string();
                        move |this, _, _, cx| {
                            this.select_column(&column_id, cx);
                        }
                    }),
                )
                .child(column.label.clone())
        });

        div().flex().w_full().children(cells)
    }

    fn render_body(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        uniform_list(
            "table_list",
            self.delegate().row_count(cx),
            cx.processor(|table, range: Range<usize>, window, cx| {
                range
                    .map(|row_ix| table.render_row(row_ix, window, cx).into_any_element())
                    .collect()
            }),
        )
        .size_full()
    }

    fn render_row(
        &mut self,
        row_ix: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let cells = (0..self.delegate().column_count(cx))
            .into_iter()
            .map(|col_ix| self.render_cell(row_ix, col_ix, window, cx).into_any_element());

        div().flex().w_full().children(cells)
    }

    fn render_cell(
        &mut self,
        row_ix: usize,
        col_ix: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let column = self.delegate().column(col_ix, cx);

        let content = div()
            .size_full()
            .border_b_1()
            .border_r_1()
            .when(col_ix == self.delegate().column_count(cx) - 1, |e| e.border_r_1())
            .border_color(cx.theme().colors.border)
            .overflow_hidden()
            .child(self.delegate().render_cell(row_ix, col_ix, window, cx));

        let row_is_selected = self.selection_contains(row_ix);
        let column_is_selected = self.selected_column() == Some(&column.id);
        let selection_highlight = if row_is_selected && column_is_selected {
            let prev_row_ix = row_ix.overflowing_sub(1).0;
            let next_row_ix = row_ix + 1;
            div()
                .border_x_2()
                .when(!self.selection_contains(prev_row_ix), |e| e.border_t_2())
                .when(!self.selection_contains(next_row_ix), |e| e.border_b_2())
                .border_color(cx.theme().colors.border_selected)
                .on_mouse_down(
                    MouseButton::Right,
                    cx.listener(|this, _, _, cx| this.edit_selection(cx)),
                )
        } else {
            div()
        }
        .size_full();

        z_stack([content.into_any_element(), selection_highlight.into_any_element()])
            .w(column.width)
            .h(self.row_height)
            .bg(if column_is_selected && row_is_selected {
                cx.theme().colors.bg_selected
            } else if row_ix % 2 == 0 {
                cx.theme().colors.bg_secondary
            } else {
                cx.theme().colors.bg_alternating
            })
            .on_mouse_down(
                MouseButton::Left,
                cx.listener({
                    let column_id = column.id.clone();
                    move |this, _, _, cx| {
                        this.is_selecting = true;
                        this.clear_selection(cx);
                        this.start_selection(column_id.clone(), row_ix, cx);
                    }
                }),
            )
            .on_mouse_move(cx.listener(move |this, _, _, cx| {
                if this.is_selecting {
                    this.end_selection(row_ix, cx);
                }
            }))
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(move |this, _, _, cx| {
                    this.end_selection(row_ix, cx);
                }),
            )
            .on_mouse_up_out(
                MouseButton::Left,
                cx.listener(move |this, _, _, _| {
                    this.is_selecting = false;
                }),
            )
    }
}

impl<D: TableDelegate + 'static> Render for Table<D> {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .key_context(TABLE_KEY_CONTEXT)
            .track_focus(&self.focus_handle)
            .id("table")
            .flex()
            .flex_col()
            .size_full()
            .on_action::<actions::ClearSelection>(cx.listener(|this, _, _, cx| {
                this.clear_selection(cx);
                cx.notify();
            }))
            .on_action::<actions::EditSelection>(
                cx.listener(|this, _, _, cx| this.edit_selection(cx)),
            )
            .child(self.render_header(window, cx))
            .child(self.render_body(window, cx))
    }
}
