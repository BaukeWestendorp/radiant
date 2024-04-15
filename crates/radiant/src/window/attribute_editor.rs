use std::rc::Rc;

use gdtf::Attribute;
use gpui::{
    div, Context, FlexDirection, IntoElement, Model, ParentElement, Render, SharedString, Styled,
    View, ViewContext, VisualContext, WindowContext,
};
use itertools::Itertools;
use theme::ActiveTheme;
use ui::{
    selector::Selector,
    slider::{Slider, SliderDelegate},
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
    selected_feature_group: Model<Option<String>>,
    feature_group_editor: View<FeatureGroupEditor>,
}

impl AttributeEditorWindow {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let selected_feature_group = cx.new_model(|_cx| Option::<String>::None);

            Self {
                feature_group_editor: FeatureGroupEditor::build(selected_feature_group.clone(), cx),
                selected_feature_group,
            }
        })
    }
}

impl Render for AttributeEditorWindow {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let predefined: Vec<SharedString> = vec![
            "Dimmer".into(),
            "Color".into(),
            "Position".into(),
            "Beam".into(),
            "Focus".into(),
            "Gobo".into(),
        ];

        let feature_group_names = ShowfileManager::show(cx)
            .feature_groups_in_selected_fixtures()
            .iter()
            .unique_by(|fg| fg.name.clone())
            .map(|fg| fg.name.clone())
            .sorted_by_key(|fg| predefined.iter().position(|p| p == fg))
            .filter(|fg| predefined.iter().any(|p| p == fg))
            .collect();

        let feature_group_selector = Selector::new(
            FlexDirection::Column,
            predefined,
            feature_group_names,
            self.selected_feature_group.clone(),
        );

        div()
            .size_full()
            .flex()
            .child(
                div()
                    .p_2()
                    .border_r()
                    .border_color(cx.theme().colors().border)
                    .child(feature_group_selector),
            )
            .child(div().size_full().child(self.feature_group_editor.clone()))
    }
}

struct FeatureGroupEditor {
    feature_group: Model<Option<String>>,
    selected_feature: Model<Option<String>>,
    attribute_editor: View<AttributeEditor>,
}

impl FeatureGroupEditor {
    pub fn build(feature_group: Model<Option<String>>, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let selected_feature = cx.new_model(|_cx| Option::<String>::None);

            let attributes = {
                let attributes =
                    get_attributes(feature_group.clone(), selected_feature.clone(), cx);
                cx.new_model(|_cx| attributes)
            };

            // FIXME: These are quite clunky.
            cx.observe(&selected_feature, {
                let attributes = attributes.clone();
                move |this: &mut Self, selected_feature, cx| {
                    if selected_feature.read(cx).is_some() {
                        let new_attributes = get_attributes(
                            this.feature_group.clone(),
                            selected_feature.clone(),
                            cx,
                        );
                        attributes.update(cx, |attributes, cx| {
                            *attributes = new_attributes;
                            cx.notify();
                        });
                        cx.notify();
                    }
                }
            })
            .detach();

            // FIXME: These are quite clunky.
            cx.observe(&feature_group, {
                let selected_feature = selected_feature.clone();
                let attributes = attributes.clone();
                move |_this: &mut Self, feature_group, cx| {
                    if feature_group.read(cx).is_some() {
                        let new_attributes =
                            get_attributes(feature_group.clone(), selected_feature.clone(), cx);
                        attributes.update(cx, |attributes, cx| {
                            *attributes = new_attributes;
                            cx.notify();
                        });
                        cx.notify();
                    }
                }
            })
            .detach();

            Self {
                feature_group,
                selected_feature,
                attribute_editor: AttributeEditor::build(attributes, cx),
            }
        })
    }

    pub fn render_feature_selector(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let features = ShowfileManager::show(cx)
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
                    .find(|fg| Some(&fg.name) == self.feature_group.read(cx).as_ref())
                    .map(|fg| fg.features.clone())
            })
            .flatten()
            .unique()
            .collect::<Vec<_>>();

        let feature_names = features.iter().map(|f| f.name.clone()).collect::<Vec<_>>();

        div().h_full().child(Selector::new(
            FlexDirection::Row,
            feature_names
                .clone()
                .into_iter()
                .map(|f| f.into())
                .collect(),
            feature_names,
            self.selected_feature.clone(),
        ))
    }
}

