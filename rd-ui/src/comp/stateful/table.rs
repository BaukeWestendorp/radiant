use std::rc::Rc;

use gpui::{
    AnyElement, App, ElementId, Entity, EventEmitter, FocusHandle, Focusable, MouseButton, Pixels,
    SharedString, UniformListScrollHandle, Window, div, prelude::*, uniform_list,
};

use crate::{
    ActiveTheme, Emphasis, InputPopup, PopupAppExt, PopupSize, StyledExt, StyledParentExt,
    StyledStatefulInteractiveElementExt,
    comp::{
        Button, ButtonVariant, Disableable, FocusableComponent, INPUT_SIZE, IconVariant,
        Identifiable, Labelled,
        stateful::{self, Submittable},
    },
    h_flex, root, v_flex,
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
    state: Entity<TableState<Row>>,
}

impl<Row: 'static> Table<Row> {
    pub fn new(
        id: impl Into<ElementId>,
        rows: Entity<Vec<Row>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let state = cx.new(|cx| TableState::new(rows, window, cx));

        cx.subscribe(&state, |_, _, _: &stateful::event::SelectionChanged, cx| {
            cx.emit(stateful::event::SelectionChanged);
        })
        .detach();

        Self { id: id.into(), state }
    }

    pub fn rows(&self, cx: &App) -> Entity<Vec<Row>> {
        self.state.read(cx).rows.clone()
    }

    pub fn columns<'a>(&'a self, cx: &'a App) -> &'a [TableColumn<Row>] {
        &self.state.read(cx).columns
    }

    pub fn set_columns(&mut self, columns: Vec<TableColumn<Row>>, cx: &mut Context<Self>) {
        self.state.update(cx, |state, _cx| {
            state.columns = columns;
        });
    }

    pub fn with_columns(mut self, columns: Vec<TableColumn<Row>>, cx: &mut Context<Self>) -> Self {
        self.set_columns(columns, cx);
        self
    }

    pub fn selection<'a>(&'a self, cx: &'a App) -> &'a TableSelection {
        &self.state.read(cx).selection
    }

    pub fn set_selection(&mut self, selection: TableSelection, cx: &mut Context<Self>) {
        self.state.update(cx, |state, cx| {
            state.selection = selection;
            cx.emit(stateful::event::SelectionChanged);
        });
    }

    pub fn with_selection(mut self, selection: TableSelection, cx: &mut Context<Self>) -> Self {
        self.set_selection(selection, cx);
        self
    }

    pub fn clear_selection(&self, cx: &mut Context<Self>) {
        self.state.update(cx, |state, cx| {
            state.selection.clear_rows();
            cx.emit(stateful::event::SelectionChanged);
        });
    }

    pub fn selected_rows<'a>(&'a self, cx: &'a App) -> Vec<&'a Row> {
        let state = self.state.read(cx);
        let rows = state.rows.read(cx);
        state.selection.rows().iter().filter_map(|&row_ix| rows.get(row_ix)).collect()
    }

    pub fn set_on_delete<F>(&mut self, cx: &mut Context<Self>, on_delete: F)
    where
        F: Fn(&mut Self, &mut Window, &mut App) + 'static,
    {
        self.state.update(cx, |state, _cx| {
            state.on_delete = Some(Box::new(on_delete));
        });
    }

    pub fn with_on_delete<F>(mut self, cx: &mut Context<Self>, on_delete: F) -> Self
    where
        F: Fn(&mut Self, &mut Window, &mut App) + 'static,
    {
        self.set_on_delete(cx, on_delete);
        self
    }

    pub fn set_on_edit<F>(&mut self, cx: &mut Context<Self>, on_edit: F)
    where
        F: Fn(&mut Self, FocusHandle, &mut Window, &mut App) + 'static,
    {
        self.state.update(cx, |state, _cx| {
            state.on_edit = Some(Box::new(on_edit));
        });
    }

    pub fn with_on_edit<F>(mut self, cx: &mut Context<Self>, on_edit: F) -> Self
    where
        F: Fn(&mut Self, FocusHandle, &mut Window, &mut App) + 'static,
    {
        self.set_on_edit(cx, on_edit);
        self
    }

    pub fn set_on_add<F>(&mut self, cx: &mut Context<Self>, on_add: F)
    where
        F: Fn(&mut Self, &mut Window, &mut App) + 'static,
    {
        self.state.update(cx, |state, _cx| {
            state.on_add = Some(Box::new(on_add));
        });
    }

    pub fn with_on_add<F>(mut self, cx: &mut Context<Self>, on_add: F) -> Self
    where
        F: Fn(&mut Self, &mut Window, &mut App) + 'static,
    {
        self.set_on_add(cx, on_add);
        self
    }

    pub fn select_all(&mut self, cx: &mut Context<Self>) {
        self.state.update(cx, |state, cx| {
            let len = state.rows.read(cx).len();
            state.selection.select_all(len);
            cx.emit(stateful::event::SelectionChanged);
        });
    }

    pub fn select_prev_row(&mut self, preserve_existing: bool, cx: &mut Context<Self>) {
        self.state.update(cx, |state, cx| {
            let Some(last_row_ix) = state.rows.read(cx).len().checked_sub(1) else {
                return;
            };
            let row_ix = state
                .selection
                .selected_row()
                .map_or(last_row_ix, |row_ix| row_ix.saturating_sub(1));
            state.selection.move_row_selection(row_ix, preserve_existing);
            cx.emit(stateful::event::SelectionChanged);
        });
    }

    pub fn select_next_row(&mut self, preserve_existing: bool, cx: &mut Context<Self>) {
        self.state.update(cx, |state, cx| {
            let Some(last_row_ix) = state.rows.read(cx).len().checked_sub(1) else {
                return;
            };
            let row_ix =
                state.selection.selected_row().map_or(0, |row_ix| (row_ix + 1).min(last_row_ix));
            state.selection.move_row_selection(row_ix, preserve_existing);
            cx.emit(stateful::event::SelectionChanged);
        });
    }

    pub fn select_prev_column(&mut self, cx: &mut Context<Self>) {
        self.state.update(cx, |state, cx| {
            let Some(last_column_ix) = state.columns.len().checked_sub(1) else {
                return;
            };
            if !state.selection.has_rows() {
                if state.rows.read(cx).is_empty() {
                    return;
                }

                state.selection.move_row_selection(0, false);
                state.selection.move_column_selection(last_column_ix);
                return;
            }
            state.selection.move_column_selection(state.selection.column().saturating_sub(1));
            cx.emit(stateful::event::SelectionChanged);
        });
    }

    pub fn select_next_column(&mut self, cx: &mut Context<Self>) {
        self.state.update(cx, |state, cx| {
            let Some(last_column_ix) = state.columns.len().checked_sub(1) else {
                return;
            };
            if !state.selection.has_rows() {
                if state.rows.read(cx).is_empty() {
                    return;
                }

                state.selection.move_row_selection(0, false);
                state.selection.move_column_selection(0);
                return;
            }
            let column_ix = (state.selection.column() + 1).min(last_column_ix);
            state.selection.move_column_selection(column_ix);
            cx.emit(stateful::event::SelectionChanged);
        });
    }

    pub fn delete_selection(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let mut on_delete_cb = self.state.update(cx, |state, _cx| state.on_delete.take());

        if let Some(on_delete) = &on_delete_cb {
            on_delete(self, window, cx);
        }

        if let Some(cb) = on_delete_cb.take() {
            self.state.update(cx, |state, _cx| state.on_delete = Some(cb));
        }
    }

    pub fn edit_selection(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        // FIXME: It would be nice if the field could be preloaded with the (first) value that is being edited.

        let mut on_edit = self.state.update(cx, |state, _cx| state.on_edit.take());
        let (rows, column, focus_handle, rows_entity) = self.state.update(cx, |state, _cx| {
            (
                state.selection.rows().to_vec(),
                state.selection.column,
                state.focus_handle.clone(),
                state.rows.clone(),
            )
        });

        if let Some(on_edit) = &on_edit {
            on_edit(self, focus_handle.clone(), window, cx);
        }

        if let Some(cb) = on_edit.take() {
            self.state.update(cx, |state, _cx| state.on_edit = Some(cb));
        }

        let mut column_on_edit = self.state.update(cx, |state, _cx| {
            if let Some(col) = state.columns.get_mut(column) { col.on_edit.take() } else { None }
        });

        if let Some(on_edit) = &column_on_edit {
            on_edit(&rows, &rows_entity, focus_handle, window, cx);
        }

        if let Some(cb) = column_on_edit.take() {
            self.state.update(cx, |state, _cx| {
                if let Some(col) = state.columns.get_mut(column) {
                    col.on_edit = Some(cb);
                }
            });
        }
    }

    pub fn add(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let mut on_add = self.state.update(cx, |state, _cx| state.on_add.take());

        if let Some(on_add) = &on_add {
            on_add(self, window, cx);
        }

        if let Some(cb) = on_add.take() {
            self.state.update(cx, |state, _cx| state.on_add = Some(cb));
        }
    }

    #[inline(always)]
    fn row_height() -> Pixels {
        INPUT_SIZE
    }

    fn render_header(&self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let cells = self.state.read(cx).columns.iter().enumerate().map(|(ix, column)| {
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
                    this.state.update(cx, |state, _cx| state.selection.set_column(ix));
                    this.select_all(cx);
                    cx.notify();
                }))
        });

        h_flex().w_full().min_h(Self::row_height()).max_h(Self::row_height()).children(cells)
    }

    fn render_body(&self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let row_count = self.state.read(cx).rows.read(cx).len();

        uniform_list("body", row_count, {
            let state = self.state.clone();
            move |range, window, cx| {
                range
                    .map(|row_ix| Self::render_row(&state, row_ix, window, cx).into_any_element())
                    .collect()
            }
        })
        .track_scroll(&self.state.read(cx).scroll_handle)
        .bg(cx.theme().bg_table)
        .size_full()
    }

    fn render_row(
        state: &Entity<TableState<Row>>,
        row_ix: usize,
        window: &mut Window,
        cx: &mut App,
    ) -> impl IntoElement {
        let cells = state
            .read(cx)
            .columns
            .iter()
            .enumerate()
            .map(|(col_ix, column)| {
                let row = state.read(cx).rows.read(cx).get(row_ix).unwrap();
                let selected = state.read(cx).selection.cell_selected(row_ix, col_ix);

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
                    .on_mouse_down(MouseButton::Left, {
                        let state = state.clone();
                        move |_, _, cx| {
                            state.update(cx, |state, cx| {
                                state.selection.clear_rows();
                                state.selection.start_row_selection(row_ix, col_ix);
                                cx.emit(stateful::event::SelectionChanged);
                                cx.notify();
                            });
                        }
                    })
            })
            .collect::<Vec<_>>();

        h_flex()
            .id(format!("row-{}", row_ix))
            .w_full()
            .min_h(Self::row_height())
            .max_h(Self::row_height())
            .when(!row_ix.is_multiple_of(2), |e| e.bg(cx.theme().bg_table_odd))
            .children(cells)
            .on_mouse_move({
                let state = state.clone();
                move |event, _, cx| {
                    if event.dragging() {
                        state.update(cx, |state, cx| {
                            state.selection.update_row_selection(row_ix);
                            cx.emit(stateful::event::SelectionChanged);
                            cx.notify();
                        });
                    }
                }
            })
            .capture_any_mouse_up({
                let state = state.clone();
                move |_, _, cx| {
                    state.update(cx, |state, cx| {
                        state.selection.commit_row_selection();
                        cx.emit(stateful::event::SelectionChanged);
                        cx.notify();
                    });
                }
            })
    }

    fn render_footer(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .emphasis(Emphasis::Secondary, cx)
            .justify_between()
            .w_full()
            .p_1()
            .border_t_1()
            .border_color(cx.theme().border_secondary)
            .child(
                h_flex()
                    .h(crate::comp::INPUT_SIZE)
                    .text_color(cx.theme().fg_secondary)
                    .child(format!("{} rows", self.state.read(cx).rows.read(cx).len())),
            )
            .child(
                h_flex()
                    .h(crate::comp::INPUT_SIZE)
                    .gap_1()
                    .h_full()
                    .when(self.state.read(cx).on_add.is_some(), |e| {
                        e.child(
                            Button::new("add-row", window, cx)
                                .with_action(root::action::Add)
                                .with_label("Add")
                                .with_variant(ButtonVariant::Primary)
                                .with_icon(IconVariant::Plus),
                        )
                    })
                    .when(self.state.read(cx).is_editable(), |e| {
                        e.child(
                            Button::new("edit-row", window, cx)
                                .with_disabled(self.selection(cx).rows().is_empty(), cx)
                                .with_action(root::action::Edit)
                                .with_label("Edit")
                                .with_variant(ButtonVariant::Secondary)
                                .with_icon(IconVariant::SquarePen),
                        )
                    })
                    .when(self.state.read(cx).on_delete.is_some(), |e| {
                        e.child(
                            Button::new("delete-row", window, cx)
                                .with_disabled(self.selection(cx).rows().is_empty(), cx)
                                .with_action(root::action::Delete)
                                .with_label("Delete")
                                .with_variant(ButtonVariant::Danger)
                                .with_icon(IconVariant::Trash),
                        )
                    }),
            )
    }
}

