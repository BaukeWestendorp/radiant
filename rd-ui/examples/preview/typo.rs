use gpui::prelude::*;
use gpui::{Window, div};
use rd_ui::{article, h1, h2, h3, h4, h5, h6, link, sub};

pub struct TypoPreview {}

impl TypoPreview {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {}
    }
}

impl Render for TypoPreview {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().p_2().size_full().child(article().children([
            h1("Header 1", cx),
            h2("Header 2", cx),
            h3("Header 3", cx),
            h4("Header 4", cx),
            h5("Header 5", cx),
            h6("Header 6", cx),
            link("Click Here for a cool website!", "https://baukewestendorp.nl", cx),
            sub("Subtext", cx),
        ]))
    }
}
