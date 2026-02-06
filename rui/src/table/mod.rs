use std::ops::Range;

use gpui::{
    AnyElement, App, Div, ElementId, Entity, FontWeight, ListSizingBehavior, MouseButton,
    MouseDownEvent, Pixels, Window, div, prelude::*, uniform_list,
};

mod column;
mod delegate;
mod state;

pub use column::*;
pub use delegate::*;
pub use state::*;

use crate::{ActiveTheme, ElementExt, Icon, IconSize, IconVariant, h_flex, theme::HslaExt};

pub(crate) mod action {
    use gpui::{App, KeyBinding, actions};

    actions!(
        root,
        [
            ClearSelection,
            EditSelection,
            DeleteSelection,
            NextColumn,
            PrevColumn,
            NextRow,
            PrevRow,
            ExtendSelectionNext,
            ExtendSelectionPrev,
            SelectAll,
        ]
    );

    pub const KEY_CONTEXT: &str = "Table";

    pub fn init(cx: &mut App) {
        cx.bind_keys([
            KeyBinding::new("escape", ClearSelection, Some(KEY_CONTEXT)),
            KeyBinding::new("enter", EditSelection, Some(KEY_CONTEXT)),
            KeyBinding::new("delete", DeleteSelection, Some(KEY_CONTEXT)),
            KeyBinding::new("backspace", DeleteSelection, Some(KEY_CONTEXT)),
            KeyBinding::new("right", NextColumn, Some(KEY_CONTEXT)),
            KeyBinding::new("left", PrevColumn, Some(KEY_CONTEXT)),
            KeyBinding::new("down", NextRow, Some(KEY_CONTEXT)),
            KeyBinding::new("up", PrevRow, Some(KEY_CONTEXT)),
            KeyBinding::new("secondary-down", ExtendSelectionNext, Some(KEY_CONTEXT)),
            KeyBinding::new("secondary-up", ExtendSelectionPrev, Some(KEY_CONTEXT)),
            KeyBinding::new("secondary-a", SelectAll, Some(KEY_CONTEXT)),
        ]);
    }
}

#[derive(IntoElement)]
pub struct Table<D: TableDelegate + 'static> {
    state: Entity<TableState<D>>,
}

impl<D: TableDelegate + 'static> Table<D> {
    pub fn new(state: Entity<TableState<D>>) -> Self {
        Self { state }
    }

    pub fn handle_clear_selection(
        state: &Entity<TableState<D>>,
        _window: &mut Window,
        cx: &mut App,
    ) {
        state.update(cx, |state, cx| {
            state.clear_selection(cx);
            cx.notify();
        });
    }

    pub fn handle_edit_selection(state: &Entity<TableState<D>>, window: &mut Window, cx: &mut App) {
        state.update(cx, |state, cx| {
            state.edit_selection(window, cx);
            cx.notify();
        });
    }

    pub fn handle_delete_selection(
        state: &Entity<TableState<D>>,
        _window: &mut Window,
        cx: &mut App,
    ) {
        state.update(cx, |state, cx| {
            state.delete_selection(cx);
            cx.notify();
        });
    }

    pub fn handle_next_column(state: &Entity<TableState<D>>, _window: &mut Window, cx: &mut App) {
        state.update(cx, |state, cx| {
            let total_columns = state.delegate().column_count(cx);
            let ix = (state.selection.selected_column_ix + 1).min(total_columns.saturating_sub(1));
            state.selection.select_column(ix);
            cx.notify();
        });
    }

    pub fn handle_prev_column(state: &Entity<TableState<D>>, _window: &mut Window, cx: &mut App) {
        state.update(cx, |state, cx| {
            let ix = state.selection.selected_column_ix.saturating_sub(1);
            state.selection.select_column(ix);
            cx.notify();
        });
    }

    pub fn handle_next_row(state: &Entity<TableState<D>>, _window: &mut Window, cx: &mut App) {
        state.update(cx, |state, cx| {
            let total = state.rows.visible_rows().len();
            let new_ix = match state.selection.range() {
                Some((_, end)) => (end + 1).min(total.saturating_sub(1)),
                None => 0,
            };
            if total > 0 {
                let col = state.selection.selected_column_ix;
                state.selection.clear();
                state.selection.select_cell(col, new_ix);
            }
            cx.notify();
        });
    }

    pub fn handle_prev_row(state: &Entity<TableState<D>>, _window: &mut Window, cx: &mut App) {
        state.update(cx, |state, cx| {
            let new_ix = match state.selection.range() {
                Some((start, _)) if start > 0 => start - 1,
                Some((0, _)) | None => 0,
                _ => 0,
            };
            let col = state.selection.selected_column_ix;
            state.selection.clear();
            state.selection.select_cell(col, new_ix);
            cx.notify();
        });
    }

    pub fn handle_extend_selection_next(
        state: &Entity<TableState<D>>,
        _window: &mut Window,
        cx: &mut App,
    ) {
        state.update(cx, |state, cx| {
            let total = state.rows.visible_rows().len();
            if total == 0 {
                return;
            }
            cx.notify();
            let Some(old_head) = state.selection.current_head_or_last() else {
                state.selection.extend_to(0);
                return;
            };
            if state.selection.anchor.is_none() {
                state.selection.anchor = Some(old_head);
            }
            let new_head = (old_head + 1).min(total - 1);
            state.selection.extend_to(new_head);
        });
    }

    pub fn handle_extend_selection_prev(
        state: &Entity<TableState<D>>,
        _window: &mut Window,
        cx: &mut App,
    ) {
        state.update(cx, |state, cx| {
            let total = state.rows.visible_rows().len();
            if total == 0 {
                return;
            }
            cx.notify();
            let Some(old_head) = state.selection.current_head_or_last() else {
                state.selection.extend_to(total - 1);
                return;
            };
            if state.selection.anchor.is_none() {
                state.selection.anchor = Some(old_head);
            }
            let new_head = old_head.saturating_sub(1);
            state.selection.extend_to(new_head);
            cx.notify();
        });
    }

    pub fn handle_select_all(state: &Entity<TableState<D>>, _window: &mut Window, cx: &mut App) {
        state.update(cx, |state, cx| {
            state.select_all(cx);
            cx.notify();
        });
    }
}

