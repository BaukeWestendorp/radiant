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

        EngineManager::exec_and_log_err(Command::Patch(PatchCommand::Edit), cx);

        cx.subscribe_in(&EngineManager::event_handler(cx), window, |_, _, event, window, cx| {
            match event {
                EngineEvent::PatchChanged => {
                    cx.update_wm(|wm, _| wm.set_edited(window, true));
                }
            }
        })
        .detach();

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

    fn handle_window_discard(&self, _window: &mut Window, cx: &mut Context<WindowWrapper<Self>>) {
        EngineManager::exec_and_log_err(Command::Patch(PatchCommand::Discard), cx);
    }

    fn handle_window_save(&self, _window: &mut Window, cx: &mut Context<WindowWrapper<Self>>) {
        EngineManager::exec_and_log_err(Command::Patch(PatchCommand::Save), cx);
    }
}
