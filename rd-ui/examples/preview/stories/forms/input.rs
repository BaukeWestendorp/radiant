use gpui::SharedString;
use rd_ui::{
    comp::{Disableable, Label, Labelled, stateful::Field},
    gpui::{Entity, Window, prelude::*},
    v_flex,
};

use crate::layout::PreviewStory;

pub struct InputPreview {
    empty: Entity<Field<SharedString>>,
    with_placeholder: Entity<Field<SharedString>>,
    prefilled: Entity<Field<SharedString>>,
    masked: Entity<Field<SharedString>>,
    disabled: Entity<Field<SharedString>>,
    integer: Entity<Field<i32>>,
    custom: Entity<Field<CustomValue>>,
    submit_validated: Entity<Field<SharedString>>,
}

impl InputPreview {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            empty: cx.new(|cx| Field::new("field-empty", window, cx)),
            with_placeholder: cx.new(|cx| {
                Field::new("field-placeholder", window, cx).with_placeholder("Type to search…", cx)
            }),
            prefilled: cx.new(|cx| {
                Field::new("field-prefilled", window, cx).with_value("Radiant UI".into(), cx)
            }),
            masked: cx.new(|cx| {
                Field::new("field-masked", window, cx)
                    .with_value("super-secret".into(), cx)
                    .with_masked(true, cx)
            }),
            disabled: cx.new(|cx| {
                Field::new("field-disabled", window, cx)
                    .with_value("Text Value".into(), cx)
                    .with_disabled(true, cx)
            }),
            integer: cx.new(|cx| {
                Field::new("field-digits-only", window, cx)
                    .with_placeholder("Digits only", cx)
                    .with_value(42, cx)
            }),
            custom: cx.new(|cx| {
                Field::custom(
                    "field-custom",
                    window,
                    cx,
                    |text| {
                        if text.starts_with("custom:") {
                            Some(CustomValue(text[7..].to_string()))
                        } else {
                            None
                        }
                    },
                    |value: &CustomValue| format!("custom:{}", value.0).into(),
                )
                .with_placeholder("Custom value", cx)
            }),
            submit_validated: cx.new(|cx| {
                Field::new("field-submit-validated", window, cx)
                    .with_placeholder("Press Enter after typing at least 3 characters", cx)
                    .with_submit_validator(cx, |text: &SharedString| text.trim().len() >= 3)
            }),
        }
    }
}

impl Render for InputPreview {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .w_full()
            .gap_4()
            .p_2()
            .child(PreviewStory::new(
                "States",
                v_flex()
                    .w_full()
                    .gap_2()
                    .p_2()
                    .child(Label::new("empty", self.empty.clone()).with_label("Empty"))
                    .child(
                        Label::new("with_placeholder", self.with_placeholder.clone())
                            .with_label("With placeholder"),
                    )
                    .child(Label::new("prefilled", self.prefilled.clone()).with_label("Prefilled"))
                    .child(Label::new("masked", self.masked.clone()).with_label("Masked"))
                    .child(Label::new("custom", self.custom.clone()).with_label("Custom"))
                    .child(Label::new("disabled", self.disabled.clone()).with_label("Disabled")),
            ))
            .child(PreviewStory::new(
                "Validation",
                v_flex()
                    .w_full()
                    .gap_2()
                    .p_2()
                    .child(Label::new("integer", self.integer.clone()).with_label("Live validator"))
                    .child(
                        Label::new("submit_validated", self.submit_validated.clone())
                            .with_label("Submit validator"),
                    ),
            ))
    }
}

#[derive(Clone)]
struct CustomValue(String);
