use gpui::prelude::*;
use gpui::{Entity, Window, div};
use ui::nav::tabs::{Orientation, Tab, Tabs};

use crate::pane::patch::PatchSettings;
use crate::window::main::MainWindow;

pub struct SettingsPane {
    tabs: Entity<Tabs>,
}

impl SettingsPane {
    pub fn new(
        main_window: Entity<MainWindow>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            tabs: cx.new(|cx| {
                Tabs::new(
                    vec![Tab::new(
                        "patch",
                        "Patch",
                        cx.new(|cx| PatchSettings::new(main_window, window, cx)),
                    )],
                    window,
                    cx,
                )
                .with_orientation(Orientation::Vertical)
            }),
        }
    }
}

impl Render for SettingsPane {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(self.tabs.clone())
    }
}
