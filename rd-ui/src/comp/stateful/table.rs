use std::rc::Rc;

use gpui::{
    AnyElement, App, ElementId, Entity, EventEmitter, FocusHandle, Focusable, MouseButton,
    MouseMoveEvent, Pixels, SharedString, Window, div, prelude::*,
};

use crate::{
    ActiveTheme, Emphasis, InputPopup, PopupAppExt, StyledExt, StyledParentExt,
    StyledStatefulInteractiveElementExt,
    comp::{FocusableComponent, INPUT_SIZE, Identifiable, stateful},
    h_flex, v_flex,
};

pub(crate) mod action {
    pub const KEY_CONTEXT: &str = "Table";

    gpui::actions!(
        table,
        [PrevRow, NextRow, ExtendSelectionPrev, ExtendSelectionNext, PrevColumn, NextColumn]
    );
}

pub struct Table<Row> {
    id: ElementId,
    focus_handle: FocusHandle,

    rows: Entity<Vec<Row>>,
    columns: Vec<TableColumn<Row>>,
    selection: TableSelection,

    on_delete: Option<Box<dyn Fn(&[usize], usize, &mut Window, &mut App) + 'static>>,
    on_edit: Option<Box<dyn Fn(&[usize], usize, FocusHandle, &mut Window, &mut App) + 'static>>,
}

