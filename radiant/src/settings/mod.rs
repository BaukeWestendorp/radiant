use gpui::{Entity, Window, div, prelude::*};
use rui::{Tab, Tabs, TabsState, TabsVariant, TitleBar, h_flex, v_flex};

use crate::settings::patch::PatchSettingsView;

mod patch;

pub struct SettingsView {
    tabs_state: Entity<TabsState>,
    patch_settings_view: Entity<PatchSettingsView>,
}

impl SettingsView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            tabs_state: cx.new(|_| TabsState::new().with_selected("patch")),
            patch_settings_view: cx.new(|cx| PatchSettingsView::new(window, cx)),
        }
    }

    fn render_title_bar_content(
        &mut self,
        window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        h_flex().size_full().justify_between().child(window.window_title())
    }

    fn render_content(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        Tabs::new(self.tabs_state.clone(), TabsVariant::Sidebar).tabs([
            Tab::new("patch", "Patch", self.patch_settings_view.clone().into_any_element()),
            Tab::new(
                "dmx_output",
                "DMX Output",
                div().child("DMX OUTPUT PANEL").into_any_element(),
            ),
        ])
    }
}

impl Render for SettingsView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .child(TitleBar::new().child(self.render_title_bar_content(window, cx)))
            .child(div().overflow_hidden().child(self.render_content(window, cx)))
    }
}
