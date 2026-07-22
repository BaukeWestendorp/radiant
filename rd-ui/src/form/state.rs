use gpui::{Context, EventEmitter, Window};

use crate::{FormDelegate, FormEvent};

pub struct FormState<D: FormDelegate + 'static> {
    pub delegate: D,
}

impl<D: FormDelegate + 'static> EventEmitter<FormEvent<D>> for FormState<D> {}

impl<D: FormDelegate + 'static> FormState<D> {
    pub fn new(delegate: D, _window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self { delegate }
    }

    pub fn delegate(&self) -> &D {
        &self.delegate
    }

    pub fn submit(&mut self, cx: &mut Context<Self>) {
        if let Some(data) = self.delegate.extract_data(cx) {
            cx.emit(FormEvent::Submit { data });
        }
    }
}
