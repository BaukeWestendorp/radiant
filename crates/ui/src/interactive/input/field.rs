use super::{TextInput, TextInputEvent};
use crate::{Disableable, interactive_container};
use gpui::*;

pub struct Field<I: FieldImpl> {
    input: Entity<TextInput>,

    _marker: std::marker::PhantomData<I>,
}

impl<I: FieldImpl + 'static> Field<I> {
    pub fn new(
        id: impl Into<ElementId>,
        focus_handle: FocusHandle,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let input =
            cx.new(|cx| TextInput::new(id, focus_handle, window, cx).px(window.rem_size() * 0.25));

        cx.subscribe(&input, |this, _, event, cx| {
            cx.notify();
            match event {
                TextInputEvent::Focus => cx.emit(FieldEvent::Focus),
                TextInputEvent::Blur => {
                    this.commit_value(cx);
                    cx.emit(FieldEvent::Blur);
                }
                TextInputEvent::Submit(_) => cx.emit(FieldEvent::Submit(this.value(cx))),
                TextInputEvent::Change(_) => cx.emit(FieldEvent::Change(this.value(cx))),
            }
        })
        .detach();

        
        Self { input, _marker: std::marker::PhantomData }
    }

    pub fn value(&self, cx: &App) -> I::Value {
        let s = self.input.read(cx).text();
        I::from_str_or_default(s)
    }

    pub fn set_value(&self, value: &I::Value, cx: &mut App) {
        self.input.update(cx, |text_field, cx| {
            text_field.set_text(I::to_shared_string(value), cx);
        })
    }

    fn commit_value(&self, cx: &mut App) {
        self.set_value(&self.value(cx), cx);
    }

    pub fn placeholder<'a>(&self, cx: &'a App) -> &'a SharedString {
        self.input.read(cx).placeholder()
    }

    pub fn set_placeholder(&self, placeholder: impl Into<SharedString>, cx: &mut App) {
        self.input.update(cx, |input, cx| {
            input.set_placeholder(placeholder.into(), cx);
        })
    }

    pub fn disabled(&self, cx: &App) -> bool {
        self.input.read(cx).disabled()
    }

    pub fn set_disabled(&self, disabled: bool, cx: &mut App) {
        self.input.update(cx, |text_field, _cx| text_field.set_disabled(disabled));
    }

    pub fn masked(&self, cx: &App) -> bool {
        self.input.read(cx).masked()
    }

    pub fn set_masked(&self, masked: bool, cx: &mut App) {
        self.input.update(cx, |text_field, _cx| text_field.set_masked(masked));
    }
}

impl<I: FieldImpl + 'static> Focusable for Field<I> {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.input.focus_handle(cx)
    }
}

impl<I: FieldImpl + 'static> Render for Field<I> {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let focus_handle = self.input.read(cx).focus_handle(cx);

        interactive_container(ElementId::View(cx.entity_id()), Some(focus_handle))
            .w_full()
            .disabled(self.disabled(cx))
            .child(self.input.clone())
    }
}

impl<I: FieldImpl + 'static> EventEmitter<FieldEvent<I::Value>> for Field<I> {}

pub trait FieldImpl {
    type Value: Default;

    fn from_str_or_default(s: &str) -> Self::Value;

    fn to_shared_string(value: &Self::Value) -> SharedString;
}

impl<T> FieldImpl for T
where
    T: Default + std::str::FromStr + std::fmt::Display,
{
    type Value = T;

    fn from_str_or_default(s: &str) -> Self::Value {
        s.parse().unwrap_or_default()
    }

    fn to_shared_string(value: &Self::Value) -> SharedString {
        value.to_string().into()
    }
}

pub enum FieldEvent<T> {
    Focus,
    Blur,
    Submit(T),
    Change(T),
}
