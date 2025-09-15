use std::ops::{Deref, DerefMut, Range};

use gpui::prelude::*;
use gpui::{
    App, Context, EventEmitter, FocusHandle, FontWeight, IntoElement, MouseButton, Pixels,
    SharedString, Window, div, uniform_list,
};

pub use column::*;
pub use delegate::*;
pub use event::*;

use crate::theme::{ActiveTheme, InteractiveColor};
use crate::utils::z_stack;

mod column;
mod delegate;
mod event;

pub mod actions {
    use gpui::{App, KeyBinding};

    gpui::actions!(
        table,
        [
            ClearSelection,
            EditSelection,
            NextColumn,
            PrevColumn,
            NextRow,
            PrevRow,
            ExtendSelectionNext,
            ExtendSelectionPrev
        ]
    );

    pub const KEY_CONTEXT: &str = "Table";

    pub(crate) fn init(cx: &mut App) {
        cx.bind_keys([
            KeyBinding::new("escape", ClearSelection, Some(KEY_CONTEXT)),
            KeyBinding::new("enter", EditSelection, Some(KEY_CONTEXT)),
            KeyBinding::new("right", NextColumn, Some(KEY_CONTEXT)),
            KeyBinding::new("left", PrevColumn, Some(KEY_CONTEXT)),
            KeyBinding::new("down", NextRow, Some(KEY_CONTEXT)),
            KeyBinding::new("up", PrevRow, Some(KEY_CONTEXT)),
            KeyBinding::new("shift-down", ExtendSelectionNext, Some(KEY_CONTEXT)),
            KeyBinding::new("shift-up", ExtendSelectionPrev, Some(KEY_CONTEXT)),
        ]);
    }
}

#[derive(Debug, Clone)]
pub struct Selection {
    pub column_id: SharedString,
    pub start_ix: usize,
    pub end_ix: usize,
    pub inverted: bool,
}

impl Selection {
    pub fn contains(&self, row_ix: usize) -> bool {
        let (low, high) = if self.start_ix <= self.end_ix {
            (self.start_ix, self.end_ix)
        } else {
            (self.end_ix, self.start_ix)
        };
        row_ix >= low && row_ix <= high
    }

    pub fn size(&self) -> usize {
        let (low, high) = if self.start_ix <= self.end_ix {
            (self.start_ix, self.end_ix)
        } else {
            (self.end_ix, self.start_ix)
        };
        high - low + 1
    }
}

pub struct Table<D: TableDelegate> {
    delegate: D,

    focus_handle: FocusHandle,

    selection: Option<Selection>,
    is_selecting: bool,

    row_height: Pixels,
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

    pub fn refresh(&mut self, window: &mut Window, cx: &mut Context<Self>)
    where
        D: Clone,
    {
        *self = Self::new(self.delegate.clone(), window, cx);
        cx.notify();
    }

    pub fn delegate(&self) -> &D {
        &self.delegate
    }

    pub fn delegate_mut(&mut self) -> &mut D {
        &mut self.delegate
    }

    pub fn start_selection(
        &mut self,
        column_id: impl Into<SharedString>,
        row_ix: usize,
        cx: &mut Context<Self>,
    ) {
        self.selection = Some(Selection {
            column_id: column_id.into(),
            start_ix: row_ix,
            end_ix: row_ix,
            inverted: false,
        });
        cx.emit(TableEvent::SelectionChanged);
        cx.notify();
    }

    pub fn end_selection(&mut self, row_ix: usize, cx: &mut Context<Self>) {
        let can_select_multiple = self.can_select_multiple_rows(cx);

        if let Some(selection) = &mut self.selection {
            if !can_select_multiple {
                selection.start_ix = row_ix;
            }

            selection.end_ix = row_ix;
            selection.inverted = row_ix <= selection.start_ix;
        }
        cx.emit(TableEvent::SelectionChanged);
        cx.notify();
    }

