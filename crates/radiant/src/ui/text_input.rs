use gpui::prelude::FluentBuilder;
use gpui::{
    actions, div, EventEmitter, FocusHandle, FocusableView, InteractiveElement, KeyDownEvent,
    ParentElement, Render, SharedString, Styled, ViewContext, WindowContext,
};

actions!(text_input, [Enter, Backspace, Delete]);

pub enum Event {
    Submit(String),
}

pub struct TextInput {
    text: String,
    placeholder: SharedString,

    focus_handle: FocusHandle,
}

impl TextInput {
    pub fn new(text: Option<String>, placeholder: &str, cx: &mut WindowContext) -> Self {
        Self {
            text: text.unwrap_or_default().into(),
            placeholder: placeholder.to_string().into(),
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn clear(&mut self, cx: &mut ViewContext<Self>) {
        self.text.clear();
        cx.notify();
    }

    fn show_placeholder(&self) -> bool {
        self.text.is_empty()
    }

    fn handle_key_down(&mut self, event: &KeyDownEvent, cx: &mut ViewContext<Self>) {
        match &event.keystroke.ime_key {
            Some(key) => {
                self.text.push_str(key.as_str());
                cx.notify();
            }
            None => {}
        }
    }

    fn enter(&mut self, _event: &Enter, cx: &mut ViewContext<Self>) {
        cx.stop_propagation();
        cx.prevent_default();

        cx.emit(Event::Submit(self.text.clone()));
    }

    fn backspace(&mut self, _event: &Backspace, cx: &mut ViewContext<Self>) {
        cx.stop_propagation();
        cx.prevent_default();

        // FIXME: Very ad-hoc. We should implement a cursor.
        if self.text.len() > 0 {
            self.text.pop();
            cx.notify();
        }
    }

    fn delete(&mut self, _event: &Delete, cx: &mut ViewContext<Self>) {
        cx.stop_propagation();
        cx.prevent_default();

        // FIXME: We can't acually implement this until we have a cursor.
    }
}

impl EventEmitter<Event> for TextInput {}

impl FocusableView for TextInput {
    fn focus_handle(&self, _cx: &gpui::AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for TextInput {
    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl gpui::prelude::IntoElement {
        let text = match self.show_placeholder() {
            true => self.placeholder.clone().into(),
            false => self.text.clone(),
        };

        div()
            .key_context("TextInput")
            .on_key_down(cx.listener(Self::handle_key_down))
            .on_action(cx.listener(Self::enter))
            .on_action(cx.listener(Self::backspace))
            .on_action(cx.listener(Self::delete))
            .track_focus(&self.focus_handle)
            .size_full()
            .flex()
            .items_center()
            .when(self.show_placeholder(), |this| {
                this.text_color(gpui::rgb(0x888888))
            })
            .child(text)
    }
}
