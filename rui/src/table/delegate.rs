use gpui::{App, Window, prelude::*};

use crate::Column;

pub trait TableDelegate {
    fn column_count(&self, cx: &App) -> usize;

    fn row_count(&self, cx: &App) -> usize;

    fn column(&self, col_ix: usize, cx: &App) -> &Column;

    fn render_td(
        &self,
        row_ix: usize,
        col_ix: usize,
        window: &mut Window,
        cx: &App,
    ) -> impl IntoElement
    where
        Self: Sized;
}
