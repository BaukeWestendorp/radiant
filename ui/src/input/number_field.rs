use gpui::*;

use crate::theme::ActiveTheme;

use super::TextInput;

pub struct NumberField {
    input: Entity<TextInput>,

    focus_handle: FocusHandle,
}

impl NumberField {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            input: cx.new(|cx| TextInput::new(window, cx).p(window.rem_size() * 0.25)),
            focus_handle: cx.focus_handle().clone(),
        }
    }

    pub fn disabled(&self, cx: &App) -> bool {
        self.input.read(cx).disabled()
    }

    pub fn set_disabled(&self, disabled: bool, cx: &mut App) {
        self.input.update(cx, |text_field, cx| text_field.set_disabled(disabled, cx));
    }

    pub fn masked(&self, cx: &App) -> bool {
        self.input.read(cx).masked()
    }

    pub fn set_masked(&self, masked: bool, cx: &mut App) {
        self.input.update(cx, |text_field, _cx| text_field.set_masked(masked));
    }

    pub fn value(&self, cx: &App) -> f64 {
        let value_str = self.input.read(cx).text();
        value_str.parse().expect("should always be able to parse value string")
    }

    pub fn set_value(&self, value: f64, cx: &mut App) {
        self.input.update(cx, |text_field, cx| {
            let value_str = value.to_string().into();
            text_field.set_text(value_str, cx);
        })
    }
}

impl Render for NumberField {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let focused = self.focus_handle.is_focused(window);

        let background_color =
            if focused { cx.theme().background_focused } else { cx.theme().background };

        let border_color = if focused {
            cx.theme().border_color_focused
        } else if self.disabled(cx) {
            cx.theme().border_color_muted
        } else {
            cx.theme().border_color
        };

        let text_color =
            if self.disabled(cx) { cx.theme().text_muted } else { cx.theme().text_primary };

        div()
            .id("number_field")
            .track_focus(&self.focus_handle)
            .w_full()
            .bg(background_color)
            .text_color(text_color)
            .border_1()
            .border_color(border_color)
            .rounded(cx.theme().radius)
            .cursor_text()
            .child(self.input.clone())
    }
}
