use rd_ui::{
    ActiveTheme, StyledExt,
    comp::{
        IconVariant, Labelled,
        stateful::{Tab, Tabs, TabsDirection},
    },
    gpui::{Entity, SharedString, Window, div, prelude::*},
    v_flex,
};

use crate::layout::PreviewStory;

pub struct TabsPreview {
    horizontal_tabs: Entity<Tabs>,
    vertical_tabs: Entity<Tabs>,
}

impl TabsPreview {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            horizontal_tabs: cx.new(|cx| {
                Tabs::new("horizontal", window, cx)
                    .with_direction(TabsDirection::Horizontal)
                    .with_tab(
                        Tab::new("Overview", cx).with_icon(IconVariant::LayoutGrid).with_content(
                            cx.new(|_| {
                                TabPanel::new(
                                    "Overview",
                                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
                                )
                            }),
                            cx,
                        ),
                    )
                    .with_tab(
                        Tab::new("Activity", cx).with_icon(IconVariant::History).with_content(
                            cx.new(|_| {
                                TabPanel::new(
                                    "Recent activity",
                                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
                                )
                            }),
                            cx,
                        ),
                    )
                    .with_tab(
                        Tab::new("Settings", cx).with_icon(IconVariant::Settings).with_content(
                            cx.new(|_| {
                                TabPanel::new(
                                    "Settings",
                                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
                                )
                            }),
                            cx,
                        ),
                    )
                    .with_selected(1)
            }),
            vertical_tabs: cx.new(|cx| {
                Tabs::new("vertical", window, cx)
                    .with_direction(TabsDirection::Vertical)
                    .with_tab(
                        Tab::new("General", cx)
                            .with_icon(IconVariant::SlidersHorizontal)
                            .with_content(
                                cx.new(|_| {
                                    TabPanel::new(
                                        "General settings",
                                        "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
                                    )
                                }),
                                cx,
                            ),
                    )
                    .with_tab(Tab::new("Access", cx).with_icon(IconVariant::Shield).with_content(
                        cx.new(|_| {
                            TabPanel::new(
                                "Access control",
                                "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
                            )
                        }),
                        cx,
                    ))
                    .with_tab(
                        Tab::new("Notifications", cx).with_icon(IconVariant::Bell).with_content(
                            cx.new(|_| {
                                TabPanel::new(
                                    "Notifications",
                                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
                                )
                            }),
                            cx,
                        ),
                    )
                    .with_selected(0)
            }),
        }
    }
}

impl Render for TabsPreview {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .w_full()
            .gap_4()
            .p_2()
            .child(PreviewStory::new(
                "Horizontal",
                div().w_full().h_56().child(self.horizontal_tabs.clone()),
            ))
            .child(PreviewStory::new(
                "Vertical",
                div().w_full().h_64().child(self.vertical_tabs.clone()),
            ))
    }
}

struct TabPanel {
    title: SharedString,
    body: SharedString,
}

impl TabPanel {
    fn new(title: impl Into<SharedString>, body: impl Into<SharedString>) -> Self {
        Self { title: title.into(), body: body.into() }
    }
}

impl Render for TabPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .gap_2()
            .p_3()
            .child(div().font_bold().child(self.title.clone()))
            .child(div().text_color(cx.theme().fg_secondary).child(self.body.clone()))
    }
}
