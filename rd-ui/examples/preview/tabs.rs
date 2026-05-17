use gpui::prelude::*;
use gpui::{Entity, Window, div};
use rd_ui::{ActiveTheme as _, Tab, Tabs, TabsState, TabsVariant};

use crate::{alpha_content, beta_content, gamma_content};

pub struct TabsPreview {
    top_tabs: Entity<TabsState>,
    sidebar_tabs: Entity<TabsState>,
}

impl TabsPreview {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            top_tabs: cx.new(|_| TabsState::new()),
            sidebar_tabs: cx.new(|_| TabsState::new().with_selected("beta")),
        }
    }
}

impl Render for TabsPreview {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let sidebar_tabs =
            Tabs::new("sidebar_tabs", self.sidebar_tabs.clone(), TabsVariant::Sidebar).tabs([
                Tab::new("alpha", "Alpha", alpha_content().into_any_element()),
                Tab::new("beta", "Beta", beta_content().into_any_element()),
                Tab::new("gamma", "Gamma", gamma_content().into_any_element()),
            ]);
        let top_tabs = Tabs::new("top_tabs", self.top_tabs.clone(), TabsVariant::Top).tabs([
            Tab::new("alpha", "Alpha", alpha_content().into_any_element()),
            Tab::new("beta", "Beta", beta_content().into_any_element()),
            Tab::new("gamma", "Gamma", gamma_content().into_any_element()),
        ]);

        div()
            .size_full()
            .p_2()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                div()
                    .size_full()
                    .border_1()
                    .border_color(cx.theme().border_primary)
                    .child(sidebar_tabs),
            )
            .child(
                div()
                    .size_full()
                    .border_1()
                    .border_color(cx.theme().border_primary)
                    .child(top_tabs),
            )
    }
}
