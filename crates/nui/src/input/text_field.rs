use gpui::prelude::*;
use gpui::{
    App, Context, ElementId, Entity, EventEmitter, FocusHandle, Focusable, SharedString, Window,
    div,
};

use crate::container::interactive_container;
use crate::input::{TextInput, TextInputEvent};

pub struct TextField {
    input: Entity<TextInput>,
}

impl TextField {
    pub fn new(
        id: impl Into<ElementId>,
        focus_handle: FocusHandle,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let input = cx.new(|cx| TextInput::new(id, focus_handle, window, cx));

        cx.subscribe(&input, |this, _, event, cx| {
            cx.notify();
            match event {
                TextInputEvent::Focus => cx.emit(FieldEvent::Focus),
                TextInputEvent::Blur => {
                    this.commit_value(cx);
                    cx.emit(FieldEvent::Blur);
                }
                TextInputEvent::Submit(_) => cx.emit(FieldEvent::Submit),
                TextInputEvent::Change(_) => cx.emit(FieldEvent::Change),
            }
        })
        .detach();

        Self { input }
    }

    pub fn input(&self) -> &Entity<TextInput> {
        &self.input
    }

    pub fn value<'a>(&self, cx: &'a App) -> &'a SharedString {
        self.input.read(cx).text()
    }

    pub fn set_value(&self, value: SharedString, cx: &mut App) {
        self.input.update(cx, |this, cx| {
            this.set_text(value, cx);
            this.move_to_end_of_line(cx);
        })
    }

    pub fn with_value(self, value: SharedString, cx: &mut App) -> Self {
        self.set_value(value, cx);
        self
    }

    fn commit_value(&self, cx: &mut App) {
        self.set_value(self.value(cx).clone(), cx);
    }

    pub fn placeholder<'a>(&self, cx: &'a App) -> &'a SharedString {
        self.input.read(cx).placeholder()
    }

    pub fn set_placeholder(&self, placeholder: impl Into<SharedString>, cx: &mut App) {
        self.input.update(cx, |input, cx| {
            input.set_placeholder(placeholder.into(), cx);
        })
    }

    pub fn with_placeholder(self, placeholder: impl Into<SharedString>, cx: &mut App) -> Self {
        self.set_placeholder(placeholder, cx);
        self
    }

    pub fn disabled(&self, cx: &App) -> bool {
        self.input.read(cx).disabled()
    }

    pub fn set_disabled(&self, disabled: bool, cx: &mut App) {
        self.input.update(cx, |text_field, _cx| text_field.set_disabled(disabled));
    }

    pub fn with_disabled(self, disabled: bool, cx: &mut App) -> Self {
        self.set_disabled(disabled, cx);
        self
    }

    pub fn masked(&self, cx: &App) -> bool {
        self.input.read(cx).masked()
    }

    pub fn set_masked(&self, masked: bool, cx: &mut App) {
        self.input.update(cx, |text_field, _cx| text_field.set_masked(masked));
    }

    pub fn with_masked(self, masked: bool, cx: &mut App) -> Self {
        self.set_masked(masked, cx);
        self
    }

    pub fn set_validator<F: Fn(&SharedString) -> bool + 'static>(
        &self,
        cx: &mut App,
        validator: F,
    ) {
        self.input.update(cx, |text_field, _cx| text_field.set_validator(validator));
    }

    pub fn with_validator<F: Fn(&SharedString) -> bool + 'static>(
        self,
        cx: &mut App,
        validator: F,
    ) -> Self {
        self.set_validator(cx, validator);
        self
    }

    pub fn set_submit_validator<F: Fn(&SharedString) -> bool + 'static>(
        &self,
        cx: &mut App,
        validator: F,
    ) {
        self.input.update(cx, |text_field, _cx| text_field.set_submit_validator(validator));
    }

    pub fn with_submit_validator<F: Fn(&SharedString) -> bool + 'static>(
        self,
        cx: &mut App,
        validator: F,
    ) -> Self {
        self.set_submit_validator(cx, validator);
        self
    }
}

impl Focusable for TextField {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.input.focus_handle(cx)
    }
}

impl Render for TextField {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let focus_handle = self.input.read(cx).focus_handle(cx);

        interactive_container(ElementId::View(cx.entity_id()), Some(focus_handle))
            .w_full()
            .disabled(self.disabled(cx))
            .child(div().size_full().p_0p5().child(self.input.clone()))
    }
}

impl EventEmitter<FieldEvent> for TextField {}

pub enum FieldEvent {
    Focus,
    Blur,
    Submit,
    Change,
}