impl<Row: 'static> Table<Row> {
    pub fn new(
        id: impl Into<ElementId>,
        rows: Entity<Vec<Row>>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            id: id.into(),
            focus_handle: cx.focus_handle().tab_stop(true),

            rows,
            columns: Vec::new(),
            selection: TableSelection::default(),

            on_delete: None,
            on_edit: None,
        }
    }

    pub fn rows(&self) -> Entity<Vec<Row>> {
        self.rows.clone()
    }

    pub fn columns(&self) -> &[TableColumn<Row>] {
        &self.columns
    }

    pub fn set_columns(&mut self, columns: Vec<TableColumn<Row>>) {
        self.columns = columns;
    }

    pub fn with_columns(mut self, columns: Vec<TableColumn<Row>>) -> Self {
        self.columns = columns;
        self
    }

    pub fn selection(&self) -> &TableSelection {
        &self.selection
    }

    pub fn selection_mut(&mut self) -> &mut TableSelection {
        &mut self.selection
    }

    pub fn set_selection(&mut self, selection: TableSelection, cx: &mut Context<Self>) {
        self.selection = selection;
        cx.emit(stateful::event::SelectionChanged);
    }

    pub fn with_selection(mut self, selection: TableSelection) -> Self {
        self.selection = selection;
        self
    }

    pub fn selected_rows<'a>(&'a self, cx: &'a App) -> Vec<&'a Row> {
        let rows = self.rows.read(cx);
        self.selection.rows().iter().filter_map(|&row_ix| rows.get(row_ix)).collect()
    }

    pub fn set_on_delete<F>(&mut self, on_delete: F)
    where
        F: Fn(&[usize], usize, &mut Window, &mut App) + 'static,
    {
        self.on_delete = Some(Box::new(on_delete));
    }

    pub fn with_on_delete<F>(mut self, on_delete: F) -> Self
    where
        F: Fn(&[usize], usize, &mut Window, &mut App) + 'static,
    {
        self.on_delete = Some(Box::new(on_delete));
        self
    }

    pub fn set_on_edit<F>(&mut self, on_edit: F)
    where
        F: Fn(&[usize], usize, FocusHandle, &mut Window, &mut App) + 'static,
    {
        self.on_edit = Some(Box::new(on_edit));
    }

    pub fn with_on_edit<F>(mut self, on_edit: F) -> Self
    where
        F: Fn(&[usize], usize, FocusHandle, &mut Window, &mut App) + 'static,
    {
        self.on_edit = Some(Box::new(on_edit));
        self
    }

    pub fn select_all(&mut self, cx: &mut Context<Self>) {
        self.selection.select_all(self.rows.read(cx).len());
        cx.emit(stateful::event::SelectionChanged);
    }

    pub fn select_prev_row(&mut self, preserve_existing: bool, cx: &mut Context<Self>) {
        let Some(last_row_ix) = self.rows.read(cx).len().checked_sub(1) else {
            return;
        };
        let row_ix =
            self.selection.selected_row().map_or(last_row_ix, |row_ix| row_ix.saturating_sub(1));
        self.selection.move_row_selection(row_ix, preserve_existing);
        cx.emit(stateful::event::SelectionChanged);
    }

    pub fn select_next_row(&mut self, preserve_existing: bool, cx: &mut Context<Self>) {
        let Some(last_row_ix) = self.rows.read(cx).len().checked_sub(1) else {
            return;
        };
        let row_ix =
            self.selection.selected_row().map_or(0, |row_ix| (row_ix + 1).min(last_row_ix));
        self.selection.move_row_selection(row_ix, preserve_existing);
        cx.emit(stateful::event::SelectionChanged);
    }

    pub fn select_prev_column(&mut self, cx: &mut Context<Self>) {
        let Some(last_column_ix) = self.columns.len().checked_sub(1) else {
            return;
        };
        if !self.selection.has_rows() {
            if self.rows.read(cx).is_empty() {
                return;
            }

            self.selection.move_row_selection(0, false);
            self.selection.move_column_selection(last_column_ix);
            return;
        }
        self.selection.move_column_selection(self.selection.column().saturating_sub(1));
        cx.emit(stateful::event::SelectionChanged);
    }

    pub fn select_next_column(&mut self, cx: &mut Context<Self>) {
        let Some(last_column_ix) = self.columns.len().checked_sub(1) else {
            return;
        };
        if !self.selection.has_rows() {
            if self.rows.read(cx).is_empty() {
                return;
            }

            self.selection.move_row_selection(0, false);
            self.selection.move_column_selection(0);
            return;
        }
        let column_ix = (self.selection.column() + 1).min(last_column_ix);
        self.selection.move_column_selection(column_ix);
        cx.emit(stateful::event::SelectionChanged);
    }

    pub fn delete_selection(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(on_delete) = &self.on_delete {
            on_delete(self.selection.rows(), self.selection.column, window, cx);
        }
    }

    pub fn edit_selection(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        // FIXME: It would be nice if the field could be preloaded with the (first) value that is being edited.

        if let Some(on_edit) = &self.on_edit {
            on_edit(
                self.selection.rows(),
                self.selection.column,
                self.focus_handle.clone(),
                window,
                cx,
            );
        }

        if let Some(column) = self.columns.get(self.selection.column) {
            if let Some(on_edit) = &column.on_edit {
                on_edit(self.selection.rows(), &self.rows, self.focus_handle.clone(), window, cx);
            }
        }
    }

    #[inline(always)]
    fn row_height(&self) -> Pixels {
        INPUT_SIZE
    }

    fn render_header(&self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let cells = self.columns.iter().enumerate().map(|(ix, column)| {
            let id = format!("th-cell-{}", ix);

            h_flex()
                .id(id)
                .interactive_emphasis(Emphasis::Secondary, cx)
                .w_full()
                .h_full()
                .px_1()
                .border_color(cx.theme().border_secondary)
                .border_b_1()
                .font_semibold()
                .text_color(cx.theme().fg_secondary)
                .when(ix != 0, |e| e.border_l_1())
                .child(column.label.clone())
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.selection_mut().set_column(ix);
                    this.select_all(cx);
                    cx.emit(stateful::event::SelectionChanged);
                    cx.notify();
                }))
        });

        h_flex().w_full().min_h(self.row_height()).max_h(self.row_height()).children(cells)
    }

    fn render_body(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let row_count = self.rows.read(cx).len();
        let rows = (0..row_count)
            .map(|row_ix| self.render_row(row_ix, window, cx).into_any_element())
            .collect::<Vec<_>>();

        v_flex().id("body").overflow_scroll().bg(cx.theme().bg_table).size_full().children(rows)
    }

    fn render_row(
        &self,
        row_ix: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let cells = self
            .columns
            .iter()
            .enumerate()
            .map(|(col_ix, column)| {
                let row = self.rows().read(cx).get(row_ix).unwrap();

                let selected = self.selection.cell_selected(row_ix, col_ix);

                h_flex()
                    .relative()
                    .size_full()
                    .border_color(cx.theme().border_secondary)
                    .border_b_1()
                    .when(col_ix != 0, |e| e.border_l_1())
                    .when(selected, |e| {
                        e.child(
                            div()
                                .size_full()
                                .absolute()
                                .inset_0()
                                .border_1()
                                .border_color(cx.theme().border_selected)
                                .bg(cx.theme().bg_selected),
                        )
                    })
                    .child(
                        div()
                            .overflow_hidden()
                            .truncate()
                            .size_full()
                            .absolute()
                            .inset_0()
                            .px_1()
                            .child(
                                (column.render)
                                    .as_ref()
                                    .map(|render| render(row, window, cx))
                                    .unwrap_or(div().into_any_element()),
                            ),
                    )
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _, _, cx| {
                            let selection = this.selection_mut();
                            selection.clear_rows();
                            selection.start_row_selection(row_ix, col_ix);
                            cx.emit(stateful::event::SelectionChanged);
                            cx.notify();
                        }),
                    )
            })
            .collect::<Vec<_>>();

        h_flex()
            .id(format!("row-{}", row_ix))
            .w_full()
            .min_h(self.row_height())
            .max_h(self.row_height())
            .when(!row_ix.is_multiple_of(2), |e| e.bg(cx.theme().bg_table_odd))
            .children(cells)
            .on_mouse_move(cx.listener(move |this, event: &MouseMoveEvent, _, cx| {
                if event.dragging() {
                    this.selection_mut().update_row_selection(row_ix);
                    cx.emit(stateful::event::SelectionChanged);
                    cx.notify();
                }
            }))
            .capture_any_mouse_up(cx.listener(move |this, _, _, cx| {
                this.selection_mut().commit_row_selection();
                cx.emit(stateful::event::SelectionChanged);
                cx.notify();
            }))
    }

    fn render_footer(&self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .emphasis(Emphasis::Secondary, cx)
            .w_full()
            .min_h(self.row_height())
            .max_h(self.row_height())
            .px_1()
            .border_t_1()
            .border_color(cx.theme().border_secondary)
            .child(
                div()
                    .text_color(cx.theme().fg_secondary)
                    .child(format!("{} rows", self.rows.read(cx).len())),
            )
    }
}

