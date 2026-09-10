mod checkbox;
mod field;
mod form;
mod picker;
mod table;
mod tabs;
mod text_input;

pub use checkbox::*;
pub use field::*;
pub use form::*;
pub use picker::*;
pub use table::*;
pub use tabs::*;
pub use text_input::*;

use gpui::{App, Context, EventEmitter};

pub mod event {
    pub struct Submit<T>(pub T);
    pub struct Change<T>(pub super::InputValue<T>);
    pub struct SelectionChanged;
}

pub trait Submittable<T: 'static>: EventEmitter<event::Submit<T>> {
    fn can_submit(&self, cx: &App) -> bool {
        self.value(cx).is_valid()
    }

    fn value(&self, cx: &App) -> InputValue<T>;

    fn submit(&self, cx: &mut Context<Self>)
    where
        Self: Sized,
    {
        if let InputValue::Valid(value) = self.value(cx)
            && self.can_submit(cx)
        {
            cx.emit(event::Submit(value))
        }
    }
}

pub enum InputValue<T> {
    Invalid,
    Valid(T),
}

impl<T> InputValue<T> {
    pub fn is_valid(&self) -> bool {
        matches!(self, InputValue::Valid(_))
    }

    pub fn is_invalid(&self) -> bool {
        matches!(self, InputValue::Invalid)
    }

    pub fn unwrap(self) -> T {
        match self {
            InputValue::Valid(value) => value,
            InputValue::Invalid => panic!("Called unwrap on an invalid InputValue"),
        }
    }
}

impl<T> From<Option<T>> for InputValue<T> {
    fn from(option: Option<T>) -> Self {
        match option {
            Some(value) => InputValue::Valid(value),
            None => InputValue::Invalid,
        }
    }
}
