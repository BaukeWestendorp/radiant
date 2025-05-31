use gpui::{ScrollHandle, Window, div, prelude::*};

pub struct TypographyTab {
    scroll_handle: ScrollHandle,
}

impl TypographyTab {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self { scroll_handle: ScrollHandle::new() }
    }
}

impl Render for TypographyTab {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let headers = div()
            .child(ui::h1("Header Level 1"))
            .child(ui::h2("Header Level 2"))
            .child(ui::h3("Header Level 3"))
            .child(ui::h4("Header Level 4"))
            .child(ui::h5("Header Level 5"))
            .child(ui::h6("Header Level 6"));

        let paragraphs = div()
            .child(ui::p("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."));

        let links =
            div().child(ui::link("example-link", "https://example.com", "Example Link", cx));

        div()
            .id("typography-tab")
            .track_scroll(&self.scroll_handle)
            .overflow_y_scroll()
            .size_full()
            .p_2()
            .child(ui::section("Headers").mb_4().child(headers))
            .child(ui::section("Paragraphs").mb_4().child(paragraphs))
            .child(ui::section("Links").mb_4().child(links))
    }
}
