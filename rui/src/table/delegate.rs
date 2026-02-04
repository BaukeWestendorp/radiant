use gpui::{App, Window, prelude::*};

use crate::{Column, TableState};

pub trait TableDelegate {
    fn columns_count(&self, cx: &App) -> usize;

    fn rows_count(&self, cx: &App) -> usize;

    fn column(&self, col_ix: usize, cx: &App) -> &Column;

    fn render_td(
        &mut self,
        row_ix: usize,
        col_ix: usize,
        window: &mut Window,
        cx: &mut Context<TableState<Self>>,
    ) -> impl IntoElement
    where
        Self: Sized;
}
