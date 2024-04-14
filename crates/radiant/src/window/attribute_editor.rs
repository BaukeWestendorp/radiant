use gpui::{
    div, IntoElement, Model, ParentElement, Render, SharedString, Styled, View, ViewContext,
    VisualContext, WindowContext,
};

use crate::showfile::Window;

use super::{WindowDelegate, WindowView};

pub struct AttributeEditorWindowDelegate {
    attribute_editor: View<AttributeEditorWindow>,
}

impl AttributeEditorWindowDelegate {
    pub fn new(cx: &mut WindowContext, window: Model<Window>) -> Self {
        Self {
            attribute_editor: AttributeEditorWindow::build(cx, window),
        }
    }
}

impl WindowDelegate for AttributeEditorWindowDelegate {
    fn title(&mut self, _cx: &mut ViewContext<WindowView<Self>>) -> Option<SharedString> {
        Some("Attribute Editor".into())
    }

    fn render_content(&mut self, _cx: &mut ViewContext<WindowView<Self>>) -> impl IntoElement {
        div().size_full().child(self.attribute_editor.clone())
    }
}

pub struct AttributeEditorWindow {}

impl AttributeEditorWindow {
    pub fn build(cx: &mut WindowContext, window: Model<Window>) -> View<Self> {
        cx.new_view(|_cx| Self {})
    }
}

impl Render for AttributeEditorWindow {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        let content = div().size_full().child("content");

        div().size_full().flex().flex_col().gap_1().child(content)
    }
}