fn get_attributes(
    feature_group: Model<Option<String>>,
    feature: Model<Option<String>>,
    cx: &mut WindowContext,
) -> Vec<Rc<Attribute>> {
    match feature.read(cx).clone() {
        Some(feature) => match feature_group.read(cx).clone() {
            Some(feature_group) => get_attributes_from_feature(&feature_group, &feature, cx),
            None => vec![],
        },
        _ => vec![],
    }
}

fn get_attributes_from_feature(
    feature_group: &str,
    feature: &str,
    cx: &mut WindowContext,
) -> Vec<Rc<Attribute>> {
    let mut attributes = vec![];
    for fixture_id in ShowfileManager::show(cx).selected_fixtures().iter() {
        let Some(fixture) = ShowfileManager::show(cx).fixture(*fixture_id) else {
            continue;
        };

        for attribute in fixture
            .attributes_for_feature(feature_group, feature)
            .into_iter()
        {
            if attribute.main_attribute.is_some() {
                continue;
                // FIXME: We should do something with secondary attributes.
            }

            if !attributes
                .iter()
                .any(|a: &Rc<Attribute>| a.name == attribute.name)
            {
                attributes.push(attribute.clone());
            }
        }
    }
    attributes
}

impl Render for FeatureGroupEditor {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let feature_selector = div().p_2().child(self.render_feature_selector(cx));

        div()
            .size_full()
            .flex()
            .flex_col()
            .child(
                div()
                    .w_full()
                    .border_b()
                    .border_color(cx.theme().colors().border)
                    .child(feature_selector),
            )
            .child(self.attribute_editor.clone())
    }
}

pub struct AttributeEditor {
    attribute_values: Vec<Model<f32>>,
    sliders: Vec<View<Slider<AttributeSliderDelegate>>>,
}

impl AttributeEditor {
    pub fn build(attributes: Model<Vec<Rc<Attribute>>>, cx: &mut WindowContext) -> View<Self> {
        let get_attribute_values =
            |attributes: Model<Vec<Rc<Attribute>>>, cx: &mut WindowContext| -> Vec<Model<f32>> {
                attributes
                    .read(cx)
                    .clone()
                    .into_iter()
                    .map(|attribute| cx.new_model(|_cx| 0.5))
                    .collect()
            };

        let get_sliders = |values: &Vec<Model<f32>>,
                           attributes: Model<Vec<Rc<Attribute>>>,
                           cx: &mut WindowContext| {
            attributes
                .read(cx)
                .clone()
                .iter_mut()
                .enumerate()
                .map(|(ix, attribute)| {
                    cx.new_view(|_cx| {
                        Slider::new(
                            &attribute.name,
                            AttributeSliderDelegate::new(attribute.clone()),
                            values[ix].clone(),
                        )
                    })
                })
                .collect()
        };

        cx.new_view(|cx| {
            cx.observe(&attributes, move |this: &mut Self, attributes, cx| {
                this.attribute_values = get_attribute_values(attributes.clone(), cx);
                this.sliders = get_sliders(&this.attribute_values, attributes.clone(), cx);
                cx.notify();
            })
            .detach();

            let attribute_values = get_attribute_values(attributes.clone(), cx);
            Self {
                attribute_values: attribute_values.clone(),
                sliders: get_sliders(&attribute_values, attributes, cx),
            }
        })
    }
}

impl Render for AttributeEditor {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .p_2()
            .flex()
            .gap_2()
            .children(self.sliders.clone().into_iter().map(|slider| {
                let attribute = slider.read(cx).delegate.attribute.clone();

                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .child(div().w_10().h_full().child(slider.clone()))
                    .child(
                        attribute
                            .pretty_name
                            .clone()
                            .unwrap_or(attribute.name.clone()),
                    )
            }))
    }
}

struct AttributeSliderDelegate {
    pub attribute: Rc<Attribute>,
}

impl AttributeSliderDelegate {
    pub fn new(attribute: Rc<Attribute>) -> Self {
        Self { attribute }
    }
}

impl SliderDelegate for AttributeSliderDelegate {}
