use std::hash::Hash;

use gpui::{App, Window, prelude::*};

use crate::Column;

pub trait TableDelegate {
    type RowId: Clone + Eq + Hash;

    fn column_count(&self, cx: &App) -> usize;

    fn column(&self, col_ix: usize, cx: &App) -> &Column;

    fn root_row_ids(&self, _cx: &App) -> Vec<Self::RowId>;

    fn row_children(&self, _row_id: &Self::RowId, _cx: &App) -> Vec<Self::RowId> {
        vec![]
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
