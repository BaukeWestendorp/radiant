use crate::app::APP_ID;
use anyhow::Context as _;
use gpui::*;
use ui::ActiveTheme as _;

use super::DEFAULT_REM_SIZE;

pub struct SettingsWindow {}

impl SettingsWindow {
    pub fn open(cx: &mut App) -> anyhow::Result<WindowHandle<Self>> {
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                None,
                size(px(800.0), px(600.0)),
                cx,
            ))),
            app_id: Some(APP_ID.to_string()),
            ..Default::default()
        };

        cx.open_window(window_options, |window, cx| {
            window.set_rem_size(DEFAULT_REM_SIZE);
            cx.new(|_cx| Self {})
        })
        .context("open settings window")
    }
}

impl Render for SettingsWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(cx.theme().background)
            .text_color(cx.theme().text_primary)
            .child("settings window")
    }
}
