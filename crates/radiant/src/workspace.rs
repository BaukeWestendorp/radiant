use gpui::{
    div, IntoElement, ParentElement, Render, Styled, View, ViewContext, VisualContext,
    WindowContext,
};

use crate::attribute_editor::AttributeEditor;

pub struct Workspace {
    attribute_editor: View<AttributeEditor>,
}

impl Workspace {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| Self {
            attribute_editor: AttributeEditor::build(cx),
        })
    }
}

impl Render for Workspace {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .text_color(gpui::white())
            .child(self.attribute_editor.clone())
    }
}
