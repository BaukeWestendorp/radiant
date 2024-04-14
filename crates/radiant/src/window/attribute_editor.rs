use gpui::{
    div, IntoElement, ParentElement, Render, SharedString, Styled, View, ViewContext,
    VisualContext, WindowContext,
};
use theme::ActiveTheme;
use ui::{
    button::{Button, ButtonStyle},
    disableable::Disableable,
};

use crate::showfile::ShowfileManager;

use super::{WindowDelegate, WindowView};

pub struct AttributeEditorWindowDelegate {
    attribute_editor: View<AttributeEditorWindow>,
}

impl AttributeEditorWindowDelegate {
    pub fn new(cx: &mut WindowContext) -> Self {
        Self {
            attribute_editor: AttributeEditorWindow::build(cx),
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
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| Self {})
    }

    pub fn render_feature_group_selector(&self, cx: &mut WindowContext) -> impl IntoElement {
        let feature_groups = ShowfileManager::show(cx).feature_groups_in_selected_fixtures();
        let feature_group_names = vec!["Dimmer", "Position", "Color", "Beam", "Focus", "Gobo"];

        let buttons = feature_group_names
            .iter()
            .map(|name| {
                Button::new(ButtonStyle::Secondary, *name, cx)
                    .w_full()
                    .px_2()
                    .py_1()
                    .disabled(!feature_groups.iter().any(|fg| fg.name == *name))
                    .child(name.to_string())
            })
            .collect::<Vec<_>>();

        div()
            .w_32()
            .h_full()
            .flex()
            .flex_col()
            .gap_2()
            .children(buttons)
    }
}

impl Render for AttributeEditorWindow {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let feature_group_selector = div()
            .p_2()
            .border_r()
            .border_color(cx.theme().colors().border)
            .child(self.render_feature_group_selector(cx));

        let feature_group_editor = div().size_full();

        div()
            .size_full()
            .flex()
            .gap_2()
            .child(feature_group_selector)
            .child(feature_group_editor)
    }
}
