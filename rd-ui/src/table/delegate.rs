use std::hash::Hash;

use crate::Column;

pub trait TableDelegate {
    type Row;

    type RowId: Hash + Eq;

    fn columns(&self) -> &[Column<Self>]
    where
        Self: Sized;

    fn rows(&self) -> impl IntoIterator<Item = &Self::Row>
    where
        Self: Sized;
}
