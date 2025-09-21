use gpui::prelude::*;
use gpui::{App, Entity, FocusHandle, Focusable, Window};
use nui::wm::{WindowDelegate, WindowWrapper};

use crate::pane::Pane;

pub struct MainWindow {
    pane: Entity<Pane>,

    focus_handle: FocusHandle,
}

impl Focusable for MainWindow {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl WindowDelegate for MainWindow {
    fn create(window: &mut Window, cx: &mut App) -> Self {
        window.set_app_id("radiant");
        window.set_window_title("Radiant");

        Self { pane: cx.new(|_| Pane::new()), focus_handle: cx.focus_handle() }
    }

    fn render_content(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<WindowWrapper<Self>>,
    ) -> impl IntoElement {
        self.pane.clone()
    }
}
