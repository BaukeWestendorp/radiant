use gpui::EventEmitter;

use crate::{FieldState, FieldValue};

#[derive(Debug, Clone)]
pub enum FieldEvent<T: FieldValue> {
    Focus,
    Blur,
    Submit(T),
    Change(T),
}

impl<T: FieldValue + 'static> EventEmitter<FieldEvent<T>> for FieldState<T> {}
