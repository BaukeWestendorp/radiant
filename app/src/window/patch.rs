use gpui::prelude::*;
use gpui::{App, Window, WindowHandle};

pub struct PatchWindow {}

impl PatchWindow {
    pub fn open(cx: &mut App) -> WindowHandle<Self> {
        cx.open_window(super::window_options(), |_, cx| cx.new(|_| Self::new()))
            .expect("should open patch window")
    }

    fn new() -> Self {
        Self {}
    }
}

impl Render for PatchWindow {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let content = "CONTENT";

        super::window_root(window, cx).child(content)
    }
}