impl<D: TableDelegate + 'static> RenderOnce for Table<D> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let (header, body) = self.state.update(cx, |state, cx| {
            (
                state.render_header(window, cx).into_any_element(),
                state.render_body(window, cx).into_any_element(),
            )
        });

        let focus_handle = &self.state.read(cx).focus_handle;

        div()
            .id("table")
            .track_focus(focus_handle)
            .key_context(action::KEY_CONTEXT)
            .flex()
            .flex_col()
            .size_full()
            .bg(cx.theme().bg_primary)
            .on_action::<action::ClearSelection>({
                let state = self.state.clone();
                move |_, window, cx| Self::handle_clear_selection(&state, window, cx)
            })
            .on_action::<action::EditSelection>({
                let state = self.state.clone();
                move |_, window, cx| Self::handle_edit_selection(&state, window, cx)
            })
            .on_action::<action::DeleteSelection>({
                let state = self.state.clone();
                move |_, window, cx| Self::handle_delete_selection(&state, window, cx)
            })
            .on_action::<action::NextColumn>({
                let state = self.state.clone();
                move |_, window, cx| Self::handle_next_column(&state, window, cx)
            })
            .on_action::<action::PrevColumn>({
                let state = self.state.clone();
                move |_, window, cx| Self::handle_prev_column(&state, window, cx)
            })
            .on_action::<action::NextRow>({
                let state = self.state.clone();
                move |_, window, cx| Self::handle_next_row(&state, window, cx)
            })
            .on_action::<action::PrevRow>({
                let state = self.state.clone();
                move |_, window, cx| Self::handle_prev_row(&state, window, cx)
            })
            .on_action::<action::ExtendSelectionNext>({
                let state = self.state.clone();
                move |_, window, cx| Self::handle_extend_selection_next(&state, window, cx)
            })
            .on_action::<action::ExtendSelectionPrev>({
                let state = self.state.clone();
                move |_, window, cx| Self::handle_extend_selection_prev(&state, window, cx)
            })
            .on_action::<action::SelectAll>({
                let state = self.state.clone();
                move |_, window, cx| Self::handle_select_all(&state, window, cx)
            })
            .on_prepaint({
                let state = self.state.clone();
                move |bounds, _, cx| {
                    state.update(cx, |state, _| {
                        state.bounds = bounds;
                    })
                }
            })
            .child(header)
            .child(body)
    }
}

