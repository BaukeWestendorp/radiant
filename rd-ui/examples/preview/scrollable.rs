use gpui::prelude::*;
use gpui::{Entity, Window, div, px};
use rd_ui::{ActiveTheme as _, Scrollable, ScrollableState, line_grid};

pub struct ScrollablePreview {
    scrollable: Entity<ScrollableState>,
}

impl ScrollablePreview {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self { scrollable: cx.new(|_| ScrollableState::new()) }
    }
}

impl Render for ScrollablePreview {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().p_2().child(
            div().size_full().border_1().border_color(cx.theme().border_primary).child(
                Scrollable::new("scrollable", self.scrollable.clone()).size_full().child(
                    div()
                        .w(px(1000.0))
                        .h(px(1000.0))
                        .child(line_grid(px(40.0), px(40.0), cx.theme().accent).size_full()),
                ),
            ),
        )
    }
}
