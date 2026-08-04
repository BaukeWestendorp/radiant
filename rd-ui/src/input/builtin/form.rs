use gpui::{
    AnyElement, App, Entity, FocusHandle, Focusable, IntoElement, RenderOnce, SharedString, Window,
    prelude::*,
};

use crate::{ActiveTheme, Button, Input, InputDelegate, InputEvent, InputState, h_flex, v_flex};

pub struct FormField {
    label: Option<SharedString>,
    input: AnyElement,
    direction: LayoutDirection,
}

impl FormField {
    pub fn new<D: InputDelegate>(input: Input<D>, cx: &mut App) -> Self {
        input.state.update(cx, |input, _| {
            input.set_is_root_input(false);
        });

        let direction = input
            .state
            .read(cx)
            .delegate()
            .form_layout_direction()
            .unwrap_or(LayoutDirection::Vertical);

        Self { label: None, input: input.into_any_element(), direction }
    }

    pub fn with_label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn with_direction(mut self, direction: LayoutDirection) -> Self {
        self.direction = direction;
        self
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LayoutDirection {
    #[default]
    Vertical,
    Horizontal,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FormFocusBehavior {
    #[default]
    FirstField,
    Form,
}

pub trait FormDelegate {
    type Data;

    fn fields(&self, cx: &mut App) -> Vec<FormField>;

    fn extract_data(&self, cx: &App) -> Option<Self::Data>;

    fn preferred_focus_handle(&self, _cx: &App) -> Option<FocusHandle> {
        None
    }
}

pub struct Form<D: FormDelegate> {
    delegate: D,
    focus_handle: FocusHandle,
    focus_behavior: FormFocusBehavior,
}

impl<D: FormDelegate + 'static> Form<D> {
    pub fn new(
        delegate: D,
        focus_handle: FocusHandle,
        _window: &mut Window,
        _cx: &mut Context<InputState<Self>>,
    ) -> Self {
        Self { delegate, focus_handle, focus_behavior: FormFocusBehavior::default() }
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
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        match self.focus_behavior {
            FormFocusBehavior::FirstField => self
                .delegate
                .preferred_focus_handle(cx)
                .unwrap_or_else(|| self.focus_handle.clone()),
            FormFocusBehavior::Form => self.focus_handle.clone(),
        }
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
                .map(|field| match field.label {
                    Some(label) => {
                        Labelled::new(label, field.input, field.direction).into_any_element()
                    }
                    None => h_flex().w_full().child(field.input).into_any_element(),
                })
                .collect::<Vec<_>>()
        });

        let has_submit_button = self.state.read(cx).is_root_input();
        let submit_button = Button::new("submit", cx.focus_handle()).label("Submit").on_click({
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

#[derive(IntoElement)]
pub struct Labelled {
    label: SharedString,
    content: AnyElement,
    direction: LayoutDirection,
}

impl Labelled {
    pub fn new(
        label: impl Into<SharedString>,
        content: impl Into<AnyElement>,
        direction: LayoutDirection,
    ) -> Self {
        Self { label: label.into(), content: content.into(), direction }
    }
}

impl RenderOnce for Labelled {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let label =
            h_flex().w_full().text_sm().text_color(cx.theme().fg_secondary).child(self.label);
        let content = h_flex().w_full().child(self.content);

        match self.direction {
            LayoutDirection::Vertical => v_flex().w_full().gap_1().child(label).child(content),
            LayoutDirection::Horizontal => h_flex().w_full().gap_2().child(label).child(content),
        }
    }
}
