use std::rc::Rc;

use backstage::show::{AttributeValue, FixtureId};
use gdtf::{Attribute, Feature, FeatureGroup};
use gpui::{
    div, Context, Global, IntoElement, Model, ParentElement, Render, SharedString, Styled, View,
    ViewContext, VisualContext, WindowContext,
};

use crate::{
    showfile::Showfile,
    ui::{Picker, PickerOption, Slider},
};

use super::{WindowDelegate, WindowView};

pub struct AttributeEditorWindowDelegate {
    attribute_editor: View<AttributeEditor>,
}

impl AttributeEditorWindowDelegate {
    pub fn new(selected_fixtures: Model<Vec<FixtureId>>, cx: &mut WindowContext) -> Self {
        Self {
            attribute_editor: AttributeEditor::build(selected_fixtures, cx),
        }
    }
}

impl WindowDelegate for AttributeEditorWindowDelegate {
    fn title(&mut self, _cx: &mut ViewContext<WindowView<Self>>) -> Option<SharedString> {
        Some("Attribute Editor".into())
    }

    fn render_content(&mut self, _cx: &mut ViewContext<WindowView<Self>>) -> impl IntoElement {
        div().size_full().p_2().child(self.attribute_editor.clone())
    }
}

pub struct AttributeEditor {
    feature_group_picker: View<Picker<Rc<FeatureGroup>>>,
    feature_picker: Option<View<Picker<Rc<Feature>>>>,
    attribute_sliders: Vec<View<AttributeSlider>>,
    selected_feature_group: Model<Option<PickerOption<Rc<FeatureGroup>>>>,
    selected_feature: Model<Option<PickerOption<Rc<Feature>>>>,
}

impl AttributeEditor {
    pub fn build(selected_fixtures: Model<Vec<FixtureId>>, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let selected_feature_group = cx.new_model(|_cx| None);
            let selected_feature = cx.new_model(|_cx| None);

            cx.observe(&selected_fixtures, {
                let selected_feature_group = selected_feature_group.clone();
                move |this: &mut Self, selected_fixtures, cx| {
                    this.attribute_sliders = Vec::new();
                    this.selected_feature.update(cx, |selected_feature, cx| {
                        *selected_feature = None;
                        cx.notify();
                    });
                    this.selected_feature_group
                        .update(cx, |selected_feature_group, cx| {
                            *selected_feature_group = None;
                            cx.notify();
                        });

                    let picker = Self::get_feature_group_picker(
                        &selected_feature_group,
                        selected_fixtures,
                        cx,
                    );
                    this.feature_group_picker = picker;
                    cx.notify();
                }
            })
            .detach();

            cx.observe(&selected_feature_group, {
                {
                    let selected_feature = selected_feature.clone();
                    let selected_fixtures = selected_fixtures.clone();
                    move |this: &mut Self, selected_feature_group, cx| {
                        this.attribute_sliders = Vec::new();
                        this.selected_feature.update(cx, |selected_feature, _cx| {
                            *selected_feature = None;
                        });

                        let Some(feature_group) = selected_feature_group
                            .read(cx)
                            .as_ref()
                            .and_then(|sfg| sfg.value.clone())
                        else {
                            this.feature_picker = None;
                            return;
                        };

                        let feature_picker = Self::get_feature_picker(
                            selected_feature.clone(),
                            &selected_fixtures,
                            &feature_group,
                            cx,
                        );
                        this.feature_picker = feature_picker;
                        cx.notify();
                    }
                }
            })
            .detach();

            cx.observe(&selected_feature, {
                move |this: &mut Self, selected_feature, cx| {
                    let Some(feature) = selected_feature
                        .read(cx)
                        .as_ref()
                        .and_then(|sf| sf.value.clone())
                    else {
                        return;
                    };

                    let sliders = Self::get_attribute_sliders(this, &feature, cx);
                    this.attribute_sliders = sliders;
                    cx.notify();
                }
            })
            .detach();

            Self {
                feature_group_picker: Self::get_feature_group_picker(
                    &selected_feature_group,
                    selected_fixtures,
                    cx,
                ),
                feature_picker: None,
                attribute_sliders: Vec::new(),
                selected_feature_group,
                selected_feature,
            }
        })
    }

    fn get_feature_group_picker(
        selected_feature_group: &Model<Option<PickerOption<Rc<FeatureGroup>>>>,
        selected_fixtures: Model<Vec<FixtureId>>,
        cx: &mut ViewContext<Self>,
    ) -> View<Picker<Rc<FeatureGroup>>> {
        // FIXME: We should not create this list every time we get the feature group picker.
        let all_feature_groups: Vec<SharedString> = vec![
            "Dimmer".into(),
            "Color".into(),
            "Position".into(),
            "Beam".into(),
            "Gobo".into(),
            "Focus".into(),
            "Control".into(),
            "Shapers".into(),
            "Video".into(),
        ];

        let mut feature_groups = Vec::<Rc<FeatureGroup>>::new();
        for fixture_id in selected_fixtures.read(cx).iter() {
            let fixture = Showfile::get(cx)
                .show
                .patchlist()
                .fixture(fixture_id)
                .unwrap();
            for feature_group in fixture.feature_groups().iter() {
                if !feature_groups
                    .iter()
                    .any(|fg| fg.name == feature_group.name)
                {
                    feature_groups.push(feature_group.clone())
                }
            }
        }

        let picker_options = all_feature_groups
            .clone()
            .into_iter()
            .map(|feature_group_name| PickerOption {
                id: feature_group_name.clone().into(),
                label: feature_group_name.clone(),
                value: feature_groups
                    .iter()
                    .find(|fg| fg.name == feature_group_name)
                    .cloned(),
            })
            .collect();

        Picker::build(picker_options, selected_feature_group.clone(), cx)
    }

    fn get_feature_picker(
        selected_feature: Model<Option<PickerOption<Rc<Feature>>>>,
        selected_fixtures: &Model<Vec<FixtureId>>,
        feature_group: &Rc<FeatureGroup>,
        cx: &mut ViewContext<Self>,
    ) -> Option<View<Picker<Rc<Feature>>>> {
        let mut features: Vec<Rc<Feature>> = Vec::new();

        for fixture_id in selected_fixtures.read(cx).iter() {
            let Some(fixture) = Showfile::get(cx).show.patchlist().fixture(fixture_id) else {
                log::warn!("Failed to get fixture with id {fixture_id}");
                continue;
            };

            for feature in fixture
                .feature_group(&feature_group.name)
                .map(|fg| fg.features.clone())
                .unwrap_or_default()
            {
                if !features
                    .iter()
                    .any(|f: &Rc<Feature>| f.name == feature.name)
                {
                    features.push(feature);
                }
            }
        }

        if features.is_empty() {
            return None;
        }

        let feature_options = features
            .iter()
            .map(|fg| PickerOption {
                id: SharedString::from(fg.name.clone()).into(),
                value: Some(fg.clone()),
                label: fg.name.clone().into(),
            })
            .collect();

        Some(Picker::build(feature_options, selected_feature, cx))
    }

    fn get_attribute_sliders(
        &mut self,
        feature: &Rc<Feature>,
        cx: &mut ViewContext<Self>,
    ) -> Vec<View<AttributeSlider>> {
        let mut attributes = vec![];
        for fixture in Showfile::get(cx).show.selected_fixtures() {
            for attribute in fixture
                .attributes_for_feature_in_current_mode(&feature)
                .into_iter()
            {
                if attributes
                    .iter()
                    .any(|attr: &Rc<Attribute>| attr.name == attribute.name)
                {
                    continue;
                }

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
            .into_iter()
            .map(|attribute| AttributeSlider::build(attribute, cx))
            .collect()
    }
}

impl Render for AttributeEditor {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .gap_2()
            .child(
                div()
                    .w_20()
                    .h_full()
                    .child(self.feature_group_picker.clone()),
            )
            .child(
                div()
                    .size_full()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .children(self.feature_picker.clone())
                    .child(
                        div()
                            .h_full()
                            .flex()
                            .gap_2()
                            .children(self.attribute_sliders.clone()),
                    ),
            )
    }
}

