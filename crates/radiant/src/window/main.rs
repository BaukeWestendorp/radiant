use gpui::prelude::*;
use gpui::{App, Entity, FocusHandle, Focusable, Window, WindowHandle, div};
use ui::interactive::button::button;

use crate::pane::Pane;
use crate::state::AppState;

pub struct MainWindow {
    pane: Entity<Pane>,

    focus_handle: FocusHandle,
}

impl MainWindow {
    pub fn open(cx: &mut App) -> WindowHandle<Self> {
        cx.open_window(super::window_options("Radiant"), |window, cx| {
            let main_window = cx.new(|cx| Self {
                pane: cx.new(|_cx| Pane::new()),
                focus_handle: cx.focus_handle(),
            });

            window.focus(&main_window.focus_handle(cx));

            main_window
        })
        .expect("should open main window")
    }

    fn render_titlebar_content(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        let settings_button = button("settings", None, "=").on_click(|_, _, cx| {
            AppState::open_settings(cx);
        });

        div().size_full().flex().justify_end().items_center().child(settings_button)
    }
}

impl Focusable for MainWindow {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for MainWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        super::window_root()
            .titlebar_child(self.render_titlebar_content(cx))
            .child(self.pane.clone())
    }
}
