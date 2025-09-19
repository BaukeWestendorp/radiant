use gpui::prelude::*;
use gpui::{App, Entity, Window};
use ui::nav::tabs::{Orientation, Tab, Tabs};
use ui::window::{WindowDelegate, WindowWrapper};

use crate::window::settings::patch::PatchSettings;

mod patch;

pub struct SettingsWindow {
    tabs: Entity<Tabs>,
}

impl SettingsWindow {
    pub fn new(window: &mut Window, cx: &mut App) -> Self {
        window.set_app_id("radiant");
        window.set_window_title("Settings");

        Self {
            tabs: cx.new(|cx| {
                Tabs::new(
                    vec![Tab::new("patch", "Patch", cx.new(|cx| PatchSettings::new(window, cx)))],
                    window,
                    cx,
                )
                .with_orientation(Orientation::Vertical)
            }),
        }
    }
}

impl WindowDelegate for SettingsWindow {
    fn render_content(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<WindowWrapper<Self>>,
    ) -> impl IntoElement {
        self.tabs.clone()
    }
}
