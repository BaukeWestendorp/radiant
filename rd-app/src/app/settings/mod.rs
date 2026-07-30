use gpui::{Entity, Window, prelude::*, px};
use rd::Project;
use rd_ui::{
    ActiveTheme, Button, ButtonVariant, IconVariant, Tab, Tabs, TabsState, h_flex, v_flex,
};

use crate::app::engine::EngineAppExt;

mod dmx_output;
mod triggers;

pub struct SettingsRootView {
    tabs: Entity<TabsState>,
    triggers_tab: Entity<triggers::TriggersTabView>,
    dmx_output_tab: Entity<dmx_output::DmxOutputTabView>,
    uncommitted_project: Entity<Project>,
}

impl SettingsRootView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let uncommitted_project = cx.new(|cx| cx.engine().with_project(|project| project.clone()));

        let this = cx.entity();
        cx.on_engine_event_in(window, {
            move |event, window, cx| match event {
                rd::Event::ProjectLoaded => {
                    this.update(cx, |this, cx| {
                        *this = Self::new(window, cx);
                        cx.notify();
                    });
                }
                _ => {}
            }
        })
        .detach();

        Self {
            tabs: cx.new(|_| TabsState::new().with_selected("triggers")),
            triggers_tab: cx
                .new(|cx| triggers::TriggersTabView::new(uncommitted_project.clone(), window, cx)),
            dmx_output_tab: cx.new(|cx| {
                dmx_output::DmxOutputTabView::new(uncommitted_project.clone(), window, cx)
            }),
            uncommitted_project,
        }
    }
}

impl Render for SettingsRootView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let project_changed = *self.uncommitted_project.read(cx)
            != cx.engine().with_project(|project| project.clone());

        let action_bar = h_flex()
            .h(px(32.0))
            .w_full()
            .p_2()
            .bg(cx.theme().bg_secondary)
            .border_t_1()
            .border_color(cx.theme().border_secondary)
            .gap_2()
            .child(
                Button::new("save", cx.focus_handle())
                    .label("Save")
                    .icon(IconVariant::Save)
                    .disabled(!project_changed)
                    .on_click(cx.listener(|this, _, _, cx| {
                        if let Err(err) = cx.update_project(|project, cx| {
                            *project = this.uncommitted_project.read(cx).clone();
                        }) {
                            log::error!("Failed to update project: {err:#}");
                        }
                    })),
            )
            .child(
                Button::new("discard", cx.focus_handle())
                    .label("Discard")
                    .icon(IconVariant::RotateCcw)
                    .disabled(!project_changed)
                    .variant(ButtonVariant::Secondary)
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.uncommitted_project.update(cx, |uncommitted_project, cx| {
                            *uncommitted_project =
                                cx.engine().with_project(|project| project.clone());
                            cx.notify();
                        });
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
                .icon(IconVariant::Joystick),
                Tab::new(
                    "dmx-output",
                    "DMX Output",
                    self.dmx_output_tab.clone().into_any_element(),
                )
                .icon(IconVariant::CircleArrowOutUpRight)
            ]))
            .child(action_bar)
    }
}
