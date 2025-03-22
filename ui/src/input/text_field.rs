use gpui::*;

use crate::theme::ActiveTheme;

use super::TextInput;

pub struct TextField {
    input: Entity<TextInput>,
}

impl TextField {
    pub fn new(id: impl Into<ElementId>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self { input: cx.new(|cx| TextInput::new(id, window, cx).p(window.rem_size() * 0.25)) }
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
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let focused = self.input.read(cx).is_focused(window);
        let focus_handle = self.input.read(cx).focus_handle(cx);

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
            .id("text_field")
            .track_focus(&focus_handle)
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
