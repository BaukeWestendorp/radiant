use gpui::{Entity, Window, prelude::*};
use rd_ui::{Tab, Tabs, TabsState};

mod triggers;

pub struct SettingsRootView {
    tabs: Entity<TabsState>,

    triggers_tab: Entity<triggers::TriggersTabView>,
}

impl SettingsRootView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            tabs: cx.new(|_| TabsState::new().with_selected("triggers")),
            triggers_tab: cx.new(|cx| triggers::TriggersTabView::new(window, cx)),
        }
    }
}

impl Render for SettingsRootView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Tabs::new("tabs", self.tabs.clone()).tabs(vec![Tab::new(
            "triggers",
            "Triggers",
            self.triggers_tab.clone().into_any_element(),
        )])
    }
}
