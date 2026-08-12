use rd_ui::{
    AppBuilder,
    comp::stateful::Tabs,
    gpui::{Entity, Window, div, prelude::*},
};

use crate::catalog::build_root_tabs;

mod catalog;
mod layout;
mod stories;

fn main() {
    pretty_env_logger::formatted_builder().filter_level(log::LevelFilter::Info).init();

    AppBuilder::new()
        .with_window_title("RD-UI Preview")
        .with_settings_window_content(|_window, cx| cx.new(|_| gpui::Empty).into())
        .run(|window, cx| cx.new(|cx| PreviewRootView::new(window, cx)));
}

struct PreviewRootView {
    tabs: Entity<Tabs>,
}

impl PreviewRootView {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self { tabs: build_root_tabs(window, cx) }
    }
}

impl Render for PreviewRootView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(self.tabs.clone())
    }
}
