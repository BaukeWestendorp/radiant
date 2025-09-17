use gpui::prelude::*;
use gpui::{App, Entity, Window, WindowHandle};
use ui::nav::tabs::{Orientation, Tab, Tabs};

use crate::window::settings::patch::PatchSettings;

mod patch;

pub struct SettingsWindow {
    tabs: Entity<Tabs>,
}

impl SettingsWindow {
    pub fn open(cx: &mut App) -> WindowHandle<Self> {
        cx.open_window(super::window_options("Settings"), |window, cx| {
            cx.new(|cx| Self {
                tabs: cx.new(|cx| {
                    Tabs::new(
                        vec![Tab::new(
                            "patch",
                            "Patch",
                            cx.new(|cx| PatchSettings::new(window, cx)),
                        )],
                        window,
                        cx,
                    )
                    .with_orientation(Orientation::Vertical)
                }),
            })
        })
        .expect("should open main window")
    }
}

impl Render for SettingsWindow {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        super::window_root().child(self.tabs.clone())
    }
}
