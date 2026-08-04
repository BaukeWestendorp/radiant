use std::rc::Rc;

use gpui::{AnyElement, App, AppContext, Entity, FocusHandle, Focusable, SharedString, Window};

use crate::{
    AutoInput, Form, FormDelegate, FormField, Input, InputDelegate, InputState, LayoutDirection,
    Picker, Popup, PopupAppExt, TableDelegate, TableState,
};

use super::TableEvent;

#[derive(Clone, Default)]
struct EnumerableEditData<V> {
    enumerate: bool,
    value: V,
}

struct EnumerableEditForm<I: InputDelegate> {
    enumerate: Option<Entity<InputState<Picker<bool>>>>,
    value: Entity<InputState<I>>,
}

impl<I> FormDelegate for EnumerableEditForm<I>
where
    I: InputDelegate + 'static,
    I::Value: Default + Clone,
{
    type Data = EnumerableEditData<I::Value>;

    fn fields(&self, cx: &mut App) -> Vec<FormField> {
        let mut fields = Vec::new();

        if let Some(enumerate) = &self.enumerate {
            fields.push(
                FormField::new(Input::new(enumerate.clone()), cx)
                    .with_label("Enumerate")
                    .with_direction(LayoutDirection::Horizontal),
            );
        }

        fields.push(FormField::new(Input::new(self.value.clone()), cx));
        fields
    }

    fn extract_data(&self, cx: &App) -> Option<Self::Data> {
        Some(EnumerableEditData {
            enumerate: self
                .enumerate
                .as_ref()
                .map(|enumerate| *enumerate.read(cx).value(cx))
                .unwrap_or(false),
            value: self.value.read(cx).delegate().value_or_default(cx).clone(),
        })
    }

    fn preferred_focus_handle(&self, cx: &App) -> Option<FocusHandle> {
        Some(Focusable::focus_handle(&self.value, cx))
    }
}

fn build_enumerable_edit_input<I, B>(
    initial_value: I::Value,
    multiple: bool,
    input_builder: B,
    window: &mut Window,
    cx: &mut App,
) -> Entity<InputState<Form<EnumerableEditForm<I>>>>
where
    I: InputDelegate + 'static,
    I::Value: Default + Clone,
    B: Fn(I::Value, &mut Window, &mut App) -> Entity<InputState<I>>,
{
    let enumerate = multiple.then(|| <bool as AutoInput>::build_input(true, window, cx));
    let value = input_builder(initial_value, window, cx);

    cx.new(move |cx| {
        let delegate = EnumerableEditForm { enumerate, value };
        let form = Form::new(delegate, cx.focus_handle(), window, cx);
        InputState::new(form, window, cx)
    })
}

pub struct Column<D: TableDelegate> {
    id: SharedString,
    name: SharedString,

    pub(crate) cell_builder: Option<Box<dyn Fn(&D::Row, &Window, &App) -> AnyElement>>,
    /// After finishing the edit, you need to emit `TableEvent::EditSubmitted`.
    pub(crate) edit_handler:
        Option<Rc<dyn Fn(Entity<TableState<D>>, Vec<D::RowId>, &mut Window, &mut App)>>,
    pub(crate) sort_handler: Option<Box<dyn Fn(&D::Row, &D::Row) -> std::cmp::Ordering>>,
}

