use gpui::prelude::*;
use gpui::{Entity, Window};
use nui::AppExt;
use nui::tabs::{Orientation, Tab, Tabs};
use nui::wm::{WindowDelegate, WindowWrapper};
use radlib::cmd::{Command, PatchCommand};
use radlib::engine::event::EngineEvent;

use crate::engine::EngineManager;
use crate::window::settings::patch::PatchSettings;

mod patch;

pub struct SettingsWindow {
    tabs: Entity<Tabs>,
}

impl WindowDelegate for SettingsWindow {
    fn create(window: &mut Window, cx: &mut Context<WindowWrapper<Self>>) -> Self {
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

    fn render_content(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<WindowWrapper<Self>>,
    ) -> impl IntoElement {
        self.tabs.clone()
    }
}
