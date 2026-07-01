use gpui::{App, ElementId, Entity, FocusHandle, Focusable, SharedString, Window, prelude::*};

use crate::{
    FieldEvent,
    input::{
        field::FieldValue,
        text_input::{TextInput, TextInputEvent},
    },
};

pub struct FieldState<T: FieldValue> {
    pub(crate) text_input: Entity<TextInput>,

    _marker: std::marker::PhantomData<T>,
}

impl<T: FieldValue + 'static> Focusable for FieldState<T> {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.text_input.focus_handle(cx)
    }
}

impl<T: FieldValue + 'static> FieldState<T> {
    pub fn new(
        id: impl Into<ElementId>,
        focus_handle: FocusHandle,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let text_input = cx.new(|cx| TextInput::new(id, focus_handle.tab_stop(true), window, cx));

        text_input.update(cx, |text_input, cx| {
            text_input.set_validator(T::validator);
            text_input.set_submit_validator(T::submit_validator);
            cx.notify();
        });

        cx.subscribe(&text_input, |this, _, event, cx| {
            cx.notify();
            match event {
                TextInputEvent::Focus => cx.emit(FieldEvent::Focus),
                TextInputEvent::Blur => {
                    this.commit_value(cx);
                    cx.emit(FieldEvent::Blur);
                }
                TextInputEvent::Submit(s) => {
                    if let Some(v) = FieldValue::from_str(s) {
                        cx.emit(FieldEvent::Submit(v))
                    }
                }
                TextInputEvent::Change(s) => {
                    if let Some(v) = FieldValue::from_str(s) {
                        cx.emit(FieldEvent::Change(v))
                    }
                }
            }
        })
        .detach();

        Self { text_input, _marker: std::marker::PhantomData }
    }

    pub fn value<'a>(&self, cx: &'a App) -> Option<T> {
        let s = self.text_input.read(cx).text();
        T::from_str(s)
    }

    pub fn set_value(&self, value: T, cx: &mut App) {
        self.text_input.update(cx, |this, cx| {
            this.set_text(value.to_shared_string().into(), cx);
            this.move_to_end_of_line(cx);
        })
    }

    pub fn with_value(self, value: T, cx: &mut App) -> Self {
        self.set_value(value.into(), cx);
        self
    }

    fn commit_value(&self, cx: &mut App) {
        if let Some(v) = self.value(cx) {
            self.set_value(v, cx);
        }
    }

    pub fn placeholder<'a>(&self, cx: &'a App) -> &'a SharedString {
        self.text_input.read(cx).placeholder()
    }

    pub fn set_placeholder(&self, placeholder: impl Into<SharedString>, cx: &mut App) {
        self.text_input.update(cx, |input, cx| {
            input.set_placeholder(placeholder.into(), cx);
        })
    }

    pub fn with_placeholder(self, placeholder: impl Into<SharedString>, cx: &mut App) -> Self {
        self.set_placeholder(placeholder, cx);
        self
    }

    pub fn disabled(&self, cx: &App) -> bool {
        self.text_input.read(cx).disabled()
    }

    pub fn set_disabled(&self, disabled: bool, cx: &mut App) {
        self.text_input.update(cx, |text_input, _cx| text_input.set_disabled(disabled));
    }

    pub fn with_disabled(self, disabled: bool, cx: &mut App) -> Self {
        self.set_disabled(disabled, cx);
        self
    }

    pub fn masked(&self, cx: &App) -> bool {
        self.text_input.read(cx).masked()
    }

    pub fn set_masked(&self, masked: bool, cx: &mut App) {
        self.text_input.update(cx, |text_input, _cx| text_input.set_masked(masked));
    }

    pub fn with_masked(self, masked: bool, cx: &mut App) -> Self {
        self.set_masked(masked, cx);
        self
    }

    pub fn set_validator<F: Fn(&str) -> bool + 'static>(&self, cx: &mut App, validator: F) {
        self.text_input.update(cx, |text_field, _cx| text_field.set_validator(validator));
    }

    pub fn with_validator<F: Fn(&str) -> bool + 'static>(self, cx: &mut App, validator: F) -> Self {
        self.set_validator(cx, validator);
        self
    }

    pub fn set_submit_validator<F: Fn(&str) -> bool + 'static>(&self, cx: &mut App, validator: F) {
        self.text_input.update(cx, |text_field, _cx| text_field.set_submit_validator(validator));
    }

    pub fn with_submit_validator<F: Fn(&str) -> bool + 'static>(
        self,
        cx: &mut App,
        validator: F,
    ) -> Self {
        self.set_submit_validator(cx, validator);
        self
    }
}
