use rd_ui::{
    comp::{
        IconVariant, Labelled,
        stateful::{Tab, Tabs, TabsDirection},
    },
    gpui::{Entity, Window, div, prelude::*},
};

mod midi;

pub struct TriggersTabView {
    tabs: Entity<Tabs>,
}

impl TriggersTabView {
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
                    .with_tab(
                        Tab::new("MIDI", cx).with_icon(IconVariant::KeyboardMusic).with_content(
                            cx.new(|cx| {
                                midi::MidiTabView::new(uncommitted_project.clone(), window, cx)
                            }),
                            cx,
                        ),
                    )
            }),
        }
    }
}

impl Render for TriggersTabView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(self.tabs.clone())
    }
}
