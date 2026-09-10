use gpui::{
    App, ElementId, Entity, EventEmitter, FocusHandle, Focusable, SharedString, StyleRefinement,
    Window, prelude::*, px,
};
use std::sync::Arc;

use crate::{
    ActiveTheme, Emphasis, HslaExt, StyledExt, StyledParentExt,
    comp::{
        Disableable, FocusableComponent, Identifiable,
        stateful::{FormWidget, InputEvent, InputValue, Submittable, TextInput},
    },
    h_flex,
};

pub struct Field<T> {
    text_input: Entity<TextInput>,
    style: StyleRefinement,
    parser: Arc<dyn Fn(&str) -> InputValue<T> + Send + Sync + 'static>,
    formatter: Arc<dyn Fn(&T) -> SharedString + Send + Sync + 'static>,
}

impl<T: FieldValue + Clone + 'static> Field<T> {
    pub fn new(id: impl Into<ElementId>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let this = Self::custom(id, window, cx, T::parse, |v| v.format());
        let parser = this.parser.clone();

        this.text_input.update(cx, |input, _| {
            input.set_validator(T::validate_input);
            input.set_submit_validator(move |s| {
                if let InputValue::Valid(parsed) = parser(s) {
                    T::validate_submit(s) && parsed.validate()
                } else {
                    false
                }
            });
        });

        this
    }
}

impl<T: Clone + 'static> Field<T> {
    pub fn custom(
        id: impl Into<ElementId>,
        window: &mut Window,
        cx: &mut Context<Self>,
        parser: impl Fn(&str) -> InputValue<T> + Send + Sync + 'static,
        formatter: impl Fn(&T) -> SharedString + Send + Sync + 'static,
    ) -> Self {
        let text_input = cx.new(move |cx| {
            TextInput::new(id, window, cx).with_text_size(cx.theme().font_size - px(1.0))
        });

        cx.subscribe(&text_input, |this: &mut Self, _, event: &InputEvent<SharedString>, cx| {
            match event {
                InputEvent::Submit(_) => {
                    let InputValue::Valid(value) = this.value(cx) else { return };
                    cx.emit(InputEvent::Submit(value));
                }
                InputEvent::Change(_) => {
                    cx.emit(InputEvent::Change(this.value(cx)));
                }
            }
        })
        .detach();

        Self {
            text_input,
            style: StyleRefinement::default(),
            parser: Arc::new(parser),
            formatter: Arc::new(formatter),
        }
    }

    pub fn value(&self, cx: &App) -> InputValue<T> {
        let text = self.text_input.read(cx).text();
        (self.parser)(text.as_ref())
    }

    pub fn set_value(&self, value: T, cx: &mut Context<Self>) {
        let text = (self.formatter)(&value);
        self.text_input.update(cx, |this, cx| {
            this.set_text(text, cx);
            this.move_to_end_of_line(cx);
        });
    }

    pub fn with_value(self, value: T, cx: &mut Context<Self>) -> Self {
        self.set_value(value, cx);
        self
    }

    pub fn clear(&self, cx: &mut Context<Self>) {
        self.text_input.update(cx, |this, cx| {
            this.set_text("", cx);
            this.move_to_end_of_line(cx);
        });
    }

    pub fn placeholder<'a>(&self, cx: &'a App) -> &'a SharedString {
        self.text_input.read(cx).placeholder()
    }

    pub fn set_placeholder(&self, placeholder: impl Into<SharedString>, cx: &mut Context<Self>) {
        self.text_input.update(cx, |input, cx| {
            input.set_placeholder(placeholder.into(), cx);
        })
    }

    pub fn with_placeholder(
        self,
        placeholder: impl Into<SharedString>,
        cx: &mut Context<Self>,
    ) -> Self {
        self.set_placeholder(placeholder, cx);
        self
    }

    pub fn masked(&self, cx: &App) -> bool {
        self.text_input.read(cx).masked()
    }

    pub fn set_masked(&self, masked: bool, cx: &mut App) {
        self.text_input.update(cx, |text_input, _cx| text_input.set_masked(masked));
    }

    pub fn with_masked(self, masked: bool, cx: &mut App) -> Self {
        self.set_masked(masked, cx);
        self
    }

    pub fn set_text_validator<F: Fn(&str) -> bool + Send + Sync + 'static>(
        &self,
        cx: &mut App,
        validator: F,
    ) {
        self.text_input.update(cx, |text_field, _cx| {
            text_field.set_validator(validator);
        });
    }

    pub fn with_text_validator<F: Fn(&str) -> bool + Send + Sync + 'static>(
        self,
        cx: &mut App,
        validator: F,
    ) -> Self {
        self.set_text_validator(cx, validator);
        self
    }

    pub fn set_validator<F: Fn(&T) -> bool + Send + Sync + 'static>(
        &self,
        cx: &mut App,
        validator: F,
    ) {
        let parser = self.parser.clone();
        self.text_input.update(cx, |text_field, _cx| {
            text_field.set_validator(move |s| {
                if let InputValue::Valid(parsed) = parser(s) { validator(&parsed) } else { false }
            });
        });
    }

    pub fn with_validator<F: Fn(&T) -> bool + Send + Sync + 'static>(
        self,
        cx: &mut App,
        validator: F,
    ) -> Self {
        self.set_validator(cx, validator);
        self
    }

    pub fn set_submit_validator<F: Fn(&T) -> bool + Send + Sync + 'static>(
        &self,
        cx: &mut App,
        validator: F,
    ) {
        let parser = self.parser.clone();
        self.text_input.update(cx, |text_field, _cx| {
            text_field.set_submit_validator(move |s| {
                if let InputValue::Valid(parsed) = parser(s) { validator(&parsed) } else { false }
            });
        });
    }

    pub fn with_submit_validator<F: Fn(&T) -> bool + Send + Sync + 'static>(
        self,
        cx: &mut App,
        validator: F,
    ) -> Self {
        self.set_submit_validator(cx, validator);
        self
    }
}

