use std::ops::Range;

use gpui::{
    AnyElement, App, Div, ElementId, Entity, FontWeight, ListSizingBehavior, MouseButton, Pixels,
    Window, div, prelude::*, px, uniform_list,
};

mod column;
mod delegate;
mod state;

pub use column::*;
pub use delegate::*;
pub use state::*;

use crate::{ActiveTheme, Icon, IconSize, IconVariant, h_flex, theme::HslaExt};

pub(crate) mod action {
    use gpui::{App, actions};

    actions!(root, []);

    pub const KEY_CONTEXT: &str = "Table";

    pub fn init(cx: &mut App) {
        cx.bind_keys([]);
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
}

impl<D: TableDelegate + 'static> RenderOnce for Table<D> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let (header, body) = self.state.update(cx, |state, cx| {
            (
                state.render_header(window, cx).into_any_element(),
                state.render_body(window, cx).into_any_element(),
            )
        });

        div()
            .id("table")
            .key_context(action::KEY_CONTEXT)
            .flex()
            .flex_col()
            .size_full()
            .bg(cx.theme().bg_primary)
            .child(header)
            .child(body)
    }
}

impl<D: TableDelegate + 'static> TableState<D> {
    fn row_height(&self, window: &mut Window) -> Pixels {
        window.line_height()
    }

    fn render_cell(&self, _col_ix: usize, _window: &mut Window, _cx: &mut Context<Self>) -> Div {
        let column_width = px(100.0);
        div().w(column_width).h_full().flex_shrink_0().overflow_hidden().whitespace_nowrap()
    }

    fn render_header_cell(
        &self,
        col_ix: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let delegate = self.delegate();
        let column_label = delegate.column(col_ix, cx).label().to_owned();
        let last_index = delegate.column_count(cx).saturating_sub(1);

        h_flex().h_full().child(
            self.render_cell(col_ix, window, cx)
                .id(ElementId::named_usize("th", col_ix))
                .bg(cx.theme().bg_secondary)
                .border_b_1()
                .when(col_ix < last_index, |e| e.border_r_1())
                .border_color(cx.theme().border_primary)
                .hover(|e| e.bg(cx.theme().bg_secondary.hover()))
                .active(|e| e.bg(cx.theme().bg_secondary.active()))
                .px_1()
                .font_weight(FontWeight::BOLD)
                .child(column_label),
        )
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
        let row_count = self.row_registry().sorted_visible_row_ids().len();
        let is_tree = self.row_registry().is_tree();
        let row_collapsible = self.row_registry().is_row_collapsible(row_id);
        let is_expanded = self.row_registry().row_expanded(row_id);

        let render_expand_button = |cx: &mut Context<Self>| {
            let icon = if is_expanded {
                Icon::new(IconVariant::ChevronDown, IconSize::ExtraSmall)
            } else {
                Icon::new(IconVariant::ChevronRight, IconSize::ExtraSmall)
            };

            h_flex()
                .id("expand-button")
                .size_full()
                .on_click(cx.listener({
                    let row_id = row_id.clone();
                    move |this, _, _, cx| {
                        this.row_registry_mut().toggle_row_expanded(row_id.clone());
                        cx.notify();
                    }
                }))
                .child(icon)
        };

        let build_cell_content = |base_td: AnyElement, col_ix, cx: &mut Context<Self>| {
            if !is_tree || col_ix != 0 {
                return base_td;
            }

            let mut prefix_elements = Vec::with_capacity(depth + 1);
            for d in 0..=depth {
                if d == 0 && row_collapsible {
                    let expand_button = h_flex()
                        .justify_center()
                        .w(row_height)
                        .h_full()
                        .child(render_expand_button(cx))
                        .into_any_element();
                    prefix_elements.push(expand_button);
                } else {
                    prefix_elements.push(div().w(row_height).h_full().into_any_element());
                }
            }

            h_flex()
                .h_full()
                .when(depth == 0, |e| e.font_weight(FontWeight::BOLD))
                .when(depth > 0, |e| e.text_color(cx.theme().fg_secondary))
                .child(h_flex().flex_row_reverse().children(prefix_elements))
                .child(base_td)
                .into_any_element()
        };

        let mut cells = Vec::with_capacity(column_count);
        for col_ix in 0..column_count {
            let base_td = self.delegate().render_td(row_id, col_ix, window, cx).into_any_element();

            let cell_content = build_cell_content(base_td, col_ix, cx);

            let cell = self
                .render_cell(col_ix, window, cx)
                .when(col_ix != column_count - 1, |e| {
                    e.border_r_1().border_color(cx.theme().border_primary)
                })
                .child(cell_content);

            cells.push(cell);
        }

        let bg = if row_ix % 2 == 0 { cx.theme().bg_table } else { cx.theme().bg_table_odd };

        h_flex()
            .id(ElementId::named_usize("table-row", row_ix))
            .w_full()
            .min_h(row_height)
            .max_h(row_height)
            .bg(bg)
            .when(row_ix != row_count - 1, |e| {
                e.border_b_1().border_color(cx.theme().border_primary)
            })
            .hover(|e| e.bg(bg.hover()))
            .active(|e| e.bg(bg.active()))
            .on_mouse_down(
                MouseButton::Right,
                cx.listener({
                    let row_id = row_id.clone();
                    move |this, _, _, cx| {
                        this.row_registry_mut().toggle_row_expanded(row_id.clone());
                        cx.notify();
                    }
                }),
            )
            .children(cells)
    }

    fn render_header(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let delegate = self.delegate();

        let mut headers = Vec::new();
        for col_ix in 0..delegate.column_count(cx) {
            let th = self.render_header_cell(col_ix, window, cx).into_any_element();
            headers.push(th);
        }

        h_flex()
            .id("table-head")
            .w_full()
            .min_h(self.row_height(window))
            .max_h(self.row_height(window))
            .bg(cx.theme().bg_secondary)
            .children(headers)
    }

    fn render_body(&self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let row_depth_ids = self.row_registry().sorted_visible_row_ids().to_vec();
        let row_count = row_depth_ids.len();

        uniform_list(
            "table-list",
            row_count,
            cx.processor(move |this, visible_range: Range<usize>, window, cx| {
                visible_range
                    .map(|row_ix| {
                        let (row_id, depth) = &row_depth_ids[row_ix];
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
}
