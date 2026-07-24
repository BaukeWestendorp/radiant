use gpui::{
    AnyElement, App, Entity, FocusHandle, Focusable, IntoElement, RenderOnce, SharedString, Window,
    div, prelude::*, px,
};

use crate::{Button, InputDelegate, InputEvent, InputState, h_flex, v_flex};

pub struct FormField {
    pub label: SharedString,
    pub input: AnyElement,
}

impl FormField {
    pub fn new(label: impl Into<SharedString>, input: impl IntoElement) -> Self {
        Self { label: label.into(), input: input.into_any_element() }
    }
}

pub trait FormDelegate {
    type Data;

    fn fields(&self, cx: &App) -> Vec<FormField>;

    fn extract_data(&self, cx: &App) -> Option<Self::Data>;
}

pub struct Form<D: FormDelegate> {
    delegate: D,

    focus_handle: FocusHandle,
}

impl<D: FormDelegate + 'static> Form<D> {
    pub fn new(
        delegate: D,
        focus_handle: FocusHandle,
        _window: &mut Window,
        _cx: &mut Context<InputState<Self>>,
    ) -> Self {
        Self { delegate, focus_handle }
    }

    pub fn delegate(&self) -> &D {
        &self.delegate
    }

    pub fn submit(&mut self, cx: &mut Context<InputState<Self>>) {
        if let Some(data) = self.delegate.extract_data(cx) {
            cx.emit(InputEvent::Submit(data));
        }
    }
}

impl<D: FormDelegate + 'static> InputDelegate for Form<D> {
    type Value = D::Data;

    fn new_element(
        state: Entity<InputState<Self>>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> impl IntoElement {
        FormElement { state }
    }

    fn value_or_default(&self, cx: &App) -> Self::Value
    where
        Self::Value: Default,
    {
        self.delegate.extract_data(cx).unwrap_or_default()
    }
}

impl<D: FormDelegate + 'static> Focusable for Form<D> {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

#[derive(IntoElement)]
struct FormElement<D: FormDelegate + 'static> {
    state: Entity<InputState<Form<D>>>,
}

impl<D: FormDelegate + 'static> RenderOnce for FormElement<D> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let fields =
            self.state.read(cx).delegate().delegate().fields(cx).into_iter().map(|field| {
                h_flex()
                    .w_full()
                    .gap_4()
                    .items_center()
                    .child(div().w(px(120.0)).child(field.label))
                    .child(div().flex_1().child(field.input))
            });

        let submit_button = Button::new("submit").child("Submit").on_click({
            let state = self.state.clone();
            move |_, _, cx| {
                state.update(cx, |state, cx| state.submit(cx));
            }
        });

        v_flex().gap_4().size_full().children(fields).child(submit_button)
    }
}
