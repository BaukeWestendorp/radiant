use gpui::*;
use regex::Regex;

use super::{TextField, TextFieldEvent};

pub struct NumberField {
    text_field: View<TextField>,
}

impl NumberField {
    pub fn new(id: impl Into<ElementId>, value: f64, cx: &mut ViewContext<Self>) -> Self {
        Self {
            text_field: {
                let field = cx.new_view(|cx| {
                    let mut field = TextField::new(id.into(), value.to_string().into(), cx);
                    field.set_pattern(Some(
                        Regex::new(r"^-?\d*\.?\d*$").expect("regex should be valid"),
                    ));

                    field
                });

                cx.subscribe(
                    &field,
                    |this, _view, event: &TextFieldEvent, cx| match event {
                        TextFieldEvent::Change(string_value) => {
                            let float_value = string_value.parse().unwrap_or_default();
                            cx.emit(NumberFieldEvent::Change(float_value));
                        }
                        TextFieldEvent::Blur => {
                            this.set_value(this.value(cx), cx);
                        }
                        _ => (),
                    },
                )
                .detach();

                field
            },
        }
    }

    pub fn value(&self, cx: &AppContext) -> f64 {
        self.text_field.read(cx).value().parse().unwrap_or_default()
    }

    pub fn set_value(&mut self, value: f64, cx: &mut ViewContext<Self>) {
        self.text_field.update(cx, |field, cx| {
            field.set_value(value.to_string().into(), cx)
        });
        cx.emit(NumberFieldEvent::Change(value));
    }

    pub fn validate<'a>(&'a self, cx: &'a AppContext) -> Option<&dyn Fn(&str) -> bool> {
        self.text_field.read(cx).validate().as_ref().map(|f| &**f)
    }

    pub fn set_validate(
        &self,
        validate: Option<Box<dyn Fn(&str) -> bool + 'static>>,
        cx: &mut ViewContext<Self>,
    ) {
        self.text_field
            .update(cx, |field, _cx| field.set_validate(validate));
    }

    pub fn pattern<'a>(&'a self, cx: &'a AppContext) -> Option<&Regex> {
        self.text_field.read(cx).pattern()
    }

    pub fn set_pattern(&self, pattern: Option<Regex>, cx: &mut ViewContext<Self>) {
        self.text_field
            .update(cx, |field, _cx| field.set_pattern(pattern));
    }

    pub fn focus(&self, cx: &mut ViewContext<Self>) {
        self.text_field.focus_handle(cx).focus(cx);
    }
}

impl Render for NumberField {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let focus_handle = self.text_field.focus_handle(cx);

        div()
            .track_focus(&focus_handle)
            .size_full()
            .overflow_hidden()
            .child(self.text_field.clone())
    }
}

impl FocusableView for NumberField {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.text_field.focus_handle(cx)
    }
}

impl EventEmitter<NumberFieldEvent> for NumberField {}

#[derive(Debug, Clone, Copy)]
pub enum NumberFieldEvent {
    Change(f64),
}
