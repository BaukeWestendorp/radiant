use gpui::{App, Entity};

use crate::Column;

pub trait TableDelegate {
    type Row;

    fn columns(&self, cx: &App) -> impl Iterator<Item = &Column<Self>>
    where
        Self: Sized;

    fn column(&self, column_id: &str, cx: &App) -> Option<&Column<Self>>
    where
        Self: Sized;

    fn rows(&self) -> Entity<Vec<Self::Row>>;

    fn insert_new_row(&self, last_item_ix: Option<usize>, cx: &mut App) -> Option<usize>;
}