pub struct AttributeSlider {
    slider: View<Slider>,
    attribute: Rc<Attribute>,
}

impl AttributeSlider {
    pub fn build(attribute: Rc<Attribute>, cx: &mut WindowContext) -> View<Self> {
        let slider_value = cx.new_model({
            let attribute_name = attribute.name.clone();
            move |cx| {
                let value = Showfile::get(cx)
                    .show
                    .programmer()
                    .get_attribute_value(&FixtureId::new(101), &attribute_name);

                value.map(|v| v.value()).unwrap_or(0.0)
            }
        });

        cx.observe(&slider_value, {
            let attribute_name = attribute.name.clone();
            move |slider_value, cx| {
                let value = *slider_value.read(cx);
                Showfile::update(cx, |showfile, _cx| {
                    let selected_fixtures = showfile
                        .show
                        .selected_fixtures()
                        .into_iter()
                        .cloned()
                        .collect::<Vec<_>>();

                    for fixture in selected_fixtures {
                        showfile
                            .show
                            .programmer_mut()
                            .set_attribute_value(
                                &fixture,
                                attribute_name.clone(),
                                AttributeValue::new(value),
                            )
                            .ok();
                    }
                });
            }
        })
        .detach();

        cx.new_view(|cx| Self {
            slider: cx.new_view(|_cx| {
                Slider::new(SharedString::from(attribute.name.clone()), slider_value)
            }),
            attribute,
        })
    }
}

impl Render for AttributeSlider {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .w_10()
            .h_full()
            .flex()
            .flex_col()
            .items_center()
            .child(div().size_full().child(self.slider.clone()))
            .child(
                self.attribute
                    .pretty_name
                    .clone()
                    .unwrap_or(self.attribute.name.clone()),
            )
    }
}