    pub fn select_column(&mut self, column_id: impl Into<SharedString>, cx: &mut Context<Self>) {
        let row_count = self.sorted_row_ids(cx).len();
        if row_count != 0 {
            self.start_selection(column_id, 0, cx);

            if self.can_select_multiple_rows(cx) {
                self.end_selection(row_count - 1, cx);
            }
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

    pub fn edit_selection(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(column_id) = self.selection.as_ref().map(|s| s.column_id.clone()) else {
            return;
        };

        let selected_row_ids = self.selected_row_ids(cx);
        self.delegate_mut().edit_selection(&column_id, selected_row_ids, window, cx)
    }

    pub fn selected_row_ids(&self, cx: &App) -> Vec<D::RowId> {
        let Some(selection) = self.selection.as_ref() else { return Vec::new() };

        let row_ixs: Vec<usize> = if selection.inverted {
            (selection.end_ix..=selection.start_ix).rev().collect()
        } else {
            (selection.start_ix..=selection.end_ix).collect()
        };

        let mut ids = Vec::new();
        for row_ix in row_ixs {
            ids.push(self.row_id(row_ix, cx).unwrap());
        }
        ids
    }

    fn row_id(&self, row_ix: usize, cx: &App) -> Option<D::RowId> {
        self.sorted_row_ids(cx).get(row_ix).cloned()
    }

    fn render_header(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let cells = (0..self.column_count(cx)).into_iter().map(|col_ix| {
            let column = self.column(col_ix, cx);

            div()
                .flex()
                .items_center()
                .px_1()
                .min_w(column.width)
                .max_w(column.width)
                .h_full()
                .bg(cx.theme().table_header)
                .border_r_1()
                .border_b_1()
                .border_color(cx.theme().table_header_border)
                .hover(|e| e.bg(cx.theme().table_header.hovered()))
                .when(col_ix == self.column_count(cx) - 1, |e| e.border_r_0())
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener({
                        let column_id = column.id.to_string();
                        move |this, _, _, cx| {
                            this.select_column(&column_id, cx);
                        }
                    }),
                )
                .on_mouse_down(
                    MouseButton::Right,
                    cx.listener({
                        let column_id = column.id.to_string();
                        move |this, _, window, cx| {
                            this.select_column(&column_id, cx);
                            this.edit_selection(window, cx);
                        }
                    }),
                )
                .child(column.label.clone())
        });

        div()
            .min_h(self.row_height)
            .max_h(self.row_height)
            .flex()
            .text_color(cx.theme().table_header_foreground)
            .font_weight(FontWeight::BOLD)
            .children(cells)
    }

    fn render_body(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let row_count = self.sorted_row_ids(cx).len();
        uniform_list(
            "table_list",
            row_count,
            cx.processor(|this, range: Range<usize>, window, cx| {
                range
                    .map(|row_ix| {
                        let row_id = this.row_id(row_ix, cx).unwrap();
                        this.render_row(&row_id, row_ix, window, cx).into_any_element()
                    })
                    .collect()
            }),
        )
        .size_full()
        .bg(cx.theme().table)
    }

    fn render_row(
        &mut self,
        row_id: &D::RowId,
        row_ix: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let selected = self.selection_contains(row_ix);

        let bg_color = if selected {
            cx.theme().selected
        } else {
            if row_ix % 2 == 1 { cx.theme().table_even } else { cx.theme().table }
        };

        div()
            .flex()
            .min_h(self.row_height)
            .max_h(self.row_height)
            .bg(bg_color)
            .hover(|e| e.bg(bg_color.hovered()))
            .border_color(cx.theme().table_row_border)
            .border_b_1()
            .cursor_crosshair()
            .children((0..self.column_count(cx)).into_iter().map(|col_ix| {
                self.render_cell(row_id, row_ix, col_ix, window, cx).into_any_element()
            }))
    }

    fn render_cell(
        &mut self,
        row_id: &D::RowId,
        row_ix: usize,
        col_ix: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let column = self.column(col_ix, cx);

        let content = div()
            .size_full()
            .border_r_1()
            .when(col_ix == self.column_count(cx) - 1, |e| e.border_r_0())
            .border_color(cx.theme().table_row_border)
            .overflow_hidden()
            .child(self.delegate().render_cell(row_id, col_ix, window, cx));

        let row_is_selected = self.selection_contains(row_ix);
        let column_is_selected = self.selected_column() == Some(&column.id);
        let selection_highlight = if row_is_selected && column_is_selected {
            let prev_row_ix = row_ix.overflowing_sub(1).0;
            let next_row_ix = row_ix + 1;
            div()
                .border_x_1()
                .when(!self.selection_contains(prev_row_ix), |e| e.border_t_1())
                .when(!self.selection_contains(next_row_ix), |e| e.border_b_1())
                .border_color(cx.theme().selected_border)
                .on_mouse_down(
                    MouseButton::Right,
                    cx.listener(|this, _, window, cx| this.edit_selection(window, cx)),
                )
        } else {
            div()
        }
        .size_full();

        z_stack([content.into_any_element(), selection_highlight.into_any_element()])
            .min_w(column.width)
            .max_w(column.width)
            .h_full()
            .when(column_is_selected && row_is_selected, |e| e.bg(cx.theme().selected))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener({
                    let column_id = column.id.clone();
                    move |this, _, event, cx| {
                        if event.modifiers().shift && this.is_selecting == false {
                            this.is_selecting = true;
                            this.end_selection(row_ix, cx);
                        } else {
                            this.is_selecting = true;
                            this.clear_selection(cx);
                            this.start_selection(column_id.clone(), row_ix, cx);
                        }
                    }
                }),
            )
            .on_mouse_down(
                MouseButton::Right,
                cx.listener({
                    let column_id = column.id.clone();
                    move |this, _, window, cx| {
                        if this.selection.as_ref().is_some_and(|s| s.size() <= 1) {
                            this.start_selection(column_id.clone(), row_ix, cx);
                            this.edit_selection(window, cx);
                        }
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

    fn move_selection_column(&mut self, offset: isize, cx: &mut Context<Self>) {
        let col_count = self.column_count(cx);
        let last_col_ix = col_count - 1;
        if let Some(selection) = self.selection.as_ref() {
            let current_col_ix = self.column_ix(&selection.column_id, cx) as isize;
            let target_ix = (current_col_ix + offset).clamp(0, last_col_ix as isize) as usize;
            let id = self.column(target_ix, cx).id.clone();
            if let Some(selection) = &mut self.selection {
                selection.column_id = id;
            }
        } else if offset < 0 {
            self.select_column(self.column(last_col_ix, cx).id.clone(), cx);
        } else {
            self.select_column(self.column(0, cx).id.clone(), cx);
        };

        cx.notify();
    }

    fn move_selection_row(&mut self, mut offset: isize, cx: &mut Context<Self>) {
        let row_count = self.sorted_row_ids(cx).len();
        let last_row_ix = row_count - 1;
        let (column_id, target_ix) = if let Some(selection) = self.selection.as_ref() {
            if selection.size() > 1 {
                offset = 0
            };

            (selection.column_id.clone(), selection.end_ix as isize + offset)
        } else if offset < 0 {
            (self.column(0, cx).id.clone(), last_row_ix as isize)
        } else {
            (self.column(0, cx).id.clone(), 0)
        };
        let target_ix = target_ix.clamp(0, last_row_ix as isize) as usize;
        self.start_selection(column_id, target_ix, cx);
        cx.notify();
    }

    fn extend_selection(&mut self, offset: isize, cx: &mut Context<Self>) {
        if let Some(selection) = &self.selection {
            let new_end = (selection.end_ix as isize + offset).max(0) as usize;
            self.end_selection(new_end, cx);
        } else if offset > 0 {
            cx.dispatch_action(&actions::NextRow);
        } else {
            cx.dispatch_action(&actions::PrevRow);
        }
        cx.notify();
    }

    fn handle_clear_selection(
        &mut self,
        _: &actions::ClearSelection,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.clear_selection(cx);
        cx.notify();
    }

    fn handle_edit_selection(
        &mut self,
        _: &actions::EditSelection,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.edit_selection(window, cx);
    }

    fn handle_next_column(
        &mut self,
        _: &actions::NextColumn,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.move_selection_column(1, cx);
    }

    fn handle_prev_column(
        &mut self,
        _: &actions::PrevColumn,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.move_selection_column(-1, cx);
    }

    fn handle_next_row(&mut self, _: &actions::NextRow, _: &mut Window, cx: &mut Context<Self>) {
        self.move_selection_row(1, cx);
    }

    fn handle_prev_row(&mut self, _: &actions::PrevRow, _: &mut Window, cx: &mut Context<Self>) {
        self.move_selection_row(-1, cx);
    }

    fn handle_extend_selection_next(
        &mut self,
        _: &actions::ExtendSelectionNext,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.extend_selection(1, cx);
    }

    fn handle_extend_selection_prev(
        &mut self,
        _: &actions::ExtendSelectionPrev,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.extend_selection(-1, cx);
    }
}

impl<D: TableDelegate> Deref for Table<D> {
    type Target = D;

    fn deref(&self) -> &Self::Target {
        &self.delegate
    }
}

impl<D: TableDelegate> DerefMut for Table<D> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.delegate
    }
}

impl<D: TableDelegate + 'static> Render for Table<D> {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .key_context(actions::KEY_CONTEXT)
            .track_focus(&self.focus_handle)
            .id("table")
            .flex()
            .flex_col()
            .size_full()
            .overflow_y_hidden()
            .overflow_x_scroll()
            .on_action(cx.listener(Self::handle_clear_selection))
            .on_action(cx.listener(Self::handle_edit_selection))
            .on_action(cx.listener(Self::handle_prev_column))
            .on_action(cx.listener(Self::handle_next_column))
            .on_action(cx.listener(Self::handle_next_row))
            .on_action(cx.listener(Self::handle_prev_row))
            .on_action(cx.listener(Self::handle_extend_selection_next))
            .on_action(cx.listener(Self::handle_extend_selection_prev))
            .child(self.render_header(window, cx))
            .child(self.render_body(window, cx))
    }
}

impl<D: TableDelegate + 'static> EventEmitter<TableEvent> for Table<D> {}
