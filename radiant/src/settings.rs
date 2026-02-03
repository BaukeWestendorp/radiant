use gpui::{Window, div, prelude::*};
use rui::{TitleBar, h_flex};

pub struct SettingsView {}

impl SettingsView {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {}
    }

    fn render_title_bar_content(
        &mut self,
        window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        h_flex().size_full().justify_between().child(window.window_title())
    }
}

impl Render for SettingsView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .child(TitleBar::new().child(self.render_title_bar_content(window, cx)))
    }
}
