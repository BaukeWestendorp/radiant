use gpui::{Entity, ScrollHandle, SharedString, Window, div, prelude::*};
use ui::{ContainerStyle, Orientation, TabView, container};

pub struct NavigationTab {
    scroll_handle: ScrollHandle,

    h_tab_view: Entity<TabView>,
    v_tab_view: Entity<TabView>,
}

impl NavigationTab {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let tabs = vec![
            ui::Tab::new(
                "tab-1",
                "Tab 1",
                cx.new(|_| TabContent { content: "Tab 1 content".into() }).into(),
            ),
            ui::Tab::new(
                "tab-2",
                "Tab 2",
                cx.new(|_| TabContent { content: "Tab 2 content".into() }).into(),
            ),
            ui::Tab::new(
                "tab-3",
                "Tab 3",
                cx.new(|_| TabContent { content: "Tab 3 content".into() }).into(),
            ),
        ];

        Self {
            scroll_handle: ScrollHandle::new(),
            h_tab_view: cx.new(|cx| {
                let mut tab_view = TabView::new(tabs.clone(), window, cx);
                tab_view.select_tab_ix(0);
                tab_view.set_orientation(Orientation::Horizontal);
                tab_view
            }),
            v_tab_view: cx.new(|cx| {
                let mut tab_view = TabView::new(tabs.clone(), window, cx);
                tab_view.select_tab_ix(0);
                tab_view.set_orientation(Orientation::Vertical);
                tab_view
            }),
        }
    }
}

impl Render for NavigationTab {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let h_tab_view = container(ContainerStyle::normal(window, cx))
            .w_full()
            .h_64()
            .child(self.h_tab_view.clone());

        let v_tab_view = container(ContainerStyle::normal(window, cx))
            .w_full()
            .h_64()
            .child(self.v_tab_view.clone());

        div()
            .id("nav_tab")
            .track_scroll(&self.scroll_handle)
            .overflow_y_scroll()
            .size_full()
            .p_2()
            .flex()
            .gap_2()
            .child(ui::section("Horizontal TabView").size_full().mb_4().child(h_tab_view))
            .child(ui::section("Vertical TabView").size_full().mb_4().child(v_tab_view))
    }
}

struct TabContent {
    content: SharedString,
}

impl Render for TabContent {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().p_2().child(self.content.clone())
    }
}
