use gpui::prelude::*;
use gpui::{
    AnyView, App, ClickEvent, Entity, FontWeight, SharedString, Window, WindowHandle, div, px,
};
use ui::interactive::button::button;

use crate::pane::Pane;
use crate::pane::settings::SettingsPane;

pub struct MainWindow {
    pane: Entity<Pane>,
}

impl MainWindow {
    pub fn open(cx: &mut App) -> WindowHandle<Self> {
        cx.open_window(super::window_options(), |_, cx| {
            cx.new(|cx| Self { pane: cx.new(|_cx| Pane::new()) })
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

    fn render_titlebar_content(&self, cx: &Context<Self>) -> impl IntoElement {
        let settings_button =
            button("settings", None, "=").on_click(cx.listener(Self::handle_click_settings_button));

        div()
            .size_full()
            .flex()
            .justify_between()
            .items_center()
            .child(div().font_weight(FontWeight::BOLD).pb(px(-2.0)).child("Radiant"))
            .child(settings_button)
    }

    fn handle_click_settings_button(
        &mut self,
        _event: &ClickEvent,
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

impl Render for MainWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        super::window_root()
            .titlebar_child(self.render_titlebar_content(cx))
            .child(self.pane.clone())
    }
}
