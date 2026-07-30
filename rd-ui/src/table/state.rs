use gpui::{App, Context, Entity, EventEmitter, FocusHandle, Focusable, Window, prelude::*};

use crate::TableDelegate;

pub struct TableState<D: TableDelegate> {
    delegate: D,

    selection: Entity<TableSelection<D>>,

    sorted_column: Option<(String, TableSortDirection)>,
    cached_row_order: Option<Vec<D::RowId>>,

    pub(crate) selection_drag: Option<(String, usize)>,

    focus_handle: FocusHandle,
}

impl<D: TableDelegate + 'static> TableState<D> {
    pub fn new(
        delegate: D,
        focus_handle: FocusHandle,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let first_sortable_column_id =
            delegate.columns(cx).find(|col| col.sortable()).map(|col| col.id().to_string());

        let mut this = Self {
            delegate,

            selection: cx.new(|_| TableSelection::multiple(None, Vec::new())),

            sorted_column: None,
            cached_row_order: None,

            focus_handle,

            selection_drag: None,
        };

        if let Some(column_id) = first_sortable_column_id {
            this.sort_by_column(column_id, TableSortDirection::Ascending, cx);
        }

        this
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

    pub fn sorted_rows<'a>(&'a self, cx: &'a App) -> Vec<(&'a D::RowId, &'a D::Row)> {
        let rows_map = self.delegate.rows().read(cx);

        if let Some(order) = &self.cached_row_order {
            if order.len() == rows_map.len() {
                return order
                    .iter()
                    .filter_map(|id| rows_map.get(id).map(|row| (id, row)))
                    .collect();
            }
        }

        let mut rows: Vec<_> = rows_map.iter().collect();

        if let Some((col_id, direction)) = &self.sorted_column {
            if let Some(column) = self.delegate.columns(cx).find(|c| c.id() == col_id) {
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

    pub fn sort_by_column(
        &mut self,
        column_id: impl Into<String>,
        direction: TableSortDirection,
        cx: &App,
    ) {
        self.sorted_column = Some((column_id.into(), direction));
        self.update_sort_cache(cx);
    }

    pub fn clear_sort(&mut self) {
        self.sorted_column = None;
        self.cached_row_order = None;
    }

    pub fn update_sort_cache(&mut self, cx: &App) {
        let Some((col_id, direction)) = &self.sorted_column else {
            self.cached_row_order = None;
            return;
        };

        let Some(column) = self.delegate.columns(cx).find(|c| c.id() == col_id) else {
            self.cached_row_order = None;
            return;
        };

        let Some(sort_handler) = &column.sort_handler else {
            self.cached_row_order = None;
            return;
        };

        let rows_map = self.delegate.rows().read(cx);
        let mut rows: Vec<_> = rows_map.iter().collect();

        rows.sort_by(|(_, a), (_, b)| {
            let cmp = sort_handler(a, b);
            match direction {
                TableSortDirection::Ascending => cmp,
                TableSortDirection::Descending => cmp.reverse(),
            }
        });

        self.cached_row_order = Some(rows.into_iter().map(|(id, _)| (*id).clone()).collect());
    }

    pub fn select_all_in_column(&self, id: impl Into<String>, cx: &mut App) {
        let id = id.into();

        let rows = self.sorted_rows(cx);
        let row_ids = rows.into_iter().map(|(id, _)| id.clone()).collect::<Vec<_>>();

        self.selection.update(cx, |selection, cx| {
            selection.column_id = Some(id);
            match selection.kind {
                TableSelectionKind::Single(_) => {
                    selection.kind = TableSelectionKind::Single(row_ids.first().cloned());
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

        let rows = self.sorted_rows(cx);
        let selected_rows =
            rows.into_iter().skip(start).take(end - start + 1).map(|(id, _)| id.clone()).collect();

        self.selection.update(cx, |selection, cx| {
            selection.column_id = Some(column_id);
            selection.kind = TableSelectionKind::Multiple(selected_rows);
            cx.notify();
        });
    }

    pub(crate) fn stop_selection_drag(&mut self, row_ix: usize, cx: &mut Context<Self>) {
        self.update_selection_drag(row_ix, cx);
        self.selection_drag = None;
    }

    pub fn can_edit(&self, cx: &App) -> bool {
        let selection = self.selection().read(cx);
        let Some(column_id) = &selection.column_id else { return false };
        let Some(column) = self.delegate.column(column_id, cx) else { return false };
        !selection.is_empty() && column.editable()
    }
}

impl<D: TableDelegate + 'static> Focusable for TableState<D> {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

pub struct TableSelection<D: TableDelegate> {
    pub column_id: Option<String>,
    pub kind: TableSelectionKind<D>,
}

impl<D: TableDelegate> TableSelection<D> {
    pub fn single(column: Option<String>, row: Option<D::RowId>) -> Self {
        Self { column_id: column, kind: TableSelectionKind::Single(row) }
    }

    pub fn multiple(column: Option<String>, rows: Vec<D::RowId>) -> Self {
        Self { column_id: column, kind: TableSelectionKind::Multiple(rows) }
    }

    pub fn row_ids(&self) -> Box<dyn Iterator<Item = &D::RowId> + '_> {
        match &self.kind {
            TableSelectionKind::Single(row) => Box::new(row.iter()),
            TableSelectionKind::Multiple(rows) => Box::new(rows.iter()),
        }
    }

    pub fn count(&self) -> usize {
        match &self.kind {
            TableSelectionKind::Single(row) => row.iter().count(),
            TableSelectionKind::Multiple(rows) => rows.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }

    pub fn is_column_selected(&self, id: &str) -> bool {
        self.column_id.as_deref() == Some(id)
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

    pub fn clear(&mut self) {
        self.column_id = None;
        match self.kind {
            TableSelectionKind::Single(_) => self.kind = TableSelectionKind::Single(None),
            TableSelectionKind::Multiple(_) => self.kind = TableSelectionKind::Multiple(Vec::new()),
        }
    }

    pub fn select_cell(&mut self, column_id: String, row_id: D::RowId) {
        match &self.kind {
            TableSelectionKind::Single(_) => {
                self.kind = TableSelectionKind::Single(Some(row_id));
            }
            TableSelectionKind::Multiple(selected_rows) => {
                let mut new_selected_rows = selected_rows.clone();
                if !new_selected_rows.contains(&row_id) {
                    new_selected_rows.push(row_id);
                }
                self.kind = TableSelectionKind::Multiple(new_selected_rows);
            }
        }
        self.column_id = Some(column_id);
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
