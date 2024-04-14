use gpui::{
    div, prelude::FluentBuilder, ElementId, IntoElement, ParentElement, Render, SharedString,
    Styled, View, ViewContext, VisualContext, WindowContext,
};
use itertools::Itertools;
use theme::ActiveTheme;
use ui::{
    button::{Button, ButtonStyle},
    disableable::Disableable,
    selectable::Selectable,
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

pub struct AttributeEditorWindow {
    selected_feature_group: Option<String>,
    selected_feature: Option<String>,
}

impl AttributeEditorWindow {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| Self {
            selected_feature_group: None,
            selected_feature: None,
        })
    }

    pub fn render_feature_group_selector(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let feature_groups = ShowfileManager::show(cx).feature_groups_in_selected_fixtures();
        let feature_group_names = vec!["Dimmer", "Position", "Color", "Beam", "Focus", "Gobo"];

        let buttons = feature_group_names
            .iter()
            .map(|name| {
                let matched_feature = feature_groups.iter().any(|fg| fg.name == *name);
                Button::new(
                    ButtonStyle::Secondary,
                    ElementId::Name(format!("FeatureGroup_{name}").into()),
                    cx,
                )
                .w_full()
                .px_2()
                .py_1()
                .disabled(!matched_feature)
                .selected(self.selected_feature_group == Some(name.to_string()))
                .on_click({
                    let name = name.to_string();
                    cx.listener(move |this, _event, _cx| {
                        this.selected_feature_group = Some(name.clone())
                    })
                })
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

    pub fn render_feature_group_editor(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let feature_selector = div().p_2().child(self.render_feature_selector(cx));

        div()
            .flex()
            .flex_col()
            .when(self.selected_feature_group.is_some(), |this| {
                this.child(
                    div()
                        .w_full()
                        .border_b()
                        .border_color(cx.theme().colors().border)
                        .child(feature_selector),
                )
            })
            .child(div().p_2().size_full())
    }

    pub fn render_feature_selector(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let features = self
            .selected_feature_group
            .as_ref()
            .map(|selected_feature_group| {
                ShowfileManager::show(cx)
                    .selected_fixtures()
                    .iter()
                    .filter_map(|fixture_id| ShowfileManager::show(cx).fixture(*fixture_id))
                    .filter_map(|fixture| {
                        fixture
                            .description
                            .fixture_type
                            .attribute_definitions
                            .feature_groups
                            .iter()
                            .find(|fg| fg.name == *selected_feature_group)
                            .map(|fg| fg.features.clone())
                    })
                    .flatten()
                    .unique()
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        let feature_buttons = features.iter().map(|feature| {
            Button::new(
                ButtonStyle::Secondary,
                ElementId::Name(format!("Feature_{}", feature.name).into()),
                cx,
            )
            .w_full()
            .px_2()
            .py_1()
            .on_click({
                let name = feature.name.clone();
                cx.listener(move |this, _event, _cx| {
                    this.selected_feature = Some(name.clone());
                })
            })
            .child(feature.name.clone())
        });

        div().h_full().flex().gap_2().children(feature_buttons)
    }
}

impl Render for AttributeEditorWindow {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let feature_group_selector = div()
            .p_2()
            .border_r()
            .border_color(cx.theme().colors().border)
            .child(self.render_feature_group_selector(cx));

        let feature_group_editor = div()
            .size_full()
            .child(self.render_feature_group_editor(cx));

        div()
            .size_full()
            .flex()
            .child(feature_group_selector)
            .child(feature_group_editor)
    }
}
