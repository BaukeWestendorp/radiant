use std::hash::Hash;

use crate::Column;

pub trait TableDelegate {
    type Row;

    type RowId: Clone + Hash + Eq;

    fn columns(&self) -> &[Column<Self>]
    where
        Self: Sized;

    fn column(&self, column_id: &str) -> Option<&Column<Self>>
    where
        Self: Sized;

    fn rows(&self) -> impl Iterator<Item = (&Self::RowId, &Self::Row)>
    where
        Self: Sized;

    fn row_count(&self) -> usize;

    fn row(&self, row_id: &Self::RowId) -> Option<&Self::Row>
    where
        Self: Sized;

    fn row_mut(&mut self, row_id: &Self::RowId) -> Option<&mut Self::Row>
    where
        Self: Sized;

    fn row_ids(&self) -> impl Iterator<Item = &Self::RowId>
    where
        Self: Sized,
    {
        self.rows().map(|(row_id, _)| row_id)
    }
}
