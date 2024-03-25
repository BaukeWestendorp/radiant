use gpui::{div, IntoElement, ParentElement, Styled, ViewContext, WindowContext};

use super::{WindowDelegate, WindowView};

pub struct ExecutorsWindowDelegate {}

impl ExecutorsWindowDelegate {
    pub fn new(_cx: &mut WindowContext) -> Self {
        Self {}
    }
}

impl WindowDelegate for ExecutorsWindowDelegate {
    fn title(&self) -> String {
        "Executors".to_string()
    }

    fn render_content(&self, _cx: &mut ViewContext<WindowView<Self>>) -> impl IntoElement {
        div().size_full().child("Executor Window contetn!")
    }
}
