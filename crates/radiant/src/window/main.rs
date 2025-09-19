use gpui::prelude::*;
use gpui::{App, Entity, FocusHandle, Focusable, Window, WindowHandle, div};
use ui::interactive::button::button;
use ui::overlay::OverlayContainer;
use ui::utils::z_stack;

use crate::pane::Pane;
use crate::state::AppState;

pub struct MainWindow {
    pane: Entity<Pane>,
    overlays: Entity<OverlayContainer>,

    focus_handle: FocusHandle,
}

impl MainWindow {
    pub fn open(cx: &mut App) -> WindowHandle<Self> {
        cx.open_window(super::window_options("Radiant"), |window, cx| {
            let main_window = cx.new(|cx| Self {
                pane: cx.new(|_| Pane::new()),
                overlays: cx.new(|_| OverlayContainer::new()),
                focus_handle: cx.focus_handle(),
            });

            window.focus(&main_window.focus_handle(cx));

            window.on_window_should_close(cx, |_, cx| {
                AppState::close_all_windows(cx);
                true
            });

            main_window
        })
        .expect("should open main window")
    }

    pub fn overlays(&self) -> Entity<OverlayContainer> {
        self.overlays.clone()
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
        super::window_root().titlebar_child(self.render_titlebar_content(cx)).child(
            z_stack([
                self.pane.clone().into_any_element(),
                self.overlays.clone().into_any_element(),
            ])
            .size_full(),
        )
    }
}
