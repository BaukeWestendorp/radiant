use rd_ui::{
    ActiveTheme,
    comp::{
        Button, ButtonVariant, Disableable, IconVariant, Labelled,
        stateful::{Tab, Tabs, TabsDirection},
    },
    gpui::{Entity, Window, prelude::*, px},
    h_flex, v_flex,
};

use crate::app::engine::EngineAppExt;

mod dmx_output;
mod patch;
mod triggers;

pub struct SettingsRootView {
    tabs: Entity<Tabs>,
    uncommitted_project: Entity<rd::Project>,
}

impl SettingsRootView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let uncommitted_project = cx.new(|cx| cx.engine().with_project(|project| project.clone()));

        Self {
            tabs: cx.new(|cx| {
                Tabs::new("tabs", window, cx)
                    .with_selected(0)
                    .with_direction(TabsDirection::Vertical)
                    .with_tab(Tab::new("Patch", cx).with_icon(IconVariant::Spotlight).with_content(
                        cx.new(|cx| {
                            patch::PatchTabView::new(uncommitted_project.clone(), window, cx)
                        }),
                        cx,
                    ))
                    .with_tab(
                        Tab::new("Triggers", cx).with_icon(IconVariant::Joystick).with_content(
                            cx.new(|cx| {
                                triggers::TriggersTabView::new(
                                    uncommitted_project.clone(),
                                    window,
                                    cx,
                                )
                            }),
                            cx,
                        ),
                    )
                    .with_tab(
                        Tab::new("DMX Output", cx)
                            .with_icon(IconVariant::CircleArrowOutUpRight)
                            .with_content(
                                cx.new(|cx| {
                                    dmx_output::DmxOutputTabView::new(
                                        uncommitted_project.clone(),
                                        window,
                                        cx,
                                    )
                                }),
                                cx,
                            ),
                    )
            }),
            uncommitted_project,
        }
    }
}

impl Render for SettingsRootView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
                Button::new("save", window, cx)
                    .with_label("Save")
                    .with_icon(IconVariant::Save)
                    .with_disabled(!project_changed, cx)
                    .on_click(cx.listener(|this, _, _, cx| {
                        let engine = cx.engine().clone();
                        let project = this.uncommitted_project.read(cx).clone();

                        cx.spawn(async move |_, _| {
                            if let Err(err) = engine.replace_project_async(project).await {
                                log::error!("Failed to update project: {err:#}");
                            }
                        })
                        .detach();
                    })),
            )
            .child(
                Button::new("discard", window, cx)
                    .with_label("Discard")
                    .with_icon(IconVariant::RotateCcw)
                    .with_disabled(!project_changed, cx)
                    .with_variant(ButtonVariant::Secondary)
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.uncommitted_project.update(cx, |uncommitted_project, cx| {
                            *uncommitted_project =
                                cx.engine().with_project(|project| project.clone());
                            cx.notify();
                        });
                    })),
            );

        v_flex().size_full().child(self.tabs.clone()).child(action_bar)
    }
}
