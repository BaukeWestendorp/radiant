use gpui::{Entity, Window, prelude::*};

use crate::TableDelegate;

pub struct TableState<D: TableDelegate> {
    delegate: D,

    selection: Entity<TableSelection<D>>,

    sorted_column: Option<(String, TableSortDirection)>,
    cached_row_order: Option<Vec<usize>>,
}

impl<D: TableDelegate + 'static> TableState<D> {
    pub fn new(delegate: D, _window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            delegate,

            selection: cx.new(|_| TableSelection::Multiple(Vec::new())),

            sorted_column: None,
            cached_row_order: None,
        }
    }

    pub fn delegate(&self) -> &D {
        &self.delegate
    }

    pub fn selection(&self) -> &Entity<TableSelection<D>> {
        &self.selection
    }

    pub fn with_selection(mut self, selection: Entity<TableSelection<D>>) -> Self {
        self.selection = selection;
        self
    }

    pub fn sorted_column(&self) -> Option<&str> {
        self.sorted_column.as_ref().map(|(id, _)| id.as_str())
    }

    pub fn sort_direction(&self) -> Option<TableSortDirection> {
        self.sorted_column.as_ref().map(|(_, direction)| *direction)
    }

    pub fn sorted_rows(&self) -> Vec<&D::Row> {
        let rows = self.delegate.rows().into_iter().collect::<Vec<_>>();

        if let Some(order) = &self.cached_row_order {
            if order.len() == rows.len() {
                return order.iter().map(|i| rows[*i]).collect();
            }
        }

        rows
    }

    pub fn sort_by_column(&mut self, column_id: impl Into<String>, direction: TableSortDirection) {
        self.sorted_column = Some((column_id.into(), direction));
        self.update_sort_cache();
    }

    pub fn clear_sort(&mut self) {
        self.sorted_column = None;
        self.cached_row_order = None;
    }

    fn update_sort_cache(&mut self) {
        let Some((col_id, direction)) = &self.sorted_column else {
            self.cached_row_order = None;
            return;
        };

        let Some(column) = self.delegate.columns().iter().find(|c| c.id() == col_id) else {
            self.cached_row_order = None;
            return;
        };

        let Some(sort_handler) = &column.sort_handler else {
            self.cached_row_order = None;
            return;
        };

        let mut indexed_rows: Vec<_> = self.delegate.rows().into_iter().enumerate().collect();

        indexed_rows.sort_by(|(_, a), (_, b)| {
            let cmp = sort_handler(*a, *b);
            match direction {
                TableSortDirection::Ascending => cmp,
                TableSortDirection::Descending => cmp.reverse(),
            }
        });

        self.cached_row_order = Some(indexed_rows.into_iter().map(|(i, _)| i).collect());
    }
}

pub enum TableSelection<D: TableDelegate> {
    Single(Option<D::RowId>),
    Multiple(Vec<D::RowId>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TableSortDirection {
    #[default]
    Ascending,
    Descending,
}
