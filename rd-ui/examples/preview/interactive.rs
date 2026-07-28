use gpui::{App, prelude::*};
use gpui::{Window, div};
use rd_ui::{Button, ButtonVariant, Icon, IconSize, IconVariant, section};

pub struct InteractivePreview {}

impl InteractivePreview {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {}
    }
}

impl Render for InteractivePreview {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let variants = [
            (ButtonVariant::Primary, "Primary", "primary"),
            (ButtonVariant::Secondary, "Secondary", "secondary"),
            (ButtonVariant::Danger, "Danger", "danger"),
            (ButtonVariant::Warning, "Warning", "warning"),
            (ButtonVariant::Ghost, "Ghost", "ghost"),
        ];

        let states = [(false, "default"), (true, "disabled")];

        div().flex().flex_col().gap_2().children(variants.into_iter().map(
            |(variant, name, prefix)| {
                div().flex().gap_2().children(states.iter().map(|&(disabled, state_suffix)| {
                    let id = format!("{}-{}", prefix, state_suffix);
                    Button::new(id, cx.focus_handle())
                        .label(name.to_string())
                        .variant(variant)
                        .disabled(disabled)
                        .w_full()
                        .on_click(|_event, _window, _cx| {
                            log::info!("Button clicked: {}", name.to_string());
                        })
                }))
            },
        ));

        let buttons = div()
            .flex()
            .flex_col()
            .gap_2()
            .child(section("States").size_full().child(render_all_button_combinations(cx)))
            .child(
                section("With Click Handler").size_full().child(
                    div()
                        .flex()
                        .gap_2()
                        .flex_wrap()
                        .child(
                            Button::new("click-me", cx.focus_handle()).label("Click Me").on_click(
                                |_, _, _| {
                                    log::info!("button clicked");
                                },
                            ),
                        )
                        .child(
                            Button::new("click-me-too", cx.focus_handle())
                                .label("Click Me Too")
                                .icon(Icon::new(IconVariant::Plus, IconSize::ExtraSmall))
                                .on_click(|_, _, _| {
                                    log::info!("button clicked (with icon)");
                                }),
                        ),
                ),
            );

        div().p_2().child(section("Buttons").child(div().p_2().child(buttons)))
    }
}

fn render_all_button_combinations(cx: &App) -> impl IntoElement {
    let variants = [
        (ButtonVariant::Primary, "Primary", "primary"),
        (ButtonVariant::Secondary, "Secondary", "secondary"),
        (ButtonVariant::Danger, "Danger", "danger"),
        (ButtonVariant::Warning, "Warning", "warning"),
        (ButtonVariant::Ghost, "Ghost", "ghost"),
    ];

    let states = [(false, "default"), (true, "disabled")];

    div().flex().flex_col().gap_2().children(variants.into_iter().map(|(variant, name, prefix)| {
        div().flex().gap_2().children(states.iter().map(|&(disabled, state_suffix)| {
            let id = format!("{}-{}", prefix, state_suffix);
            Button::new(id, cx.focus_handle())
                .label(name.to_string())
                .variant(variant)
                .disabled(disabled)
                .w_full()
                .on_click(|_event, _window, _cx| {
                    log::info!("Button clicked: {}", name.to_string());
                })
        }))
    }))
}
