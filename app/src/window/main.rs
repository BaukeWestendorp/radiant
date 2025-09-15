use gpui::prelude::*;
use gpui::{App, Window, WindowHandle};

pub struct MainWindow {}

impl MainWindow {
    pub fn open(cx: &mut App) -> WindowHandle<Self> {
        cx.open_window(super::window_options(), |window, cx| cx.new(|cx| Self::new(window, cx)))
            .expect("should open main window")
    }

    fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {}
    }
}

impl Render for MainWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        super::window_root().child(ui::utils::todo(cx))
    }
}
