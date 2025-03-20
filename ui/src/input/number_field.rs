use gpui::*;

use super::{TextField, TextFieldEvent};

pub struct NumberField {
    text_field: Entity<TextField>,
}

impl NumberField {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self { text_field: cx.new(|cx| TextField::new(window, cx)) }
    }

    pub fn disabled(&self, cx: &App) -> bool {
        self.text_field.read(cx).disabled()
    }

    pub fn set_disabled(&self, disabled: bool, cx: &mut App) {
        self.text_field.update(cx, |text_field, cx| text_field.set_disabled(disabled, cx));
    }

    pub fn value(&self, cx: &App) -> f64 {
        let value_str = self.text_field.read(cx).text();
        value_str.parse().expect("should always be able to parse value string")
    }

    pub fn set_value(&self, value: f64, cx: &mut App) {
        self.text_field.update(cx, |text_field, cx| {
            let value_str = value.to_string().into();
            text_field.set_text(value_str, cx);
        })
    }
}

impl Render for NumberField {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.text_field.clone()
    }
}

impl EventEmitter<TextFieldEvent> for NumberField {}
