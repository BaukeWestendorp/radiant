use gpui::{
    App, ElementId, Entity, FocusHandle, Focusable, RenderOnce, SharedString, Window, div,
    prelude::*,
};

use crate::{
    InputDelegate, InputEvent, InputState, input::builtin::text_input::TextInput,
    interactive_container,
};

mod value;

pub use value::*;

pub struct Field<V: FieldValue> {
    text_input: Entity<TextInput>,

    _marker: std::marker::PhantomData<V>,
}

impl<V: FieldValue + 'static> Field<V> {
    pub fn new(
        focus_handle: FocusHandle,
        window: &mut Window,
        cx: &mut Context<InputState<Self>>,
    ) -> Self {
        let id = ElementId::View(cx.entity_id());

        let text_input = cx.new(move |cx| {
            let mut text_input = TextInput::new(id, focus_handle, window, cx);
            text_input.set_validator(V::validator);
            text_input.set_submit_validator(V::submit_validator);
            text_input
        });

        cx.subscribe(&text_input, |this, _, event, cx| {
            match event {
                InputEvent::Focus => cx.emit(InputEvent::Focus),
                InputEvent::Blur => {
                    this.commit_value(cx);
                    cx.emit(InputEvent::Blur);
                }
                InputEvent::Submit(s) => {
                    if let Some(v) = FieldValue::from_str(s) {
                        cx.emit(InputEvent::Submit(v))
                    }
                }
                InputEvent::Change(s) => {
                    if let Some(v) = FieldValue::from_str(s) {
                        cx.emit(InputEvent::Change(v))
                    }
                }
            }
            cx.notify();
        })
        .detach();

        Self { text_input, _marker: std::marker::PhantomData }
    }

    pub fn value<'a>(&self, cx: &'a App) -> Option<V> {
        let s = self.text_input.read(cx).text();
        V::from_str(s)
    }

    pub fn set_value(&self, value: V, cx: &mut Context<InputState<Self>>) {
        self.text_input.update(cx, |this, cx| {
            this.set_text(value.to_shared_string().into(), cx);
            this.move_to_end_of_line(cx);
        });
    }

    pub fn with_value(self, value: V, cx: &mut Context<InputState<Self>>) -> Self {
        self.set_value(value.into(), cx);
        self
    }

    fn commit_value(&self, cx: &mut Context<InputState<Self>>) {
        if let Some(v) = self.value(cx) {
            self.set_value(v, cx);
        }
    }

    pub fn placeholder<'a>(&self, cx: &'a App) -> &'a SharedString {
        self.text_input.read(cx).placeholder()
    }

    pub fn set_placeholder(
        &self,
        placeholder: impl Into<SharedString>,
        cx: &mut Context<InputState<Self>>,
    ) {
        self.text_input.update(cx, |input, cx| {
            input.set_placeholder(placeholder.into(), cx);
        })
    }

    pub fn with_placeholder(
        self,
        placeholder: impl Into<SharedString>,
        cx: &mut Context<InputState<Self>>,
    ) -> Self {
        self.set_placeholder(placeholder, cx);
        self
    }

    pub fn disabled(&self, cx: &App) -> bool {
        self.text_input.read(cx).disabled()
    }

    pub fn set_disabled(&self, disabled: bool, cx: &mut Context<InputState<Self>>) {
        self.text_input.update(cx, |text_input, _cx| text_input.set_disabled(disabled));
    }

    pub fn with_disabled(self, disabled: bool, cx: &mut Context<InputState<Self>>) -> Self {
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

impl<V: FieldValue + 'static> InputDelegate for Field<V> {
    type Value = V;

    fn new_element(
        state: Entity<InputState<Self>>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> impl IntoElement {
        FieldElement { state }
    }
}

impl<V: FieldValue + 'static> Focusable for Field<V> {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.text_input.focus_handle(cx)
    }
}

#[derive(IntoElement)]
struct FieldElement<V: FieldValue + 'static> {
    state: Entity<InputState<Field<V>>>,
}

impl<V: FieldValue + 'static> RenderOnce for FieldElement<V> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl gpui::IntoElement {
        let id = self.state.read(cx).text_input.read(cx).element_id().clone();
        let focus_handle = self.state.focus_handle(cx);
        let disabled = self.state.read(cx).disabled(cx);

        let overlay = V::render_overlay(window, cx).map(|e| e.into_any_element());

        interactive_container(id, Some(focus_handle))
            .relative()
            .w_full()
            .disabled(disabled)
            .child(div().size_full().px_1().py_0p5().child(self.state.read(cx).text_input.clone()))
            .when_some(overlay, |e, overlay| e.child(div().absolute().inset_0().child(overlay)))
    }
}