impl<D: TableDelegate + 'static> Column<D> {
    pub fn new(id: impl Into<SharedString>, name: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            sort_handler: None,
            edit_handler: None,
            cell_builder: None,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn editable(&self) -> bool {
        self.edit_handler.is_some()
    }

    pub fn sortable(&self) -> bool {
        self.sort_handler.is_some()
    }

    pub fn with_cell_builder(
        mut self,
        cell_builder: impl Fn(&D::Row, &Window, &App) -> AnyElement + 'static,
    ) -> Self {
        self.cell_builder = Some(Box::new(cell_builder));
        self
    }

    pub fn with_edit_handler(
        mut self,
        edit_handler: impl Fn(Entity<TableState<D>>, Vec<D::RowId>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.edit_handler = Some(Rc::new(edit_handler));
        self
    }

    pub fn with_input_popup_edit_handler<
        V: 'static,
        S: Fn(&mut D::Row) -> V + 'static,
        P: Fn(V, Entity<TableState<D>>, Vec<D::RowId>, &mut Window, &mut App) -> Popup + 'static,
    >(
        self,
        initial_value: S,
        popup_builder: P,
    ) -> Self {
        let initial_value = Rc::new(initial_value);
        let popup_builder = Rc::new(popup_builder);

        self.with_edit_handler(move |table, row_ids, window, cx| {
            let Some(first_row_id) = row_ids.first() else {
                return;
            };

            let Some(first_value) = table.update(cx, {
                let initial_value = Rc::clone(&initial_value);
                move |state, cx| {
                    state.delegate().rows().update(cx, |rows, _| {
                        let row = rows.get_mut(first_row_id)?;
                        Some(initial_value(row))
                    })
                }
            }) else {
                return;
            };

            let popup_builder = Rc::clone(&popup_builder);

            cx.open_popup(window, move |window, cx| {
                popup_builder(first_value, table, row_ids, window, cx)
            });
        })
    }

    pub fn with_editor<
        I: InputDelegate + 'static,
        F: Fn(&mut D::Row) -> &mut I::Value + 'static,
        B: Fn(I::Value, &mut Window, &mut App) -> Entity<InputState<I>> + 'static,
    >(
        self,
        field_selector: F,
        input_builder: B,
    ) -> Self
    where
        I::Value: Clone + Default,
    {
        let field_selector = Rc::new(field_selector);
        let input_builder = Rc::new(input_builder);

        self.with_input_popup_edit_handler(
            {
                let field_selector = Rc::clone(&field_selector);
                move |row| field_selector(row).clone()
            },
            move |first_value, table, row_ids, window, cx| {
                let field_selector = Rc::clone(&field_selector);

                let input = input_builder(first_value, window, cx);

                Popup::input("Edit value(s)", input, window, cx, move |new_value: &I::Value, cx| {
                    table.update(cx, |state, cx| {
                        state.delegate().rows().update(cx, |rows, cx| {
                            for row_id in &row_ids {
                                if let Some(row) = rows.get_mut(row_id) {
                                    let target_field = field_selector(row);
                                    *target_field = new_value.clone();
                                }
                            }
                            cx.notify();
                        });
                        state.update_sort_cache(cx);
                        cx.notify();
                        cx.emit(TableEvent::EditSubmitted);
                    });
                })
            },
        )
    }

    pub fn with_auto_editor<V: AutoInput + Default, F: Fn(&mut D::Row) -> &mut V + 'static>(
        self,
        field_selector: F,
    ) -> Self {
        self.with_editor(field_selector, V::build_input)
    }

    pub fn with_enumerable_editor<
        I: InputDelegate + 'static,
        F: Fn(&mut D::Row) -> &mut I::Value + 'static,
        B: Fn(I::Value, &mut Window, &mut App) -> Entity<InputState<I>> + 'static,
    >(
        self,
        field_selector: F,
        input_builder: B,
    ) -> Self
    where
        I::Value: EnumerableValue + Default,
    {
        let field_selector = Rc::new(field_selector);
        let input_builder = Rc::new(input_builder);

        self.with_input_popup_edit_handler(
            {
                let field_selector = Rc::clone(&field_selector);
                move |row| field_selector(row).clone()
            },
            move |first_value, table, row_ids, window, cx| {
                let field_selector = Rc::clone(&field_selector);
                let input_builder = Rc::clone(&input_builder);

                let input = build_enumerable_edit_input(
                    first_value,
                    row_ids.len() > 1,
                    move |value, window, cx| input_builder(value, window, cx),
                    window,
                    cx,
                );

                Popup::input(
                    "Edit value(s)",
                    input,
                    window,
                    cx,
                    move |edit: &EnumerableEditData<I::Value>, cx| {
                        table.update(cx, |state, cx| {
                            state.delegate().rows().update(cx, |rows, cx| {
                                for (offset, row_id) in row_ids.iter().enumerate() {
                                    if let Some(row) = rows.get_mut(row_id) {
                                        let target_field = field_selector(row);
                                        *target_field = if edit.enumerate {
                                            edit.value.enumerated_value(offset)
                                        } else {
                                            edit.value.clone()
                                        };
                                    }
                                }
                                cx.notify();
                            });
                            state.update_sort_cache(cx);
                            cx.notify();
                            cx.emit(TableEvent::EditSubmitted);
                        });
                    },
                )
            },
        )
    }

    pub fn with_auto_enumerable_editor<
        V: AutoInput + EnumerableValue + Default,
        F: Fn(&mut D::Row) -> &mut V + 'static,
    >(
        self,
        field_selector: F,
    ) -> Self {
        self.with_enumerable_editor(field_selector, V::build_input)
    }

    pub fn with_sort_handler(
        mut self,
        sort_handler: impl Fn(&D::Row, &D::Row) -> std::cmp::Ordering + 'static,
    ) -> Self {
        self.sort_handler = Some(Box::new(sort_handler));
        self
    }
}

pub trait EnumerableValue: Clone {
    fn enumerated_value(&self, offset: usize) -> Self;
}

impl EnumerableValue for String {
    fn enumerated_value(&self, offset: usize) -> Self {
        let mut digit_start = self.len();
        for (idx, c) in self.char_indices().rev() {
            if c.is_ascii_digit() {
                digit_start = idx;
            } else {
                break;
            }
        }

        let (base_name, start_num) = if digit_start < self.len() {
            (&self[..digit_start], self[digit_start..].parse::<usize>().ok())
        } else {
            (&self[..], None)
        };

        match start_num {
            Some(num) => format!("{}{}", base_name, num + offset),
            None => self.clone(),
        }
    }
}

impl EnumerableValue for SharedString {
    fn enumerated_value(&self, offset: usize) -> Self {
        let s: String = self.to_string();
        SharedString::from(s.enumerated_value(offset))
    }
}

macro_rules! impl_enumerate_number {
    ($($t:ty),*) => {
        $(
            impl EnumerableValue for $t {
                fn enumerated_value(&self, offset: usize) -> Self {
                    (*self).saturating_add(offset as $t)
                }
            }
        )*
    };
}

impl_enumerate_number!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);

impl<T: EnumerableValue> EnumerableValue for Option<T> {
    fn enumerated_value(&self, offset: usize) -> Self {
        match self {
            Some(value) => Some(value.enumerated_value(offset)),
            None => None,
        }
    }
}
