use gpui::{EventEmitter, Focusable, Window, prelude::*};

use crate::{InputDelegate, InputEvent};

pub struct InputState<D: InputDelegate> {
    delegate: D,
}

impl<D: InputDelegate> InputState<D> {
    pub fn new(delegate: D, _window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self { delegate }
    }

    pub fn delegate(&self) -> &D {
        &self.delegate
    }

    pub fn delegate_mut(&mut self) -> &mut D {
        &mut self.delegate
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
