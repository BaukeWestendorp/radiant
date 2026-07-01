use crate::FormDelegate;
use gpui::{Context, EventEmitter, Window};

pub struct FormState<D: FormDelegate> {
    pub delegate: D,
}

impl<D: FormDelegate> EventEmitter<FormEvent<D>> for FormState<D> {}

impl<D: FormDelegate + 'static> FormState<D> {
    pub fn new(delegate: D, _window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self { delegate }
    }

    pub fn delegate(&self) -> &D {
        &self.delegate
    }

    pub fn submit(&mut self, cx: &mut Context<Self>) {
        let Some(data) = self.delegate.extract_data(cx) else { return };
        cx.emit(FormEvent::Submit { data });
    }
}

pub enum FormEvent<D: FormDelegate> {
    Submit { data: D::Data },
}
