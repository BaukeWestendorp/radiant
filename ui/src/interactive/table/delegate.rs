use gpui::prelude::*;
use gpui::{App, Window, div};

use crate::interactive::table::{Column, Selection, Table};

pub trait TableDelegate: Sized + 'static {
    fn column_count(&self, cx: &App) -> usize;
    fn row_count(&self, cx: &App) -> usize;

    fn column(&self, col_ix: usize, cx: &App) -> &Column;

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
        row_ix: usize,
        col_ix: usize,
        window: &mut Window,
        cx: &mut Context<Table<Self>>,
    ) -> impl IntoElement;

    fn edit_selection(&mut self, _selection: Selection, _cx: &mut App) {}
}