impl<T: 'static> Styled for Field<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl<T: 'static> Focusable for Field<T> {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.text_input.read(cx).focus_handle(cx)
    }
}

impl<T: 'static> FocusableComponent for Field<T> {
    fn set_focus_handle(&mut self, focus_handle: FocusHandle, cx: &mut App) {
        self.text_input.update(cx, |text_input, cx| {
            text_input.set_focus_handle(focus_handle, cx);
        })
    }
}

impl<T: 'static> Disableable for Field<T> {
    fn disabled(&self, cx: &App) -> bool {
        self.text_input.read(cx).disabled(cx)
    }

    fn set_disabled(&mut self, disabled: bool, cx: &mut App) {
        self.text_input.update(cx, |text_input, cx| {
            text_input.set_disabled(disabled, cx);
            cx.notify();
        })
    }
}

impl<T: 'static> Identifiable for Field<T> {
    fn id<'a>(&'a self, cx: &'a App) -> &'a ElementId {
        self.text_input.read(cx).id(cx)
    }
}

impl<T: 'static> Render for Field<T> {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .id(self.id(cx).clone())
            .focus_ring(&self.focus_handle(cx), window, cx)
            .min_w(crate::comp::INPUT_SIZE * 2.0)
            .w(crate::comp::INPUT_SIZE * 6.0)
            .h(crate::comp::INPUT_SIZE)
            .px_1p5()
            .py_0p5()
            .when(!self.disabled(cx), |e| {
                e.emphasis_bordered(Emphasis::Secondary, cx).hover(|e| {
                    e.bg(Emphasis::Secondary.bg_color(cx).hover())
                        .border_color(Emphasis::Secondary.border_color(cx).hover())
                })
            })
            .when(self.disabled(cx), |e| e.emphasis_bordered(Emphasis::Secondary, cx))
            .refine_style(&self.style)
            .child(self.text_input.clone())
    }
}

impl<T: Clone + 'static> EventEmitter<InputEvent<T>> for Field<T> {}

impl<T: Clone + 'static> Submittable<T> for Field<T> {
    fn value(&self, cx: &App) -> InputValue<T> {
        Field::<T>::value(&self, cx)
    }
}

impl<T: Clone + 'static> FormWidget<T> for Field<T> {
    fn value(&self, cx: &App) -> InputValue<T> {
        self.value(cx)
    }

    fn set_value(&mut self, value: T, cx: &mut Context<Self>) {
        Field::set_value(self, value, cx);
    }
}

pub trait FieldValue: Sized {
    fn parse(text: &str) -> InputValue<Self>;

    fn format(&self) -> SharedString;

    fn validate_input(text: &str) -> bool {
        text.is_empty() || Self::parse(text).is_valid()
    }

    fn validate_submit(text: &str) -> bool {
        Self::parse(text).is_valid()
    }

    fn validate(&self) -> bool {
        true
    }
}

impl FieldValue for String {
    fn parse(text: &str) -> InputValue<Self> {
        InputValue::Valid(text.to_string())
    }

    fn format(&self) -> SharedString {
        self.clone().into()
    }
}

impl FieldValue for SharedString {
    fn parse(text: &str) -> InputValue<Self> {
        InputValue::Valid(text.into())
    }

    fn format(&self) -> SharedString {
        self.clone()
    }
}

macro_rules! impl_field_value_for_numeric {
    ($($t:ty),*) => {
        $(
            impl FieldValue for $t {
                fn parse(text: &str) -> InputValue<Self> {
                    text.parse::<$t>().ok().into()
                }

                fn format(&self) -> SharedString {
                    self.to_string().into()
                }
            }
        )*
    };
}

impl_field_value_for_numeric!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
);
