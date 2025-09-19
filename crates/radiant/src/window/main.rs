use gpui::prelude::*;
use gpui::{App, Entity, FocusHandle, Focusable, Window, div};
use ui::interactive::button::button;
use ui::window::{WindowDelegate, WindowManager, WindowWrapper};

use crate::pane::Pane;
use crate::window::settings::SettingsWindow;

pub struct MainWindow {
    pane: Entity<Pane>,

    focus_handle: FocusHandle,
}

impl MainWindow {
    pub fn new(window: &mut Window, cx: &mut App) -> Self {
        window.set_app_id("radiant");
        window.set_window_title("Radiant");

        Self { pane: cx.new(|_| Pane::new()), focus_handle: cx.focus_handle() }
    }
}

impl Focusable for MainWindow {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl WindowDelegate for MainWindow {
    fn render_content(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<WindowWrapper<Self>>,
    ) -> impl IntoElement {
        self.pane.clone()
    }

    fn render_titlebar_content(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<WindowWrapper<Self>>,
    ) -> impl IntoElement {
        let settings_button = button("settings", None, "=").on_click(|_, _, cx| {
            WindowManager::open_window(cx, |window, cx| SettingsWindow::new(window, cx));
        });

        div().size_full().flex().justify_end().items_center().child(settings_button)
    }
}