impl<D: TableDelegate + 'static> TableState<D> {
    fn row_height(&self, window: &Window) -> Pixels {
        window.line_height()
    }

    fn column_width(&self, col_ix: usize) -> Pixels {
        self.column_widths[col_ix]
    }

    fn is_last_column(&self, col_ix: usize, cx: &Context<Self>) -> bool {
        col_ix == self.delegate().column_count(cx) - 1
    }

    pub fn render_header(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let height = self.row_height(window);
        let column_count = self.delegate().column_count(cx);

        let headers = (0..column_count)
            .map(|col_ix| self.render_header_cell(col_ix, cx).into_any_element())
            .collect::<Vec<_>>();

        h_flex()
            .id("table-head")
            .w_full()
            .min_h(height)
            .max_h(height)
            .bg(cx.theme().bg_secondary)
            .children(headers)
    }

    fn render_header_cell(&self, col_ix: usize, cx: &mut Context<Self>) -> impl IntoElement {
        let label = self.delegate().column(col_ix, cx).label().to_owned();

        div()
            .id(ElementId::named_usize("table-header-cell", col_ix))
            .w(self.column_width(col_ix))
            .h_full()
            .px_1()
            .flex_shrink_0()
            .overflow_hidden()
            .whitespace_nowrap()
            .bg(cx.theme().bg_secondary)
            .border_b_1()
            .when(!self.is_last_column(col_ix, cx), |e| e.border_r_1())
            .border_color(cx.theme().border_primary)
            .font_weight(FontWeight::BOLD)
            .hover(|e| e.bg(cx.theme().bg_secondary.hover()))
            .active(|e| e.bg(cx.theme().bg_secondary.active()))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, _, _, cx| {
                    this.selection.select_column(col_ix);
                    this.select_all(cx);
                    cx.notify();
                }),
            )
            .child(label)
    }

    pub fn render_body(&self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let visible_rows = self.rows.visible_rows().to_vec();
        let row_count = visible_rows.len();

        uniform_list(
            "table-list",
            row_count,
            cx.processor(move |this, range: Range<usize>, window, cx| {
                range
                    .map(|row_ix| {
                        let (row_id, depth) = &visible_rows[row_ix];
                        this.render_row(*depth, row_id, row_ix, window, cx).into_any_element()
                    })
                    .collect()
            }),
        )
        .flex()
        .flex_col()
        .size_full()
        .with_sizing_behavior(ListSizingBehavior::Infer)
        .track_scroll(&self.vertical_scroll_handle)
    }

    fn render_row(
        &self,
        depth: usize,
        row_id: &D::RowId,
        row_ix: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let row_height = self.row_height(window);
        let column_count = self.delegate().column_count(cx);
        let total_rows = self.rows.visible_rows().len();

        let is_row_selected = self.selection.contains(row_ix);
        let bg = if is_row_selected {
            cx.theme().bg_selected
        } else if row_ix % 2 == 0 {
            cx.theme().bg_table
        } else {
            cx.theme().bg_table_odd
        };

        let cells = (0..column_count)
            .map(|col_ix| self.render_cell(row_id, row_ix, col_ix, depth, window, cx))
            .collect::<Vec<_>>();

        h_flex()
            .id(ElementId::named_usize("table-row", row_ix))
            .w_full()
            .min_h(row_height)
            .max_h(row_height)
            .bg(bg)
            .when(row_ix + 1 < total_rows, |e| {
                e.border_b_1().border_color(cx.theme().border_primary)
            })
            .when(is_row_selected, |e| e.border_color(cx.theme().border_tertiary))
            .hover(|e| e.bg(bg.hover()))
            .active(|e| e.bg(bg.active()))
            .children(cells)
    }

    fn render_cell(
        &self,
        row_id: &D::RowId,
        row_ix: usize,
        col_ix: usize,
        depth: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Div {
        let selected_col = self.selected_column();
        let is_selected_row = self.selection_contains(row_id);
        let is_selected_cell = is_selected_row && col_ix == selected_col;

        let base = div()
            .relative()
            .w(self.column_width(col_ix))
            .h_full()
            .flex_shrink_0()
            .overflow_hidden()
            .whitespace_nowrap()
            .when(!self.is_last_column(col_ix, cx), |e| {
                e.border_r_1().border_color(cx.theme().border_primary)
            })
            .when(is_selected_cell, |e| {
                e.bg(cx.theme().bg_selected_extra).child(
                    div().absolute().inset_0().border_1().border_color(cx.theme().border_selected),
                )
            })
            .on_mouse_down(
                MouseButton::Right,
                cx.listener({
                    move |this, _, window, cx| {
                        if !this.selection.contains(row_ix) {
                            this.selection.select_cell(col_ix, row_ix);
                        }
                        this.edit_selection(window, cx);
                    }
                }),
            )
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, event: &MouseDownEvent, _, cx| {
                    if !event.modifiers.secondary() {
                        this.selection.clear();
                    }

                    this.selection.select_cell(col_ix, row_ix);
                    this.selection.start(col_ix, row_ix);
                    cx.notify();
                }),
            )
            .on_mouse_move(cx.listener(move |this, _, _, cx| {
                if this.selection.is_selecting {
                    this.selection.extend_to(row_ix);
                    cx.notify();
                }
            }))
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(move |this, _, _, cx| {
                    this.selection.finish();
                    cx.notify();
                }),
            )
            .on_mouse_up_out(
                MouseButton::Left,
                cx.listener(move |this, _, _, cx| {
                    this.selection.finish();
                    cx.notify();
                }),
            );

        let content = self.render_cell_content(row_id, col_ix, depth, window, cx);

        base.child(content)
    }

    fn render_cell_content(
        &self,
        row_id: &D::RowId,
        col_ix: usize,
        depth: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let base = self.delegate().render_cell(row_id, col_ix, window, cx).into_any_element();

        if !self.rows.is_tree() || col_ix != 0 {
            return base;
        }

        self.render_tree_cell(base, row_id, depth, window, cx)
    }

    fn render_tree_cell(
        &self,
        base: AnyElement,
        row_id: &D::RowId,
        depth: usize,
        window: &Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let row_height = self.row_height(window);
        let is_collapsible = self.rows.is_collapsible(row_id);
        let is_expanded = self.rows.is_expanded(row_id);

        let prefix = self
            .render_tree_prefix(row_id, depth, row_height, is_collapsible, is_expanded, cx)
            .into_any_element();

        h_flex()
            .h_full()
            .when(depth == 0, |e| e.font_weight(FontWeight::BOLD))
            .when(depth > 0, |e| e.text_color(cx.theme().fg_secondary))
            .child(prefix)
            .child(base)
            .into_any_element()
    }

    fn render_tree_prefix(
        &self,
        row_id: &D::RowId,
        depth: usize,
        row_height: Pixels,
        is_collapsible: bool,
        is_expanded: bool,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let mut items = Vec::with_capacity(depth + 1);

        for level in 0..=depth {
            let item = if level == 0 && is_collapsible {
                self.render_expand_button(row_id.clone(), is_expanded, row_height, cx)
                    .into_any_element()
            } else {
                div().w(row_height).h_full().into_any_element()
            };
            items.push(item);
        }

        h_flex().flex_row_reverse().children(items)
    }

    fn render_expand_button(
        &self,
        row_id: D::RowId,
        expanded: bool,
        size: Pixels,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let icon = if expanded {
            Icon::new(IconVariant::ChevronDown, IconSize::ExtraSmall)
        } else {
            Icon::new(IconVariant::ChevronRight, IconSize::ExtraSmall)
        };

        h_flex()
            .id("expand-button")
            .w(size)
            .h_full()
            .justify_center()
            .block_mouse_except_scroll()
            .on_click(cx.listener(move |this, _, _, cx| {
                this.rows.toggle_expanded(row_id.clone());
                this.selection.clear();
                cx.notify();
            }))
            .child(icon)
    }
}

// pub fn start_selection(&mut self, col_ix: usize, row_id: &D::RowId) {
//     if let Some(ix) = self.rows.visible_rows().iter().position(|(id, _)| id == row_id) {
//         self.selection.start(col_ix, ix);
//     }
// }

// pub fn end_selection(&mut self, row_id: &D::RowId, cx: &mut Context<Self>) {
//     if let Some(ix) = self.rows.visible_rows().iter().position(|(id, _)| id == row_id) {
//         self.selection.extend_to(ix);
//         self.selection.finish();
//         cx.notify();
//     }
// }

// pub fn selection_contains(&self, row_id: &D::RowId) -> bool {
//     if let Some(ix) = self.rows.visible_rows().iter().position(|(id, _)| id == row_id) {
//         self.selection.contains(ix)
//     } else {
//         false
//     }
// }
// pub fn selected_column_ix(&self) -> usize {
//     self.selection.selected_column_ix
// }

// pub fn select_cell(&mut self, col_ix: usize, row_id: &D::RowId, cx: &mut Context<Self>) {
//     if let Some(ix) = self.rows.visible_rows().iter().position(|(id, _)| id == row_id) {
//         self.selection.select_single(col_ix, ix);
//         cx.notify();
//     }
// }
