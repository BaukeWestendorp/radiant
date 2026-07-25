use gpui::{
    AnyElement, App, Entity, FocusHandle, Focusable, IntoElement, RenderOnce, SharedString, Window,
    div, prelude::*,
};

use crate::{ActiveTheme, Button, Input, InputDelegate, InputEvent, InputState, h_flex, v_flex};

pub struct FormField {
    pub label: SharedString,
    pub input: AnyElement,
    pub direction: LayoutDirection,
}

impl FormField {
    pub fn new<D: InputDelegate>(
        label: impl Into<SharedString>,
        input: Input<D>,
        cx: &mut App,
    ) -> Self {
        input.state.update(cx, |input, _| {
            input.set_is_root_input(false);
        });

        let direction = input
            .state
            .read(cx)
            .delegate()
            .form_layout_direction()
            .unwrap_or(LayoutDirection::Vertical);

        Self { label: label.into(), input: input.into_any_element(), direction }
    }

    pub fn with_direction(mut self, direction: LayoutDirection) -> Self {
        self.direction = direction;
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LayoutDirection {
    Vertical,
    Horizontal,
}

pub trait FormDelegate {
    type Data;

    fn fields(&self, cx: &mut App) -> Vec<FormField>;

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
        let fields = self.state.update(cx, |state, cx| {
            state
                .delegate()
                .delegate()
                .fields(cx)
                .into_iter()
                .map(|field| {
                    let label =
                        div().text_sm().text_color(cx.theme().fg_secondary).child(field.label);
                    let input = div().child(field.input);

                    match field.direction {
                        LayoutDirection::Vertical => v_flex()
                            .w_full()
                            .items_center()
                            .child(
                                label.border_b_1().border_color(cx.theme().border_primary).w_full(),
                            )
                            .child(
                                input
                                    .w_full()
                                    .p_2()
                                    .bg(cx.theme().contrast.opacity(0.025))
                                    .border_b_1()
                                    .border_x_1()
                                    .border_color(cx.theme().contrast.opacity(0.05))
                                    .rounded_b(cx.theme().radius),
                            ),
                        LayoutDirection::Horizontal => {
                            h_flex().w_full().gap_2().child(label.w_full()).child(input.w_full())
                        }
                    }
                })
                .collect::<Vec<_>>()
        });

        let has_submit_button = self.state.read(cx).is_root_input();
        let submit_button = Button::new("submit").child("Submit").on_click({
            let state = self.state.clone();
            move |_, _, cx| {
                state.update(cx, |state, cx| state.submit(cx));
            }
        });

        v_flex()
            .gap_4()
            .size_full()
            .children(fields)
            .when(has_submit_button, |e| e.child(submit_button))
    }
}
