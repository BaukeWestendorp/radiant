use gpui::{
    App, ElementId, Entity, EventEmitter, FocusHandle, Focusable, SharedString, StyleRefinement,
    Window, prelude::*, px,
};

use crate::{
    ActiveTheme, Emphasis, StyledExt, StyledParentExt, StyledStatefulInteractiveElementExt,
    comp::{
        Disableable, FocusableComponent, Identifiable,
        stateful::{self, FormWidget, TextInput},
    },
    h_flex,
};

pub struct Field {
    text_input: Entity<TextInput>,
    style: StyleRefinement,
}

impl Field {
    pub fn new(id: impl Into<ElementId>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let text_input = cx.new(move |cx| {
            TextInput::new(id, window, cx).with_text_size(cx.theme().font_size - px(1.0))
        });
        cx.subscribe(&text_input, |_, _, event: &stateful::event::Submit<SharedString>, cx| {
            cx.emit(stateful::event::Submit(event.0.clone()))
        })
        .detach();
        cx.subscribe(&text_input, |_, _, event: &stateful::event::Change<SharedString>, cx| {
            cx.emit(stateful::event::Change(event.0.clone()))
        })
        .detach();
        Self { text_input, style: StyleRefinement::default() }
    }

    pub fn text<'a>(&self, cx: &'a App) -> &'a SharedString {
        self.text_input.read(cx).text()
    }

    pub fn set_text(&self, text: impl Into<SharedString>, cx: &mut Context<Self>) {
        self.text_input.update(cx, |this, cx| {
            this.set_text(text.into(), cx);
            this.move_to_end_of_line(cx);
        });
    }

    pub fn with_text(self, text: impl Into<SharedString>, cx: &mut Context<Self>) -> Self {
        self.set_text(text.into(), cx);
        self
    }

    pub fn clear(&self, cx: &mut Context<Self>) {
        self.text_input.update(cx, |this, cx| {
            this.set_text("", cx);
            this.move_to_end_of_line(cx);
        });
    }

    pub fn placeholder<'a>(&self, cx: &'a App) -> &'a SharedString {
        self.text_input.read(cx).placeholder()
    }

    pub fn set_placeholder(&self, placeholder: impl Into<SharedString>, cx: &mut Context<Self>) {
        self.text_input.update(cx, |input, cx| {
            input.set_placeholder(placeholder.into(), cx);
        })
    }

    pub fn with_placeholder(
        self,
        placeholder: impl Into<SharedString>,
        cx: &mut Context<Self>,
    ) -> Self {
        self.set_placeholder(placeholder, cx);
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

impl Styled for Field {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl Focusable for Field {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.text_input.read(cx).focus_handle(cx)
    }
}

impl FocusableComponent for Field {
    fn set_focus_handle(&mut self, focus_handle: FocusHandle, cx: &mut App) {
        self.text_input.update(cx, |text_input, cx| {
            text_input.set_focus_handle(focus_handle, cx);
        })
    }
}

impl Disableable for Field {
    fn disabled(&self, cx: &App) -> bool {
        self.text_input.read(cx).disabled(cx)
    }

    fn set_disabled(&mut self, disabled: bool, cx: &mut App) {
        self.text_input.update(cx, |text_input, cx| {
            text_input.set_disabled(disabled, cx);
            cx.notify();
        })
    }
}

impl Identifiable for Field {
    fn id<'a>(&'a self, cx: &'a App) -> &'a ElementId {
        self.text_input.read(cx).id(cx)
    }
}

impl Render for Field {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .id(self.id(cx).clone())
            .focus_ring(&self.focus_handle(cx), window, cx)
            .h(crate::comp::INPUT_SIZE)
            .px_1p5()
            .py_0p5()
            .min_w(crate::comp::INPUT_SIZE * 2.0)
            .w(crate::comp::INPUT_SIZE * 6.0)
            .when(!self.disabled(cx), |e| e.interactive_emphasis_bordered(Emphasis::Secondary, cx))
            .when(self.disabled(cx), |e| e.emphasis_bordered(Emphasis::Secondary, cx))
            .refine_style(&self.style)
            .child(self.text_input.clone())
    }
}

impl EventEmitter<stateful::event::Submit<SharedString>> for Field {}
impl EventEmitter<stateful::event::Change<SharedString>> for Field {}

impl FormWidget<SharedString> for Field {
    fn get_value(&self, cx: &App) -> SharedString {
        self.text(cx).clone()
    }

    fn set_value(&mut self, value: SharedString, cx: &mut Context<Self>) {
        self.set_text(value, cx);
    }
}
