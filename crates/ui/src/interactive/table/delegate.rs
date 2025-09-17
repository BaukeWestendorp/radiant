use std::hash::Hash;

use gpui::prelude::*;
use gpui::{App, Window, div};

use crate::interactive::table::{Column, Table};

pub trait TableDelegate: Sized + 'static {
    type RowId: Clone + Eq + Hash;

    fn column_count(&self, cx: &App) -> usize;

    fn column(&self, col_ix: usize, cx: &App) -> &Column;

    fn column_ix(&self, column_id: &str, cx: &App) -> usize;

    fn sorted_row_ids(&self, cx: &App) -> Vec<Self::RowId>;

    fn can_select_multiple_rows(&self, _cx: &App) -> bool {
        true
    }

    fn validate(&self, _cx: &App) -> bool {
        true
    }

    fn edit_selection(
        &mut self,
        _column_id: &str,
        _row_ids: Vec<Self::RowId>,
        _window: &mut Window,
        _cx: &mut Context<Table<Self>>,
    ) {
    }

    fn render_empty(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Table<Self>>,
    ) -> impl IntoElement {
        div().size_full()
    }

    fn render_last_empty_col(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<Table<Self>>,
    ) -> impl IntoElement {
        div().flex().items_center().w_3().h_full().flex_shrink_0()
    }

    fn render_cell(
        &self,
        row_id: &Self::RowId,
        col_ix: usize,
        window: &mut Window,
        cx: &mut Context<Table<Self>>,
    ) -> impl IntoElement;
}