impl<Row: 'static> Focusable for Table<Row> {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl<Row: 'static> FocusableComponent for Table<Row> {
    fn set_focus_handle(&mut self, focus_handle: FocusHandle, _cx: &mut App) {
        self.focus_handle = focus_handle;
    }
}

impl<Row: 'static> Identifiable for Table<Row> {
    fn id(&self, _cx: &App) -> &ElementId {
        &self.id
    }
}

impl<Row: 'static> Render for Table<Row> {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .id(self.id.clone())
            .track_focus(&self.focus_handle)
            .focus_ring(&self.focus_handle, window, cx)
            .key_context(action::KEY_CONTEXT)
            .size_full()
            .child(self.render_header(window, cx))
            .child(self.render_body(window, cx))
            .child(self.render_footer(window, cx))
            .on_action::<crate::root::action::SelectionAll>(cx.listener(move |this, _, _, cx| {
                this.select_all(cx);
                cx.notify();
            }))
            .on_action::<crate::root::action::SelectionClear>(cx.listener(move |this, _, _, cx| {
                this.selection_mut().clear_rows();
                cx.notify();
            }))
            .on_action::<crate::root::action::Delete>(cx.listener(move |this, _, window, cx| {
                this.delete_selection(window, cx);
                cx.notify();
            }))
            .on_action::<crate::root::action::Edit>(cx.listener(move |this, _, window, cx| {
                this.edit_selection(window, cx);
                cx.notify();
            }))
            .on_action::<action::PrevRow>(cx.listener(move |this, _, _, cx| {
                this.select_prev_row(false, cx);
                cx.notify();
            }))
            .on_action::<action::NextRow>(cx.listener(move |this, _, _, cx| {
                this.select_next_row(false, cx);
                cx.notify();
            }))
            .on_action::<action::ExtendSelectionPrev>(cx.listener(move |this, _, _, cx| {
                this.select_prev_row(true, cx);
                cx.notify();
            }))
            .on_action::<action::ExtendSelectionNext>(cx.listener(move |this, _, _, cx| {
                this.select_next_row(true, cx);
                cx.notify();
            }))
            .on_action::<action::PrevColumn>(cx.listener(move |this, _, _, cx| {
                this.select_prev_column(cx);
                cx.notify();
            }))
            .on_action::<action::NextColumn>(cx.listener(move |this, _, _, cx| {
                this.select_next_column(cx);
                cx.notify();
            }))
    }
}

