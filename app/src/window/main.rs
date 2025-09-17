use gpui::prelude::*;
use gpui::{
    AnyView, App, Entity, FocusHandle, Focusable, FontWeight, SharedString, Window, WindowHandle,
    div, px,
};
use ui::interactive::button::button;

use crate::pane::Pane;
use crate::pane::settings::SettingsPane;

pub mod actions {
    use gpui::{App, KeyBinding};

    gpui::actions!(main_window, [OpenSettings]);

    pub const KEY_CONTEXT: &str = "MainWindow";

    pub(crate) fn init(cx: &mut App) {
        cx.bind_keys([
            KeyBinding::new("f12", OpenSettings, Some(KEY_CONTEXT)),
            KeyBinding::new("secondary-,", OpenSettings, Some(KEY_CONTEXT)),
        ]);
    }
}

pub struct MainWindow {
    pane: Entity<Pane>,

    focus_handle: FocusHandle,
}

impl MainWindow {
    pub fn open(cx: &mut App) -> WindowHandle<Self> {
        cx.open_window(super::window_options(), |window, cx| {
            let main_window = cx.new(|cx| Self {
                pane: cx.new(|_cx| Pane::new()),
                focus_handle: cx.focus_handle(),
            });

            window.focus(&main_window.focus_handle(cx));

            main_window
        })
        .expect("should open main window")
    }

    pub fn pane(&self) -> &Entity<Pane> {
        &self.pane
    }

    pub fn push_overlay(
        &mut self,
        id: impl Into<SharedString>,
        title: impl Into<SharedString>,
        content: impl Into<AnyView>,
        cx: &mut App,
    ) {
        self.pane().update(cx, |pane, cx| pane.push_overlay(id, title, content, cx));
    }

    pub fn pop_overlay(&mut self, cx: &mut App) {
        self.pane().update(cx, |pane, cx| pane.pop_overlay(cx));
    }

    fn render_titlebar_content(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let settings_button =
            button("settings", None, "=").on_click(cx.listener(|this, _, window, cx| {
                this.handle_open_settings(&actions::OpenSettings, window, cx)
            }));

        div()
            .size_full()
            .flex()
            .justify_between()
            .items_center()
            .child(div().font_weight(FontWeight::BOLD).pb(px(-2.0)).child("Radiant"))
            .child(settings_button)
    }

    fn handle_open_settings(
        &mut self,
        _: &actions::OpenSettings,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let this = cx.entity();
        self.push_overlay(
            "settings",
            "Settings",
            cx.new(|cx| SettingsPane::new(this, window, cx)),
            cx,
        )
    }
}

impl Focusable for MainWindow {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for MainWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("main_window")
            .track_focus(&self.focus_handle)
            .key_context(actions::KEY_CONTEXT)
            .on_action(cx.listener(Self::handle_open_settings))
            .size_full()
            .child(
                super::window_root()
                    .titlebar_child(self.render_titlebar_content(cx))
                    .child(self.pane.clone()),
            )
    }
}
