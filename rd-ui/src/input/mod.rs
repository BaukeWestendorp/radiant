use gpui::{App, Entity, EventEmitter, FocusHandle, Focusable, Window, prelude::*};

pub(crate) mod dropdown;
pub(crate) mod field;
pub(crate) mod number_field;
pub(crate) mod text_input;

pub const INPUT_HEIGHT: gpui::Pixels = gpui::px(26.0);

pub trait InputState: Focusable + EventEmitter<InputEvent<Self::Value>> {
    type Value: Clone;
    type Element: IntoElement;

    fn new_element(this: Entity<Input<Self>>, window: &mut Window, cx: &mut App) -> Self::Element
    where
        Self: Sized;
}

#[derive(Debug)]
pub struct Input<S: InputState> {
    state: Entity<S>,
}

impl<S: InputState> Input<S> {
    pub fn new(state: Entity<S>, cx: &mut Context<Self>) -> Self {
        cx.subscribe(&state, |_, _, event, cx| {
            cx.emit(event.clone());
        })
        .detach();

        Self { state }
    }

    pub fn state(&self) -> &Entity<S> {
        &self.state
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputEvent<V> {
    Focus,
    Blur,
    Submit(V),
    Change(V),
}

impl<S: InputState + 'static> EventEmitter<InputEvent<S::Value>> for Input<S> {}

impl<S: InputState + 'static> Focusable for Input<S> {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.state.focus_handle(cx)
    }
}
