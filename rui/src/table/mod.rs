use gpui::{
    App, Div, DragMoveEvent, ElementId, EmptyView, Entity, EntityId, FontWeight, Pixels,
    SharedString, Window, div, prelude::*, px,
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

impl<D: TableDelegate> Table<D> {
    pub fn new(state: Entity<TableState<D>>) -> Self {
        Self { state }
    }

    fn render_cell(&self, col_ix: usize, _window: &mut Window, cx: &mut App) -> Div {
        let column_width = self.state.read(cx).column_bounds[col_ix].size.width;

        div().w(column_width).h_full().flex_shrink_0().overflow_hidden().whitespace_nowrap()
    }

    fn render_th(&self, col_ix: usize, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let delegate = self.state.read(cx).delegate();
        let column_label = delegate.column(col_ix, cx).label().to_owned();
        let column_count = delegate.columns_count(cx) - 1;

        h_flex()
            .h_full()
            .child(
                self.render_cell(col_ix, window, cx)
                    .id(ElementId::named_usize("th", col_ix))
                    .bg(cx.theme().bg_secondary)
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

    fn render_tr(&self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
    }

    fn render_header(&self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let delegate = self.state.read(cx).delegate();

        let mut ths = Vec::new();
        for col_ix in 0..delegate.columns_count(cx) {
            let th = self.render_th(col_ix, window, cx).into_any_element();
            ths.push(th);
        }

        h_flex().id("table-head").w_full().bg(cx.theme().bg_secondary).children(ths)
    }

    fn render_data_rows(&self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
    }

    fn render_resize_handle(
        &self,
        col_ix: usize,
        _window: &mut Window,
        cx: &mut App,
    ) -> impl IntoElement {
        const HANDLE_SIZE: Pixels = px(2.0);

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
                    .w(px(1.)),
            )
            .on_drag_move({
                let state = self.state.clone();
                move |event: &DragMoveEvent<(EntityId, usize)>, _, cx| {
                    let (entity_id, col_ix) = *event.drag(cx);

                    if state.entity_id() != entity_id {
                        return;
                    }

                    let col_bounds = state.read(cx).column_bounds[col_ix];

                    state.update(cx, |state, cx| {
                        state.resize_col(
                            col_ix,
                            event.event.position.x - HANDLE_SIZE - col_bounds.left(),
                            cx,
                        );

                        cx.notify();
                    });
                }
            })
            .on_drag((self.state.entity_id(), col_ix), |_, _, _, cx| {
                cx.stop_propagation();
                cx.new(|_| EmptyView)
            })
            .into_any_element()
    }
}

impl<D: TableDelegate + 'static> RenderOnce for Table<D> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .id("table")
            .key_context(action::KEY_CONTEXT)
            .flex()
            .flex_col()
            .size_full()
            .bg(cx.theme().bg_primary)
            .child(self.render_header(window, cx))
            .child(self.render_data_rows(window, cx))
            .on_prepaint({
                let state = self.state.clone();
                move |bounds, _, cx| state.update(cx, |state, _| state.table_bounds = Some(bounds))
            })
    }
}
