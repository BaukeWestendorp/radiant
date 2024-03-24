use gpui::prelude::FluentBuilder;
use gpui::{
    div, FocusHandle, FocusableView, InteractiveElement, KeyDownEvent, ParentElement, Render,
    SharedString, Styled, ViewContext, WindowContext,
};

pub struct TextInput {
    text: SharedString,
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

    fn show_placeholder(&self) -> bool {
        self.text.is_empty()
    }

    fn handle_key_down(&mut self, event: &KeyDownEvent, _cx: &mut ViewContext<Self>) {
        dbg!(&event.keystroke);
    }
}

impl FocusableView for TextInput {
    fn focus_handle(&self, _cx: &gpui::AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for TextInput {
    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl gpui::prelude::IntoElement {
        let text = match self.show_placeholder() {
            true => self.placeholder.clone(),
            false => self.text.clone(),
        };

        div()
            .key_context("TextInput")
            .on_key_down(cx.listener(Self::handle_key_down))
            .track_focus(&self.focus_handle)
            .w_full()
            .when(self.show_placeholder(), |this| {
                this.text_color(gpui::rgb(0x888888))
            })
            .child(text)
    }
}
