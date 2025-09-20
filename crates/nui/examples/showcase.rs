use gpui::prelude::*;
use gpui::{App, Application, KeyBinding, Window, div};
use nui::AppExt;
use nui::wm::{WindowDelegate, WindowWrapper};

gpui::actions!(app, [OpenSettings]);

pub fn main() {
    Application::new().run(move |cx: &mut App| {
        cx.activate(true);

        nui::init(cx).expect("nui failed to init");

        cx.bind_keys([KeyBinding::new("secondary-,", OpenSettings, None)]);

        cx.on_action::<OpenSettings>(|_, cx| {
            cx.update_wm(|wm, cx| wm.open_singleton_window::<SettingsWindow>(cx));
        });

        cx.update_wm(|wm, cx| {
            wm.quit_when_all_windows_closed(true);
            wm.open_singleton_window::<MainWindow>(cx);
        });
    });
}

pub struct MainWindow {}

impl WindowDelegate for MainWindow {
    fn create(window: &mut Window, _cx: &mut App) -> Self {
        window.set_window_title("Showcase Application");

        Self {}
    }

    fn render_content(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<WindowWrapper<Self>>,
    ) -> impl IntoElement {
        div()
            .text_color(gpui::white())
            .on_mouse_down(gpui::MouseButton::Left, |_, _, cx| {
                cx.update_wm(|wm, cx| wm.open_singleton_window::<SettingsWindow>(cx));
            })
            .child("open settings")
    }
}

pub struct SettingsWindow {}

impl WindowDelegate for SettingsWindow {
    fn create(window: &mut Window, _cx: &mut App) -> Self {
        window.set_window_title("Settings");

        Self {}
    }

    fn handle_window_save(&self, _window: &mut Window, _cx: &mut Context<WindowWrapper<Self>>) {
        eprintln!("SAVE");
    }

    fn handle_window_discard(&self, _window: &mut Window, _cx: &mut Context<WindowWrapper<Self>>) {
        eprintln!("DISCARD");
    }

    fn render_content(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<WindowWrapper<Self>>,
    ) -> impl IntoElement {
        div()
            .child("Settings Window")
            .child(
                div()
                    .text_color(gpui::white())
                    .on_mouse_down(gpui::MouseButton::Left, |_, window, cx| {
                        cx.update_wm(|wm, _| wm.set_edited(window, true))
                    })
                    .child("make edit"),
            )
            .child(
                div()
                    .text_color(gpui::white())
                    .on_mouse_down(gpui::MouseButton::Left, |_, window, cx| {
                        cx.update_wm(|wm, _| wm.set_edited(window, false))
                    })
                    .child("save changes"),
            )
    }
}
