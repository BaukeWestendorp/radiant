use rd_ui::{
    comp::{Button, ButtonVariant, Disableable, IconVariant, Label, Labelled},
    gpui::{Window, prelude::*},
    h_flex, v_flex,
};

pub struct ButtonPreview {}

impl ButtonPreview {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {}
    }
}

impl Render for ButtonPreview {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let variants = [
            (ButtonVariant::Primary, "Primary", "button-primary"),
            (ButtonVariant::Secondary, "Secondary", "button-secondary"),
            (ButtonVariant::Ghost, "Ghost", "button-ghost"),
            (ButtonVariant::Danger, "Danger", "button-danger"),
            (ButtonVariant::Warning, "Warning", "button-warning"),
        ];

        let variants = variants.into_iter().map(|(variant, label, id)| {
            let content = v_flex()
                .gap_2()
                .child(
                    h_flex()
                        .gap_2()
                        .child(
                            Button::new(format!("{id}-text"), window, cx)
                                .with_label(label)
                                .with_variant(variant),
                        )
                        .child(
                            Button::new(format!("{id}-icon"), window, cx)
                                .with_icon(IconVariant::Star)
                                .with_variant(variant),
                        )
                        .child(
                            Button::new(format!("{id}-both"), window, cx)
                                .with_label(label)
                                .with_icon(IconVariant::Star)
                                .with_variant(variant),
                        ),
                )
                .child(
                    h_flex()
                        .gap_2()
                        .child(
                            Button::new(format!("{id}-text-disabled"), window, cx)
                                .with_label(label)
                                .with_variant(variant)
                                .with_disabled(true, cx),
                        )
                        .child(
                            Button::new(format!("{id}-icon-disabled"), window, cx)
                                .with_icon(IconVariant::Star)
                                .with_variant(variant)
                                .with_disabled(true, cx),
                        )
                        .child(
                            Button::new(format!("{id}-both-disabled"), window, cx)
                                .with_label(label)
                                .with_icon(IconVariant::Star)
                                .with_variant(variant)
                                .with_disabled(true, cx),
                        ),
                );

            Label::new(format!("{id}"), content).with_label(label)
        });

        v_flex().gap_2().w_full().p_2().children(variants)
    }
}
