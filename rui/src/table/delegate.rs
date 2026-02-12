use std::hash::Hash;

use gpui::{App, Window, prelude::*};

use crate::Column;

pub trait TableDelegate {
    type RowId: Clone + Eq + Hash + std::fmt::Debug;

    fn column_count(&self, cx: &App) -> usize;

    fn column(&self, col_ix: usize, cx: &App) -> &Column;

    fn root_row_ids(&self, _cx: &App) -> Vec<Self::RowId>;

    fn row_children(&self, _row_id: &Self::RowId, _cx: &App) -> Vec<Self::RowId> {
        vec![]
    }

    fn edit_rows(&self, _row_ids: &[Self::RowId], _cx: &App) {}

    fn delete_rows(&self, _row_ids: &[Self::RowId], _cx: &App) {}

    fn render_cell(
        &self,
        row_id: &Self::RowId,
        col_ix: usize,
        window: &mut Window,
        cx: &App,
    ) -> impl IntoElement
    where
        Self: Sized;
}
