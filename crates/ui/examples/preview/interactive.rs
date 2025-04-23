use gpui::{ScrollHandle, Window, div, prelude::*};

pub struct InteractiveTab {
    scroll_handle: ScrollHandle,
}

impl InteractiveTab {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self { scroll_handle: ScrollHandle::new() }
    }
}

impl Render for InteractiveTab {
    fn render(&mut self, _w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("typography-tab")
            .track_scroll(&self.scroll_handle)
            .overflow_y_scroll()
            .size_full()
            .m_2()
    }
}
