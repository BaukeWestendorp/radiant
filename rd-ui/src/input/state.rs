use gpui::{EventEmitter, Focusable, Window, prelude::*};

use crate::{InputDelegate, InputEvent};

pub struct InputState<D: InputDelegate> {
    delegate: D,

    is_root_input: bool,
}

impl<D: InputDelegate> InputState<D> {
    pub fn new(delegate: D, _window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self { delegate, is_root_input: true }
    }

    pub fn delegate(&self) -> &D {
        &self.delegate
    }

    pub fn delegate_mut(&mut self) -> &mut D {
        &mut self.delegate
    }

    pub fn is_root_input(&self) -> bool {
        self.is_root_input
    }

    pub fn set_is_root_input(&mut self, is_root_input: bool) {
        self.is_root_input = is_root_input;
    }

    pub fn with_is_root_input(mut self, is_root_input: bool) -> Self {
        self.is_root_input = is_root_input;
        self
    }
}

impl<D: InputDelegate> EventEmitter<InputEvent<D::Value>> for InputState<D> {}

impl<D: InputDelegate> Focusable for InputState<D> {
    fn focus_handle(&self, cx: &gpui::App) -> gpui::FocusHandle {
        self.delegate.focus_handle(cx)
    }
}

impl<D: InputDelegate> std::ops::Deref for InputState<D> {
    type Target = D;

    fn deref(&self) -> &Self::Target {
        &self.delegate
    }
}

impl<D: InputDelegate> std::ops::DerefMut for InputState<D> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.delegate
    }
}
