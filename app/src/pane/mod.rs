use gpui::prelude::*;
use gpui::{AnyView, FontWeight, SharedString, Window, div, px};
use ui::interactive::button::button;
use ui::theme::ActiveTheme;
use ui::utils::z_stack;

pub mod patch;
pub mod settings;

pub struct Pane {
    overlays: Vec<(SharedString, AnyView)>,
}

impl Pane {
    pub fn new() -> Self {
        Self { overlays: Vec::new() }
    }

    pub fn push_overlay(
        &mut self,
        title: impl Into<SharedString>,
        content: impl Into<AnyView>,
        cx: &mut Context<Self>,
    ) {
        self.overlays.push((title.into(), content.into()));
        cx.notify();
    }

    pub fn pop_overlay(&mut self, cx: &mut Context<Self>) {
        self.overlays.pop();
        cx.notify();
    }
}

impl Render for Pane {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let render_overlay = |(title, content)| {
            let header = div()
                .w_full()
                .min_h(px(32.0))
                .max_h(px(32.0))
                .flex()
                .justify_between()
                .items_center()
                .px_2()
                .bg(cx.theme().header)
                .border_1()
                .border_color(cx.theme().header_border)
                .text_color(cx.theme().header_foreground)
                .child(div().font_weight(FontWeight::BOLD).child(title))
                .child(
                    button("close", None, "X")
                        .on_click(cx.listener(|this, _, _, cx| this.pop_overlay(cx))),
                );

            div().flex().flex_col().size_full().occlude().child(header).child(content)
        };

        let overlays = self.overlays.iter().cloned().map(render_overlay);

        let mut layers = vec![div().size_full().child(ui::utils::todo(cx))];
        layers.extend(overlays);

        z_stack(layers).size_full()
    }
}
