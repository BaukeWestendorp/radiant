use gpui::{Entity, Window, prelude::*};
use rd::Project;
use rd_ui::{
    ActiveTheme, Button, Icon, IconSize, IconVariant, Tab, Tabs, TabsState, h_flex, v_flex,
};

use crate::app::engine::EngineAppExt;

mod triggers;

pub struct SettingsRootView {
    tabs: Entity<TabsState>,
    triggers_tab: Entity<triggers::TriggersTabView>,

    uncommitted_project: Entity<Project>,
}

impl SettingsRootView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let uncommitted_project = cx.new(|cx| cx.engine().with_project(|project| project.clone()));
        Self {
            tabs: cx.new(|_| TabsState::new().with_selected("triggers")),
            triggers_tab: cx
                .new(|cx| triggers::TriggersTabView::new(uncommitted_project.clone(), window, cx)),
            uncommitted_project,
        }
    }
}

impl Render for SettingsRootView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let bottom_bar = h_flex()
            .justify_end()
            .w_full()
            .p_1()
            .bg(cx.theme().bg_primary)
            .border_t_1()
            .border_color(cx.theme().border_primary)
            .child(
                Button::new("save", cx.focus_handle())
                    .label("Save Settings")
                    .icon(IconVariant::Save)
                    .on_click(cx.listener(|this, _, _, cx| {
                        if let Err(err) = cx.update_project(|project, cx| {
                            *project = this.uncommitted_project.read(cx).clone();
                        }) {
                            log::error!("Failed to update project: {err:#}");
                        }
                    })),
            );

        v_flex()
            .size_full()
            .child(Tabs::new("tabs", self.tabs.clone()).tabs(vec![
                Tab::new(
                    "triggers",
                    "Triggers",
                    self.triggers_tab.clone().into_any_element(),
                )
                .icon(IconVariant::Plug)
            ]))
            .child(bottom_bar)
    }
}
