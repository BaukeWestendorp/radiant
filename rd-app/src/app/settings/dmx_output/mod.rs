use rd_ui::{
    comp::{
        IconVariant, Labelled,
        stateful::{Tab, Tabs, TabsDirection},
    },
    gpui::{Entity, Window, div, prelude::*},
};

mod artnet;

pub struct DmxOutputTabView {
    tabs: Entity<Tabs>,
}

impl DmxOutputTabView {
    pub fn new(
        uncommitted_project: Entity<rd::Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            tabs: cx.new(|cx| {
                Tabs::new("tabs", window, cx)
                    .with_selected(0)
                    .with_direction(TabsDirection::Horizontal)
                    .with_tab(Tab::new("Art-Net", cx).with_icon(IconVariant::Network).with_content(
                        cx.new(|cx| {
                            artnet::ArtnetOutputTabView::new(
                                uncommitted_project.clone(),
                                window,
                                cx,
                            )
                        }),
                        cx,
                    ))
            }),
        }
    }
}

impl Render for DmxOutputTabView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(self.tabs.clone())
    }
}