impl<Row: 'static> Focusable for Table<Row> {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.state.read(cx).focus_handle.clone()
    }
}

impl<Row: 'static> FocusableComponent for Table<Row> {
    fn set_focus_handle(&mut self, focus_handle: FocusHandle, cx: &mut App) {
        self.state.update(cx, |state, cx| {
            state.focus_handle = focus_handle;
            cx.notify();
        })
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
            .track_focus(&self.state.read(cx).focus_handle)
            .focus_ring(&self.state.read(cx).focus_handle, window, cx)
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
                this.state.update(cx, |state, cx| {
                    state.selection.clear_rows();
                    cx.emit(stateful::event::SelectionChanged);
                    cx.notify();
                });
            }))
            .on_action::<crate::root::action::Delete>(cx.listener(move |this, _, window, cx| {
                if !this.selection(cx).rows().is_empty() {
                    this.delete_selection(window, cx);
                    this.clear_selection(cx);
                }
                cx.notify();
            }))
            .on_action::<crate::root::action::Edit>(cx.listener(move |this, _, window, cx| {
                if !this.selection(cx).rows().is_empty() {
                    this.edit_selection(window, cx);
                    cx.notify();
                }
            }))
            .on_action::<crate::root::action::Add>(cx.listener(move |this, _, window, cx| {
                this.add(window, cx);
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

struct TableState<Row> {
    focus_handle: FocusHandle,
    scroll_handle: UniformListScrollHandle,

    rows: Entity<Vec<Row>>,
    columns: Vec<TableColumn<Row>>,
    selection: TableSelection,

    on_delete: Option<Box<dyn Fn(&mut Table<Row>, &mut Window, &mut App) + 'static>>,
    on_edit: Option<Box<dyn Fn(&mut Table<Row>, FocusHandle, &mut Window, &mut App) + 'static>>,
    on_add: Option<Box<dyn Fn(&mut Table<Row>, &mut Window, &mut App) + 'static>>,
}

impl<Row> TableState<Row> {
    pub fn new(rows: Entity<Vec<Row>>, _window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle().tab_stop(true),
            scroll_handle: UniformListScrollHandle::new(),

            rows,
            columns: Vec::new(),
            selection: TableSelection::default(),

            on_delete: None,
            on_edit: None,
            on_add: None,
        }
    }

    fn is_editable(&self) -> bool {
        self.on_edit.is_some() || self.columns.iter().any(|column| column.on_edit.is_some())
    }
}

impl<Row: 'static> EventEmitter<stateful::event::SelectionChanged> for TableState<Row> {}

pub struct TableCellEditor<Row, V, Input, CreateField, Apply> {
    popup_title: String,
    create_field: CreateField,
    apply: Apply,
    popup_size: PopupSize,
    _marker: std::marker::PhantomData<fn() -> (Row, V, Input)>,
}

impl<Row, V, Input, CreateField, Apply> TableCellEditor<Row, V, Input, CreateField, Apply>
where
    Row: 'static,
    V: Clone + 'static,
    Input: Render + Focusable + Submittable<V> + EventEmitter<stateful::event::Change<V>> + 'static,
    CreateField: Fn(&mut Window, &mut Context<Table<Row>>) -> Entity<Input> + 'static,
    Apply: Fn(&mut Row, &V, usize, &mut App) + 'static,
{
    pub fn new(popup_title: impl Into<String>, create_field: CreateField, apply: Apply) -> Self {
        Self {
            popup_title: popup_title.into(),
            create_field,
            apply,
            popup_size: PopupSize::default(),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn set_popup_size(&mut self, size: PopupSize) {
        self.popup_size = size;
    }

    pub fn with_popup_size(mut self, size: PopupSize) -> Self {
        self.set_popup_size(size);
        self
    }

    fn build(
        self,
    ) -> Box<
        dyn Fn(&[usize], &Entity<Vec<Row>>, FocusHandle, &mut Window, &mut Context<Table<Row>>)
            + 'static,
    > {
        let popup_title = self.popup_title;
        let apply = Rc::new(self.apply);
        let create_field = self.create_field;
        let popup_size = self.popup_size;

        Box::new(move |row_ixs, rows, table_focus_handle, window, cx| {
            let row_ixs = row_ixs.to_vec();
            let rows = rows.clone();
            let popup_title = popup_title.clone();

            let field = create_field(window, cx);
            let apply = Rc::clone(&apply);

            let popup = InputPopup::new(field, window, cx).with_on_submit(
                window,
                cx,
                move |value, _, cx| {
                    rows.update(cx, |rows, cx| {
                        // FIXME: This should be sorted by visual order.
                        for (i, row_ix) in row_ixs.iter().enumerate() {
                            if let Some(row) = rows.get_mut(*row_ix) {
                                apply(row, value, i, cx);
                            }
                        }
                        cx.notify();
                    });
                },
            );

            let popup = cx.new(move |_| popup);

            cx.push_popup(&popup_title, popup, popup_size, Some(table_focus_handle));
        })
    }
}

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

    pub fn set_editor<V, Input, CreateField, Apply>(
        &mut self,
        editor: TableCellEditor<Row, V, Input, CreateField, Apply>,
    ) where
        V: Clone + 'static,
        Input: Render
            + Focusable
            + Submittable<V>
            + EventEmitter<stateful::event::Change<V>>
            + 'static,
        CreateField: Fn(&mut Window, &mut Context<Table<Row>>) -> Entity<Input> + 'static,
        Apply: Fn(&mut Row, &V, usize, &mut App) + 'static,
    {
        self.on_edit = Some(editor.build());
    }

    pub fn with_editor<V, Input, CreateField, Apply>(
        mut self,
        editor: TableCellEditor<Row, V, Input, CreateField, Apply>,
    ) -> Self
    where
        V: Clone + 'static,
        Input: Render
            + Focusable
            + Submittable<V>
            + EventEmitter<stateful::event::Change<V>>
            + 'static,
        CreateField: Fn(&mut Window, &mut Context<Table<Row>>) -> Entity<Input> + 'static,
        Apply: Fn(&mut Row, &V, usize, &mut App) + 'static,
    {
        self.set_editor(editor);
        self
    }

    pub fn editor<V, Input, CreateField, Apply>(
        mut self,
        editor: TableCellEditor<Row, V, Input, CreateField, Apply>,
    ) -> Self
    where
        V: Clone + 'static,
        Input: Render
            + Focusable
            + Submittable<V>
            + EventEmitter<stateful::event::Change<V>>
            + 'static,
        CreateField: Fn(&mut Window, &mut Context<Table<Row>>) -> Entity<Input> + 'static,
        Apply: Fn(&mut Row, &V, usize, &mut App) + 'static,
    {
        self.set_editor(editor);
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
