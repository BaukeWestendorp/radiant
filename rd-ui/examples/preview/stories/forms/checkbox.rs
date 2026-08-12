use rd_ui::{
    comp::{Disableable, Label, Labelled, stateful::Checkbox},
    gpui::{Entity, Window, prelude::*},
    v_flex,
};

pub struct CheckboxPreview {
    unchecked: Entity<Checkbox>,
    checked: Entity<Checkbox>,
    disabled_unchecked: Entity<Checkbox>,
    disabled_checked: Entity<Checkbox>,
}

impl CheckboxPreview {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            unchecked: cx.new(|cx| Checkbox::new("checkbox-unchecked", window, cx)),
            checked: cx.new(|cx| Checkbox::new("checkbox-checked", window, cx).with_checked(true)),
            disabled_unchecked: cx.new(|cx| {
                Checkbox::new("checkbox-disabled-unchecked", window, cx).with_disabled(true, cx)
            }),
            disabled_checked: cx.new(|cx| {
                Checkbox::new("checkbox-disabled-checked", window, cx)
                    .with_checked(true)
                    .with_disabled(true, cx)
            }),
        }
    }
}

impl Render for CheckboxPreview {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .p_2()
            .child(Label::new("unchecked", self.unchecked.clone()).with_label("Unchecked"))
            .child(Label::new("checked", self.checked.clone()).with_label("Checked"))
            .child(
                Label::new("disabled_unchecked", self.disabled_unchecked.clone())
                    .with_label("Disabled unchecked"),
            )
            .child(
                Label::new("disabled_checked", self.disabled_checked.clone())
                    .with_label("Disabled checked"),
            )
    }
}