impl<Row: 'static> EventEmitter<stateful::event::SelectionChanged> for Table<Row> {}

pub struct TableColumn<Row> {
    label: SharedString,
    // FIXME: It would be nice if we can make this` &mut App` instead of `&App`.
    render: Option<Box<dyn Fn(&Row, &mut Window, &App) -> AnyElement>>,
    on_edit: Option<
        Box<
            dyn Fn(&[usize], &Entity<Vec<Row>>, FocusHandle, &mut Window, &mut Context<Table<Row>>)
                + 'static,
        >,
    >,
}

impl<Row: 'static> TableColumn<Row> {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self { label: label.into(), render: None, on_edit: None }
    }

    pub fn label(&self) -> &SharedString {
        &self.label
    }

    pub fn set_label(&mut self, label: impl Into<SharedString>) {
        self.label = label.into();
    }

    pub fn with_label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }

    pub fn set_element<F>(&mut self, render: F)
    where
        F: Fn(&Row, &mut Window, &App) -> AnyElement + 'static,
    {
        self.render = Some(Box::new(render));
    }

    pub fn with_element<F>(mut self, render: F) -> Self
    where
        F: Fn(&Row, &mut Window, &App) -> AnyElement + 'static,
    {
        self.render = Some(Box::new(render));
        self
    }

    pub fn set_on_edit<F>(&mut self, on_edit: F)
    where
        F: Fn(&[usize], &Entity<Vec<Row>>, FocusHandle, &mut Window, &mut Context<Table<Row>>)
            + 'static,
    {
        self.on_edit = Some(Box::new(on_edit));
    }

    pub fn with_on_edit<F>(mut self, on_edit: F) -> Self
    where
        F: Fn(&[usize], &Entity<Vec<Row>>, FocusHandle, &mut Window, &mut Context<Table<Row>>)
            + 'static,
    {
        self.on_edit = Some(Box::new(on_edit));
        self
    }

    pub fn with_editor<V, Input, CreateField, Apply>(
        mut self,
        popup_title: impl Into<String>,
        create_field: CreateField,
        apply: Apply,
    ) -> Self
    where
        V: Clone + 'static,
        Input: Render
            + Focusable
            + EventEmitter<stateful::event::Submit<Option<V>>>
            + EventEmitter<stateful::event::Change<Option<V>>>
            + 'static,
        CreateField: Fn(&mut Window, &mut Context<Table<Row>>) -> Entity<Input> + 'static,
        Apply: Fn(&mut Row, &V, usize) + 'static,
    {
        let popup_title = popup_title.into();

        let apply = Rc::new(apply);

        self.on_edit = Some(Box::new(move |row_ixs, rows, table_focus_handle, window, cx| {
            let row_ixs = row_ixs.to_vec();
            let rows = rows.clone();
            let popup_title = popup_title.clone();

            let field = create_field(window, cx);
            let apply = Rc::clone(&apply);

            let popup = InputPopup::new(field, window, cx).with_on_submit(
                window,
                cx,
                move |value, window, cx| {
                    let Some(value) = value else { return };

                    rows.update(cx, |rows, cx| {
                        for (i, row_ix) in row_ixs.iter().enumerate() {
                            if let Some(row) = rows.get_mut(*row_ix) {
                                apply(row, value, i);
                            }
                        }
                        cx.notify();
                    });

                    cx.dismiss_popup(window);
                },
            );
            let popup = cx.new(move |_| popup);

            cx.set_popup(&popup_title, popup, Some(table_focus_handle));
        }));

        self
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TableSelection {
    rows: Vec<usize>,
    column: usize,
    mode: TableSelectionMode,
    anchor_row: Option<usize>,
    active_row: Option<usize>,
    partial_selection: Option<TablePartialSelection>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TablePartialSelection {
    first_row: usize,
    last_row: usize,
}

impl TableSelection {
    pub fn new(mode: TableSelectionMode) -> Self {
        Self {
            rows: Vec::new(),
            column: 0,
            mode,
            anchor_row: None,
            active_row: None,
            partial_selection: None,
        }
    }

    pub fn rows(&self) -> &[usize] {
        &self.rows
    }

    pub fn set_rows(&mut self, row_ixs: Vec<usize>) {
        self.rows = self.normalize_rows(row_ixs);
        self.anchor_row = self.rows.last().copied();
        self.active_row = self.rows.last().copied();
    }

    pub fn with_rows(mut self, row_ixs: Vec<usize>) -> Self {
        self.set_rows(row_ixs);
        self
    }

    pub fn add_row(&mut self, row_ix: usize) {
        match self.mode {
            TableSelectionMode::Single => self.rows = vec![row_ix],
            TableSelectionMode::Multiple => {
                if !self.rows.contains(&row_ix) {
                    self.rows.push(row_ix);
                }
            }
        }

        self.anchor_row = Some(row_ix);
        self.active_row = Some(row_ix);
    }

    pub fn add_rows(&mut self, row_ixs: Vec<usize>) {
        match self.mode {
            TableSelectionMode::Single => self.rows = self.normalize_rows(row_ixs),
            TableSelectionMode::Multiple => {
                for row_ix in row_ixs {
                    self.add_row(row_ix);
                }
            }
        }
    }

    pub fn remove_row(&mut self, row_ix: usize) {
        self.rows.retain(|&ix| ix != row_ix);
        if self.anchor_row == Some(row_ix) {
            self.anchor_row = self.rows.last().copied();
        }
        if self.active_row == Some(row_ix) {
            self.active_row = self.rows.last().copied();
        }
    }

    pub fn remove_rows(&mut self, row_ixs: Vec<usize>) {
        for row_ix in row_ixs {
            self.remove_row(row_ix);
        }
    }

    pub fn clear_rows(&mut self) {
        self.rows.clear();
        self.anchor_row = None;
        self.active_row = None;
        self.clear_partial_row_selection();
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn set_column(&mut self, column_ix: usize) {
        self.column = column_ix;
    }

    pub fn with_column(mut self, column_ix: usize) -> Self {
        self.column = column_ix;
        self
    }

    pub fn mode(&self) -> TableSelectionMode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: TableSelectionMode) {
        self.mode = mode;
        let rows = std::mem::take(&mut self.rows);
        self.rows = self.normalize_rows(rows);
        self.anchor_row = self.rows.last().copied();
        self.active_row = self.rows.last().copied();
    }

    pub fn with_mode(mut self, mode: TableSelectionMode) -> Self {
        self.set_mode(mode);
        self
    }

    pub fn select_all(&mut self, row_count: usize) {
        self.set_rows((0..row_count).collect());
    }

    pub fn start_row_selection(&mut self, row_ix: usize, column_ix: usize) {
        self.column = column_ix;
        self.anchor_row = Some(row_ix);
        self.active_row = Some(row_ix);
        self.partial_selection =
            Some(TablePartialSelection { first_row: row_ix, last_row: row_ix });
    }

    pub fn update_row_selection(&mut self, row_ix: usize) {
        if let Some(partial_selection) = &mut self.partial_selection {
            partial_selection.last_row = row_ix;
            self.active_row = Some(row_ix);
        }
    }

    pub fn commit_row_selection(&mut self) {
        if let Some(partial_selection) = self.partial_selection.take() {
            let last_row = partial_selection.last_row;
            self.add_rows(partial_selection.rows(self.mode));
            self.active_row = Some(last_row);
        }
    }

    pub fn clear_partial_row_selection(&mut self) {
        self.partial_selection = None;
    }

    pub fn selected_row(&self) -> Option<usize> {
        self.partial_selection
            .as_ref()
            .map(|partial_selection| partial_selection.last_row)
            .or(self.active_row)
            .or(self.rows.last().copied())
    }

    pub fn has_rows(&self) -> bool {
        !self.rows.is_empty()
    }

    pub fn move_row_selection(&mut self, row_ix: usize, preserve_existing: bool) {
        self.clear_partial_row_selection();

        if preserve_existing && self.mode == TableSelectionMode::Multiple {
            let anchor_row = self.anchor_row.or_else(|| self.selected_row()).unwrap_or(row_ix);
            self.rows = self.normalize_rows(
                TablePartialSelection { first_row: anchor_row, last_row: row_ix }.rows(self.mode),
            );
            self.anchor_row = Some(anchor_row);
            self.active_row = Some(row_ix);
        } else {
            self.anchor_row = Some(row_ix);
            self.active_row = Some(row_ix);
            self.set_rows(vec![row_ix]);
        }
    }

    pub fn move_column_selection(&mut self, column_ix: usize) {
        self.column = column_ix;
    }

    fn cell_selected(&self, row_ix: usize, col_ix: usize) -> bool {
        if self.column != col_ix {
            return false;
        }

        self.rows.contains(&row_ix)
            || self
                .partial_selection
                .as_ref()
                .is_some_and(|partial_selection| partial_selection.contains(self.mode, row_ix))
    }

    fn normalize_rows(&self, row_ixs: Vec<usize>) -> Vec<usize> {
        let mut rows = Vec::new();

        for row_ix in row_ixs {
            if !rows.contains(&row_ix) {
                rows.push(row_ix);
            }
        }

        match self.mode {
            TableSelectionMode::Single => rows.into_iter().take(1).collect(),
            TableSelectionMode::Multiple => rows,
        }
    }
}

impl TablePartialSelection {
    fn contains(&self, mode: TableSelectionMode, row_ix: usize) -> bool {
        match mode {
            TableSelectionMode::Single => row_ix == self.last_row,
            TableSelectionMode::Multiple => {
                let (start, end) = self.row_range();
                row_ix >= start && row_ix <= end
            }
        }
    }

    fn rows(&self, mode: TableSelectionMode) -> Vec<usize> {
        match mode {
            TableSelectionMode::Single => vec![self.last_row],
            TableSelectionMode::Multiple => {
                let (start, end) = self.row_range();
                (start..=end).collect()
            }
        }
    }

    fn row_range(&self) -> (usize, usize) {
        if self.first_row <= self.last_row {
            (self.first_row, self.last_row)
        } else {
            (self.last_row, self.first_row)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TableSelectionMode {
    Single,
    #[default]
    Multiple,
}
