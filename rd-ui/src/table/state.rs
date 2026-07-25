use gpui::{App, Entity, EventEmitter, Window, prelude::*};

use crate::TableDelegate;

pub struct TableState<D: TableDelegate> {
    delegate: D,

    selection: Entity<TableSelection<D>>,

    sorted_column: Option<(String, TableSortDirection)>,
    cached_row_order: Option<Vec<usize>>,

    selection_drag: Option<(String, usize)>,
}

impl<D: TableDelegate + 'static> TableState<D> {
    pub fn new(delegate: D, _window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            delegate,

            selection: cx.new(|_| TableSelection::multiple(None, Vec::new())),

            sorted_column: None,
            cached_row_order: None,

            selection_drag: None,
        }
    }

    pub fn delegate(&self) -> &D {
        &self.delegate
    }

    pub fn delegate_mut(&mut self) -> &mut D {
        self.cached_row_order = None;
        &mut self.delegate
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

    pub fn sorted_rows(&self) -> Vec<(&D::RowId, &D::Row)> {
        let mut rows = self.delegate.rows().into_iter().collect::<Vec<_>>();

        if let Some(order) = &self.cached_row_order {
            if order.len() == rows.len() {
                return order.iter().map(|i| rows[*i]).collect();
            }
        }

        if let Some((col_id, direction)) = &self.sorted_column {
            if let Some(column) = self.delegate.columns().iter().find(|c| c.id() == col_id) {
                if let Some(sort_handler) = &column.sort_handler {
                    rows.sort_by(|(_, a), (_, b)| {
                        let cmp = sort_handler(a, b);
                        match direction {
                            TableSortDirection::Ascending => cmp,
                            TableSortDirection::Descending => cmp.reverse(),
                        }
                    });
                }
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

    pub fn update_sort_cache(&mut self) {
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

        indexed_rows.sort_by(|(_, (_, a)), (_, (_, b))| {
            let cmp = sort_handler(a, b);
            match direction {
                TableSortDirection::Ascending => cmp,
                TableSortDirection::Descending => cmp.reverse(),
            }
        });

        self.cached_row_order = Some(indexed_rows.into_iter().map(|(i, _)| i).collect());
    }

    pub fn select_all_in_column(&self, id: impl Into<String>, cx: &mut App) {
        let id = id.into();

        let selection = self.selection.read(cx);
        if selection.column.as_deref() == Some(&id) {
            return;
        }

        let rows = self.sorted_rows();
        let row_ids = rows.into_iter().map(|(id, _)| id.clone()).collect();

        self.selection.update(cx, |selection, cx| {
            selection.column = Some(id);
            match selection.kind {
                TableSelectionKind::Single(_) => {
                    selection.kind = TableSelectionKind::Multiple(row_ids);
                }
                TableSelectionKind::Multiple(_) => {
                    selection.kind = TableSelectionKind::Multiple(row_ids);
                }
            }
            cx.notify();
        });
    }

    pub(crate) fn start_selection_drag(
        &mut self,
        column_id: String,
        row_ix: usize,
        cx: &mut Context<Self>,
    ) {
        self.selection_drag = Some((column_id, row_ix));
        self.update_selection_drag(row_ix, cx);
    }

    pub(crate) fn is_dragging_selection(&self) -> bool {
        self.selection_drag.is_some()
    }

    pub(crate) fn update_selection_drag(&mut self, current_row_ix: usize, cx: &mut Context<Self>) {
        let Some((column_id, start_row_ix)) = self.selection_drag.clone() else {
            return;
        };

        let start = start_row_ix.min(current_row_ix);
        let end = start_row_ix.max(current_row_ix);

        let rows = self.sorted_rows();
        let selected_rows =
            rows.into_iter().skip(start).take(end - start + 1).map(|(id, _)| id.clone()).collect();

        self.selection.update(cx, |selection, cx| {
            selection.column = Some(column_id);
            selection.kind = TableSelectionKind::Multiple(selected_rows);
            cx.notify();
        });
    }

    pub(crate) fn stop_selection_drag(&mut self, _row_ix: usize, _cx: &mut Context<Self>) {
        self.selection_drag = None;
    }
}

pub struct TableSelection<D: TableDelegate> {
    pub column: Option<String>,
    pub kind: TableSelectionKind<D>,
}

impl<D: TableDelegate> TableSelection<D> {
    pub fn single(column: Option<String>, row: Option<D::RowId>) -> Self {
        Self { column, kind: TableSelectionKind::Single(row) }
    }

    pub fn multiple(column: Option<String>, rows: Vec<D::RowId>) -> Self {
        Self { column, kind: TableSelectionKind::Multiple(rows) }
    }

    pub fn is_column_selected(&self, id: &str) -> bool {
        self.column.as_deref() == Some(id)
    }

    pub fn is_row_selected(&self, row_id: &D::RowId) -> bool {
        match &self.kind {
            TableSelectionKind::Single(selected_row) => selected_row.as_ref() == Some(row_id),
            TableSelectionKind::Multiple(selected_rows) => selected_rows.contains(row_id),
        }
    }

    pub fn is_cell_selected(&self, column_id: &str, row_id: &D::RowId) -> bool {
        self.is_column_selected(column_id) && self.is_row_selected(row_id)
    }
}

pub enum TableSelectionKind<D: TableDelegate> {
    Single(Option<D::RowId>),
    Multiple(Vec<D::RowId>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TableSortDirection {
    #[default]
    Ascending,
    Descending,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableEvent {
    EditSubmitted,
}

impl<D: TableDelegate + 'static> EventEmitter<TableEvent> for TableState<D> {}
