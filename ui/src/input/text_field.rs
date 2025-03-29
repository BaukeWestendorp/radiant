use super::{TextInput, TextInputEvent};
use crate::{Disableable, InteractiveContainer};
use gpui::*;

pub struct TextField {
    input: Entity<TextInput>,
}

impl TextField {
    pub fn new(id: impl Into<ElementId>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input = cx.new(|cx| TextInput::new(id, window, cx).px(window.rem_size() * 0.25));

        cx.subscribe(&input, |_number_field, _input, event, cx| {
            cx.emit(event.clone());
            cx.notify();
        })
        .detach();

        Self { input }
    }

    pub fn placeholder<'a>(&self, cx: &'a App) -> &'a SharedString {
        self.input.read(cx).placeholder()
    }

    pub fn set_placeholder(&self, placeholder: SharedString, cx: &mut App) {
        self.input.update(cx, |input, cx| {
            input.set_placeholder(placeholder, cx);
        })
    }

    pub fn disabled(&self, cx: &App) -> bool {
        self.input.read(cx).disabled()
    }

    pub fn set_disabled(&self, disabled: bool, cx: &mut App) {
        self.input.update(cx, |text_field, _cx| text_field.set_disabled(disabled));
    }

    pub fn masked(&self, cx: &App) -> bool {
        self.input.read(cx).masked()
    }

    pub fn set_masked(&self, masked: bool, cx: &mut App) {
        self.input.update(cx, |text_field, _cx| text_field.set_masked(masked));
    }

    pub fn value<'a>(&self, cx: &'a App) -> &'a str {
        self.input.read(cx).text()
    }

    pub fn set_value(&self, value: SharedString, cx: &mut App) {
        self.input.update(cx, |text_field, cx| {
            text_field.set_text(value, cx);
        })
    }
}

impl Render for TextField {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let focus_handle = self.input.read(cx).focus_handle(cx);

        InteractiveContainer::new(ElementId::View(cx.entity_id()), focus_handle)
            .disabled(self.disabled(cx))
            .cursor_text()
            .child(self.input.clone())
    }
}

impl EventEmitter<TextInputEvent> for TextField {}
