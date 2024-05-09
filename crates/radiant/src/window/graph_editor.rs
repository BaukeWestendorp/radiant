use gpui::{div, IntoElement, ParentElement, SharedString, Styled, ViewContext, WindowContext};

use super::{WindowDelegate, WindowView};

pub struct GraphEditorWindowDelegate {}

impl GraphEditorWindowDelegate {
    pub fn new(_cx: &mut WindowContext) -> Self {
        Self {}
    }
}

impl WindowDelegate for GraphEditorWindowDelegate {
    fn title(&mut self, _cx: &mut ViewContext<WindowView<Self>>) -> Option<SharedString> {
        Some("Graph Editor".into())
    }

    fn render_content(&mut self, _cx: &mut ViewContext<WindowView<Self>>) -> impl IntoElement {
        div().size_full().child("GRAPH EDITOR")
    }
}
