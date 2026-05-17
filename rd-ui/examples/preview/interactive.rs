use gpui::prelude::*;
use gpui::{Window, div};
use rd_ui::{Button, Icon, IconSize, IconVariant, section};

pub struct InteractivePreview {}

impl InteractivePreview {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {}
    }
}

impl Render for InteractivePreview {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let buttons = div()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                section("States").size_full().child(
                    div()
                        .flex()
                        .gap_2()
                        .flex_wrap()
                        .child(Button::new("default").child("Default"))
                        .child(Button::new("selected").selected(true).child("Selected"))
                        .child(Button::new("disabled").disabled(true).child("Disabled"))
                        .child(
                            Button::new("icon")
                                .icon(Icon::new(IconVariant::Plus, IconSize::ExtraSmall))
                                .child("With Icon"),
                        ),
                ),
            )
            .child(
                section("With Click Handler").size_full().child(
                    div()
                        .flex()
                        .gap_2()
                        .flex_wrap()
                        .child(Button::new("click-me").child("Click Me").on_click(|_, _, _| {
                            log::info!("button clicked");
                        }))
                        .child(
                            Button::new("click-me-too")
                                .icon(Icon::new(IconVariant::Plus, IconSize::ExtraSmall))
                                .child("Click Me Too")
                                .on_click(|_, _, _| {
                                    log::info!("button clicked (with icon)");
                                }),
                        ),
                ),
            );

        div().p_2().child(section("Buttons").child(div().p_2().child(buttons)))
    }
}
