use gpui::{Entity, Window, div, prelude::*};
use rd::Project;
use rd_ui::{IconVariant, Tab, Tabs, TabsState, TabsVariant};

mod artnet;

pub struct DmxOutputTabView {
    tabs: Entity<TabsState>,
    artnet_tab: Entity<artnet::ArtnetOutputTabView>,
}

impl DmxOutputTabView {
    pub fn new(
        uncommitted_project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            tabs: cx.new(|_| TabsState::new().with_selected("artnet")),
            artnet_tab: cx.new(|cx| {
                artnet::ArtnetOutputTabView::new(uncommitted_project.clone(), window, cx)
            }),
        }
    }
}

impl Render for DmxOutputTabView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(
            Tabs::new("tabs", self.tabs.clone()).variant(TabsVariant::Top).tabs(vec![
                Tab::new("artnet", "Art-Net", self.artnet_tab.clone().into_any_element())
                    .icon(IconVariant::Network),
            ]),
        )
    }
}
