use std::rc::Rc;

use backstage::show::{AttributeValue, FixtureId};
use gdtf::{Attribute, Feature};
use gpui::{
    div, Context, Global, IntoElement, ParentElement, Render, SharedString, Styled, View,
    ViewContext, VisualContext, WindowContext,
};

use crate::{
    showfile::Showfile,
    ui::{Picker, Slider},
};

pub struct AttributeEditor {
    feature_group_picker: View<Picker>,
    feature_picker: Option<View<Picker>>,
    attribute_sliders: Vec<View<AttributeSlider>>,
}

impl AttributeEditor {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        let feature_groups = vec![
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

        cx.new_view(|cx| {
            let selected_feature_group = cx.new_model(|_cx| None);

            cx.observe(&selected_feature_group, {
                let feature_groups = feature_groups.clone();
                move |this: &mut Self, selected_feature_group, cx| {
                    this.attribute_sliders = Vec::new();

                    let Some(feature_group): Option<&SharedString> = selected_feature_group
                        .read(cx)
                        .and_then(|ix| feature_groups.get(ix))
                    else {
                        this.feature_picker = None;
                        return;
                    };

                    Self::update_feature_picker(this, &feature_group, cx);
                }
            })
            .detach();

            Self {
                feature_group_picker: Picker::build(feature_groups, selected_feature_group, cx),
                feature_picker: None,
                attribute_sliders: Vec::new(),
            }
        })
    }

    fn update_feature_picker(&mut self, feature_group: &SharedString, cx: &mut ViewContext<Self>) {
        let mut features: Vec<Rc<Feature>> = Vec::new();

        for fixture in Showfile::get(cx).show.selected_fixtures().iter() {
            for feature in fixture
                .feature_group(feature_group)
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
            self.feature_picker = None;
            cx.notify();
            return;
        }

        let selected_feature = cx.new_model(|_cx| None);

        cx.observe(&selected_feature, {
            let features = features.clone();
            move |this: &mut Self, selected_feature, cx| {
                let Some(feature): Option<&Rc<Feature>> =
                    selected_feature.read(cx).and_then(|ix| features.get(ix))
                else {
                    cx.notify();
                    return;
                };

                Self::update_attribute_sliders(this, &feature, cx)
            }
        })
        .detach();

        let feature_names = features.iter().map(|fg| fg.name.clone().into()).collect();

        self.feature_picker = Some(Picker::build(feature_names, selected_feature, cx));
        cx.notify();
    }

    fn update_attribute_sliders(&mut self, feature: &Rc<Feature>, cx: &mut ViewContext<Self>) {
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

        self.attribute_sliders = attributes
            .into_iter()
            .map(|attribute| AttributeSlider::build(attribute, cx))
            .collect();
        cx.notify();
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
