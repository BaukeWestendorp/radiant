use rd_ui::{
    comp::{Disableable, Label, Labelled, stateful::Field},
    gpui::{Entity, Window, prelude::*},
    v_flex,
};

use crate::layout::PreviewStory;

pub struct InputPreview {
    empty: Entity<Field>,
    with_placeholder: Entity<Field>,
    prefilled: Entity<Field>,
    masked: Entity<Field>,
    disabled: Entity<Field>,
    digits_only: Entity<Field>,
    submit_validated: Entity<Field>,
}

impl InputPreview {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            empty: cx.new(|cx| Field::new("field-empty", window, cx)),
            with_placeholder: cx.new(|cx| {
                Field::new("field-placeholder", window, cx).with_placeholder("Type to search…", cx)
            }),
            prefilled: cx
                .new(|cx| Field::new("field-prefilled", window, cx).with_text("Radiant UI", cx)),
            masked: cx.new(|cx| {
                Field::new("field-masked", window, cx)
                    .with_text("super-secret", cx)
                    .with_masked(true, cx)
            }),
            disabled: cx.new(|cx| {
                Field::new("field-disabled", window, cx)
                    .with_text("Disabled value", cx)
                    .with_disabled(true, cx)
            }),
            digits_only: cx.new(|cx| {
                Field::new("field-digits-only", window, cx)
                    .with_placeholder("Digits only", cx)
                    .with_validator(cx, |text| text.chars().all(|char| char.is_ascii_digit()))
            }),
            submit_validated: cx.new(|cx| {
                Field::new("field-submit-validated", window, cx)
                    .with_placeholder("Press Enter after typing at least 3 characters", cx)
                    .with_submit_validator(cx, |text| text.trim().len() >= 3)
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
                    .child(Label::new("disabled", self.disabled.clone()).with_label("Disabled")),
            ))
            .child(PreviewStory::new(
                "Validation",
                v_flex()
                    .w_full()
                    .gap_2()
                    .p_2()
                    .child(
                        Label::new("digits_only", self.digits_only.clone())
                            .with_label("Live validator"),
                    )
                    .child(
                        Label::new("submit_validated", self.submit_validated.clone())
                            .with_label("Submit validator"),
                    ),
            ))
    }
}
