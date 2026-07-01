use gpui::{
    App, ElementId, Entity, EventEmitter, FocusHandle, Focusable, SharedString, Window, div,
    prelude::*,
};

use crate::{
    input::text_input::{TextInput, TextInputEvent},
    interactive_container,
};

pub trait FieldValue {
    fn from_str(s: &str) -> Option<Self>
    where
        Self: Sized;

    fn to_shared_string(&self) -> impl Into<SharedString>;

    fn validator(s: &str) -> bool;

    fn submit_validator(s: &str) -> bool;
}

impl FieldValue for f64 {
    fn from_str(s: &str) -> Option<Self> {
        s.parse().ok()
    }

    fn to_shared_string(&self) -> impl Into<SharedString> {
        self.to_string()
    }

    fn validator(s: &str) -> bool {
        s.parse::<f64>().is_ok()
    }

    fn submit_validator(s: &str) -> bool {
        s.parse::<f64>().is_ok()
    }
}

impl FieldValue for SharedString {
    fn from_str(s: &str) -> Option<Self> {
        Some(s.into())
    }

    fn to_shared_string(&self) -> impl Into<SharedString> {
        self.clone()
    }

    fn validator(s: &str) -> bool {
        !s.trim().is_empty()
    }

    fn submit_validator(s: &str) -> bool {
        !s.trim().is_empty()
    }
}

pub struct FieldState<T: FieldValue> {
    text_input: Entity<TextInput>,

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

#[derive(Debug, Clone)]
pub enum FieldEvent<T: FieldValue> {
    Focus,
    Blur,
    Submit(T),
    Change(T),
}

impl<T: FieldValue + 'static> EventEmitter<FieldEvent<T>> for FieldState<T> {}

#[derive(IntoElement)]
pub struct Field<T: FieldValue + 'static> {
    state: Entity<FieldState<T>>,
}

impl<T: FieldValue + 'static> Field<T> {
    pub fn new(state: Entity<FieldState<T>>) -> Self {
        Self { state }
    }
}

impl<T: FieldValue + 'static> Focusable for Field<T> {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.state.read(cx).text_input.focus_handle(cx)
    }
}

impl<T: FieldValue + 'static> RenderOnce for Field<T> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let id = ElementId::View(self.state.entity_id());
        let focus_handle = self.state.read(cx).text_input.read(cx).focus_handle(cx);
        let disabled = self.state.read(cx).disabled(cx);

        interactive_container(id, Some(focus_handle))
            .w_full()
            .disabled(disabled)
            .child(div().size_full().p_0p5().child(self.state.read(cx).text_input.clone()))
    }
}
