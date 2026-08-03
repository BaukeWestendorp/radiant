use gpui::{Entity, Window, div, prelude::*};
use rd_ui::{IconVariant, Tab, Tabs, TabsState, TabsVariant};

mod midi;

pub struct TriggersTabView {
    tabs: Entity<TabsState>,
    pub(crate) midi_tab: Entity<midi::MidiTabView>,
}

impl TriggersTabView {
    pub fn new(
        uncommitted_project: Entity<rd::Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            tabs: cx.new(|_| TabsState::new().with_selected("midi")),
            midi_tab: cx.new(|cx| midi::MidiTabView::new(uncommitted_project.clone(), window, cx)),
        }
    }
}

impl Render for TriggersTabView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(
            Tabs::new("tabs", self.tabs.clone()).variant(TabsVariant::Top).tabs(vec![
                Tab::new("midi", "MIDI", self.midi_tab.clone().into_any_element())
                    .icon(IconVariant::KeyboardMusic),
            ]),
        )
    }
}
