use std::{collections::HashMap, hash::Hash};

use gpui::Entity;

use gpui::App;

use crate::Column;

pub trait TableDelegate {
    type Row;

    type RowId: Clone + Hash + Eq;

    fn columns(&self, cx: &App) -> impl Iterator<Item = &Column<Self>>
    where
        Self: Sized;

    fn column(&self, column_id: &str, cx: &App) -> Option<&Column<Self>>
    where
        Self: Sized;

    fn rows(&self) -> Entity<HashMap<Self::RowId, Self::Row>>;

    fn insert_new_row(&self, _cx: &mut App) -> Option<Self::RowId>;
}
