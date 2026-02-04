use gpui::{App, Window, prelude::*};

use crate::Column;

pub trait TableDelegate {
    type RowId;

    fn column_count(&self, cx: &App) -> usize;

    fn row_count(&self, cx: &App) -> usize;

    fn column(&self, col_ix: usize, cx: &App) -> &Column;

    fn row_id(&self, row_ix: usize, cx: &App) -> Self::RowId;

    fn root_rows(&self, _cx: &App) -> impl Iterator<Item = Self::RowId> {
        std::iter::empty()
    }

    fn row_children(&self, _row_id: Self::RowId, _cx: &App) -> impl Iterator<Item = Self::RowId> {
        std::iter::empty()
    }

    fn tree_mode_enabled(&self, cx: &App) -> bool {
        self.root_rows(cx).nth(0).is_some()
    }

    fn render_td(
        &self,
        row_id: &Self::RowId,
        col_ix: usize,
        window: &mut Window,
        cx: &App,
    ) -> impl IntoElement
    where
        Self: Sized;
}
