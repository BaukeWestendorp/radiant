use gpui::{ScrollHandle, Window, div, prelude::*, px};
use ui::{ActiveTheme, ContainerStyle, container, dot_grid, line_grid};

pub struct MiscTab {
    scroll_handle: ScrollHandle,
}

impl MiscTab {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self { scroll_handle: ScrollHandle::new() }
    }
}

impl Render for MiscTab {
    fn render(&mut self, w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let line_grid = container(ContainerStyle::normal(w, cx))
            .w_full()
            .h_64()
            .child(line_grid(px(10.0), cx.theme().colors.grid).size_full());
        let dot_grid = container(ContainerStyle::normal(w, cx))
            .w_full()
            .h_64()
            .child(dot_grid(px(10.0), cx.theme().colors.grid).size_full());

        div()
            .id("misc-tab")
            .track_scroll(&self.scroll_handle)
            .overflow_y_scroll()
            .size_full()
            .p_2()
            .child(ui::section("Line Grid").mb_4().child(line_grid))
            .child(ui::section("Dot Grid").mb_4().child(dot_grid))
    }
}
