use gpui::prelude::*;
use gpui::{Window, div};

pub struct Pane {}

impl Pane {
    pub fn new() -> Self {
        Self {}
    }
}

impl Render for Pane {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(ui::utils::todo(cx))
    }
}
