use super::{TextInput, TextInputEvent};
use crate::{Disableable, interactive_container};
use gpui::*;

pub struct Field<V: FieldValue + Default> {
    input: Entity<TextInput>,
    _marker: std::marker::PhantomData<V>,
}

impl<V: FieldValue + Default + 'static> Field<V> {
    pub fn new(
        id: impl Into<ElementId>,
        focus_handle: FocusHandle,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let input =
            cx.new(|cx| TextInput::new(id, focus_handle, window, cx).px(window.rem_size() * 0.25));

        cx.subscribe(&input, |this, _, event, cx| {
            cx.emit(event.clone());
            cx.notify();
            match event {
                TextInputEvent::Blur => {
                    this.commit_value(cx);
                    this.input.update(cx, |input, _cx| input.interactive(false));
                }
                _ => {}
            }
        })
        .detach();

        let this = Self { input, _marker: Default::default() };
        this.set_value(&V::default(), cx);
        this
    }

    pub fn value<'a>(&self, cx: &'a App) -> Result<V, V::DeError> {
        let string = self.input.read(cx).text();
        V::deserialize(&string)
    }

    pub fn set_value(&self, value: &V, cx: &mut App) {
        self.input.update(cx, |text_field, cx| {
            text_field.set_text(V::serialize(value), cx);
        })
    }

    fn commit_value(&self, cx: &mut App) {
        self.set_value(&self.value(cx).unwrap_or_default(), cx);
    }

    pub fn placeholder<'a>(&self, cx: &'a App) -> &'a SharedString {
        self.input.read(cx).placeholder()
    }

    pub fn set_placeholder(&self, placeholder: SharedString, cx: &mut App) {
        self.input.update(cx, |input, cx| {
            input.set_placeholder(placeholder, cx);
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

impl<V: FieldValue + Default + 'static> Render for Field<V> {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let focus_handle = self.input.read(cx).focus_handle(cx);

        interactive_container(ElementId::View(cx.entity_id()), Some(focus_handle))
            .disabled(self.disabled(cx))
            .child(self.input.clone())
    }
}

impl<V: FieldValue + Default + 'static> Focusable for Field<V> {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.input.focus_handle(cx)
    }
}

impl<V: FieldValue + Default + 'static> EventEmitter<TextInputEvent> for Field<V> {}

pub trait FieldValue {
    type DeError;

    fn serialize(value: &Self) -> SharedString;

    fn deserialize(string: &SharedString) -> Result<Self, Self::DeError>
    where
        Self: Sized;
}

impl<T> FieldValue for T
where
    T: std::str::FromStr + std::fmt::Display + 'static,
{
    type DeError = T::Err;

    fn serialize(value: &Self) -> SharedString {
        value.to_string().into()
    }

    fn deserialize(string: &SharedString) -> Result<Self, Self::DeError> {
        T::from_str(string)
    }
}
