use std::ops::Range;

use gpui::{
    App, Div, DragMoveEvent, ElementId, EmptyView, Entity, EntityId, FontWeight,
    ListSizingBehavior, Pixels, SharedString, Window, div, prelude::*, px, uniform_list,
};

mod column;
mod delegate;
mod state;

pub use column::*;
pub use delegate::*;
pub use state::*;

use crate::{ActiveTheme, ElementExt, h_flex, theme::HslaExt};

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
            .on_prepaint({
                let state = self.state.clone();
                move |bounds, _, cx| {
                    state.update(cx, |state, cx| {
                        state.table_bounds = Some(bounds);
                        state.update_column_widths(cx);
                        cx.notify();
                    })
                }
            })
    }
}

impl<D: TableDelegate + 'static> TableState<D> {
    fn row_height(&self, window: &mut Window) -> Pixels {
        window.line_height()
    }

    fn render_cell(&self, col_ix: usize, _window: &mut Window, _cx: &mut Context<Self>) -> Div {
        let column_width = self.column_bounds[col_ix].size.width;
        div().w(column_width).h_full().flex_shrink_0().overflow_hidden().whitespace_nowrap()
    }

    fn render_th(
        &self,
        col_ix: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let delegate = self.delegate();
        let column_label = delegate.column(col_ix, cx).label().to_owned();
        let column_count = delegate.column_count(cx) - 1;

        h_flex()
            .h_full()
            .child(
                self.render_cell(col_ix, window, cx)
                    .id(ElementId::named_usize("th", col_ix))
                    .bg(cx.theme().bg_secondary)
                    .border_b_1()
                    .border_color(cx.theme().border_primary)
                    .hover(|e| e.bg(cx.theme().bg_secondary.hover()))
                    .active(|e| e.bg(cx.theme().bg_secondary.active()))
                    .px_1()
                    .font_weight(FontWeight::BOLD)
                    .child(column_label),
            )
            .children(
                (col_ix != column_count).then(|| self.render_resize_handle(col_ix, window, cx)),
            )
    }

    fn render_tr(
        &self,
        row_ix: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let column_count = self.delegate().column_count(cx);
        let row_count = self.delegate().row_count(cx);

        let mut tds = Vec::new();
        for col_ix in 0..column_count {
            let td = self.delegate().render_td(row_ix, col_ix, window, cx).into_any_element();
            tds.push(
                self.render_cell(col_ix, window, cx)
                    .when(col_ix != column_count - 1, |e| {
                        e.border_r_1().border_color(cx.theme().border_primary)
                    })
                    .child(td),
            );
        }

        let bg = if row_ix % 2 == 0 { cx.theme().bg_table } else { cx.theme().bg_table_odd };

        h_flex()
            .id(ElementId::named_usize("table-row", row_ix))
            .w_full()
            .min_h(self.row_height(window))
            .max_h(self.row_height(window))
            .bg(bg)
            .when(row_ix != row_count - 1, |e| {
                e.border_b_1().border_color(cx.theme().border_primary)
            })
            .hover(|e| e.bg(bg.hover()))
            .active(|e| e.bg(bg.active()))
            .children(tds)
    }

    fn render_header(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let delegate = self.delegate();

        let mut ths = Vec::new();
        for col_ix in 0..delegate.column_count(cx) {
            let th = self.render_th(col_ix, window, cx).into_any_element();
            ths.push(th);
        }

        h_flex()
            .id("table-head")
            .w_full()
            .min_h(self.row_height(window))
            .max_h(self.row_height(window))
            .bg(cx.theme().bg_secondary)
            .children(ths)
    }

    fn render_body(&self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let row_ixs = self.sorted_row_ixs(cx);
        let row_count = row_ixs.len();

        uniform_list(
            "table-list",
            row_count,
            cx.processor(move |this, visible_range: Range<usize>, window, cx| {
                visible_range
                    .map(|row_ix| this.render_tr(row_ix, window, cx).into_any_element())
                    .collect()
            }),
        )
        .flex()
        .flex_col()
        .size_full()
        .with_sizing_behavior(ListSizingBehavior::Infer)
        .track_scroll(&self.vertical_scroll_handle)
    }

    fn render_resize_handle(
        &self,
        col_ix: usize,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        const HANDLE_SIZE: Pixels = px(4.0);

        let group_id = SharedString::from(format!("resizable-handle:{}", col_ix));

        h_flex()
            .id(ElementId::named_usize("resizable-handle", col_ix))
            .group(group_id.clone())
            .occlude()
            .cursor_col_resize()
            .h_full()
            .w(HANDLE_SIZE)
            .ml(-(HANDLE_SIZE))
            .justify_end()
            .items_center()
            .child(
                div()
                    .h_full()
                    .justify_center()
                    .bg(cx.theme().border_secondary)
                    .group_hover(&group_id, |this| {
                        this.bg(cx.theme().border_secondary.hover()).h_full()
                    })
                    .w(px(1.0)),
            )
            .on_drag_move(cx.listener({
                let this_entity_id = cx.entity_id();
                move |this, event: &DragMoveEvent<(EntityId, usize)>, _, cx| {
                    let (entity_id, col_ix) = *event.drag(cx);

                    if this_entity_id != entity_id {
                        return;
                    }

                    let col_bounds = this.column_bounds[col_ix];

                    this.resize_col(
                        col_ix,
                        event.event.position.x - HANDLE_SIZE - col_bounds.left(),
                        cx,
                    );

                    cx.notify();
                }
            }))
            .on_drag((cx.entity_id(), col_ix), |_, _, _, cx| {
                cx.stop_propagation();
                cx.new(|_| EmptyView)
            })
            .into_any_element()
    }
}
