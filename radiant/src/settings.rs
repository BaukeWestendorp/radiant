use gpui::{Entity, Window, div, prelude::*};
use rui::{Tab, Tabs, TabsState, TabsVariant, TitleBar, h_flex, v_flex};

pub struct SettingsView {
    tabs_state: Entity<TabsState>,
}

impl SettingsView {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self { tabs_state: cx.new(|_| TabsState::new().with_selected("patch")) }
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
        Tabs::new(self.tabs_state.clone(), TabsVariant::Top).tabs([
            Tab::new("patch", "Patch", div().child("PATCH PANEL").into_any_element()),
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
            .child(self.render_content(window, cx))
    }
}
