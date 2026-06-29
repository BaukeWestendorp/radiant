use gpui::{Entity, FontWeight, Window, div, prelude::*};
use rd_ui::{ActiveTheme as _, Tab, Tabs, TabsState, TabsVariant, TitleBar, h_flex, todo, v_flex};

mod patch;

pub struct SettingsView {
    tabs_state: Entity<TabsState>,

    patch_view: Entity<patch::PatchView>,
}

impl SettingsView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            tabs_state: cx.new(|_| TabsState::new().with_selected("patch")),
            patch_view: cx.new(|cx| patch::PatchView::new(window, cx)),
        }
    }

    fn render_title_bar_content(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        h_flex().size_full().justify_between().child(
            div()
                .font_weight(FontWeight::BOLD)
                .text_color(cx.theme().fg_secondary)
                .child(window.window_title()),
        )
    }

    fn render_content(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Tabs::new("tabs", self.tabs_state.clone(), TabsVariant::Sidebar).tabs([
            Tab::new("patch", "Patch", self.patch_view.clone().into_any_element()),
            Tab::new("dmx_output", "DMX Output", todo(cx).into_any_element()),
        ])
    }
}

impl Render for SettingsView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .child(TitleBar::new().child(self.render_title_bar_content(window, cx)))
            .child(div().size_full().overflow_hidden().child(self.render_content(window, cx)))
    }
}
