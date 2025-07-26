use std::ops::Range;
use std::str::FromStr;

use gpui::prelude::*;
use gpui::{App, EmptyView, Entity, ReadGlobal, SharedString, UpdateGlobal, Window, div};
use radiant::engine::Command;
use radiant::gdtf::attribute::{Feature, FeatureGroup};
use radiant::gdtf::dmx_mode::{ChannelFunction, LogicalChannel};
use radiant::show::{Attribute, AttributeValue, FixtureId, Patch};
use ui::{Disableable, FieldEvent, NumberField, Tab, TabView, button, section, v_divider};

use crate::app::AppState;

const ALL_FEATURE_GROUPS: [&str; 9] =
    ["Dimmer", "Position", "Gobo", "Color", "Beam", "Focus", "Control", "Shapers", "Video"];

pub struct AttributeEditor {
    feature_group_tabs: Entity<TabView>,
}

impl AttributeEditor {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let show = AppState::global(cx).engine.show();

        let fids = show.selected_fixtures();

        let feature_groups =
            feature_groups_for_fids(fids, show.patch()).into_iter().cloned().collect::<Vec<_>>();

        let tabs = ALL_FEATURE_GROUPS
            .into_iter()
            .map(|name| {
                match feature_groups.iter().find(|feature_group| {
                    feature_group
                        .name
                        .as_ref()
                        .is_some_and(|feature_group_name| **feature_group_name == *name)
                }) {
                    Some(feature_group) => {
                        let editor =
                            cx.new(|cx| FeatureGroupEditor::new(feature_group, window, cx));
                        Tab::new(name, name, editor.into())
                    }
                    None => Tab::new(name, name, cx.new(|_| EmptyView).into()).disabled(true),
                }
            })
            .collect();

        Self { feature_group_tabs: cx.new(|cx| TabView::new(tabs, window, cx)) }
    }
}

impl Render for AttributeEditor {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(self.feature_group_tabs.clone())
    }
}

struct FeatureGroupEditor {
    feature_tabs: Entity<TabView>,
}

impl FeatureGroupEditor {
    pub fn new(feature_group: &FeatureGroup, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let tabs = feature_group
            .features
            .iter()
            .map(|feature| {
                let name = feature
                    .name
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or("<unnamed feature>".to_string());
                let editor = cx.new(|cx| FeatureEditor::new(feature, window, cx));
                Tab::new(name.clone(), name, editor.into())
            })
            .collect();
        Self {
            feature_tabs: cx.new(|cx| {
                let mut tab_view = TabView::new(tabs, window, cx);
                tab_view.set_show_tabs(feature_group.features.len() > 1);
                tab_view
            }),
        }
    }
}

impl Render for FeatureGroupEditor {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(self.feature_tabs.clone())
    }
}

struct FeatureEditor {
    channel_tabs: Entity<TabView>,
}

impl FeatureEditor {
    pub fn new(feature: &Feature, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let show = AppState::global(cx).engine.show();
        let fids = show.selected_fixtures();

        let channels = channels_for_fids(fids, &feature, show.patch())
            .into_iter()
            .cloned()
            .collect::<Vec<_>>();

        let tabs = channels
            .into_iter()
            .map(|logical_channel| {
                let name = logical_channel.name().to_string();
                let pretty_name = Attribute::from_str(&name).unwrap().pretty();
                let editor = cx.new(|cx| LogicalChannelEditor::new(&logical_channel, window, cx));
                Tab::new(name, pretty_name, editor.into())
            })
            .collect();
        Self { channel_tabs: cx.new(|cx| TabView::new(tabs, window, cx)) }
    }
}

impl Render for FeatureEditor {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(self.channel_tabs.clone())
    }
}

struct LogicalChannelEditor {
    channel_function_tabs: Entity<TabView>,
}

impl LogicalChannelEditor {
    pub fn new(
        logical_channel: &LogicalChannel,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let tabs = logical_channel
            .channel_functions
            .iter()
            .enumerate()
            .map(|(ix, channel_function)| {
                let name = channel_function
                    .name
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or("<unnamed channel function>".to_string());
                let next_channel_function = logical_channel.channel_functions.get(ix + 1);
                let editor = cx.new(|cx| {
                    ChannelFunctionEditor::new(
                        channel_function.clone(),
                        next_channel_function.cloned(),
                        window,
                        cx,
                    )
                });
                Tab::new(name.clone(), name, editor.into())
            })
            .collect();

        Self {
            channel_function_tabs: cx.new(|cx| {
                let mut tab_view = TabView::new(tabs, window, cx);
                tab_view.set_show_tabs(logical_channel.channel_functions.len() > 1);
                tab_view
            }),
        }
    }
}

impl Render for LogicalChannelEditor {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(self.channel_function_tabs.clone())
    }
}

struct ChannelFunctionEditor {
    channel_function: ChannelFunction,
    next_channel_function: Option<ChannelFunction>,
    function_relative_value_field: Entity<NumberField<f32>>,
}

impl ChannelFunctionEditor {
    pub fn new(
        channel_function: ChannelFunction,
        next_channel_function: Option<ChannelFunction>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let function_relative_value_field = cx.new(|cx| {
            let mut field =
                NumberField::<f32>::new("channel_function_value", cx.focus_handle(), window, cx);
            field.set_value(AttributeValue::from(channel_function.default).as_f32(), cx);
            field.set_min(Some(AttributeValue::MIN), cx);
            field.set_max(Some(AttributeValue::MAX), cx);
            field
        });

        cx.subscribe(&function_relative_value_field, {
            move |this, _, event: &FieldEvent<f32>, cx| match event {
                FieldEvent::Submit(value) => {
                    let fids = AppState::global(cx).engine.show().selected_fixtures().to_vec();
                    for fid in fids {
                        this.set_programmer_attribute(fid, *value, cx);
                    }
                }
                _ => {}
            }
        })
        .detach();

        Self { channel_function, next_channel_function, function_relative_value_field }
    }

    fn set_programmer_attribute(&self, fid: FixtureId, value: f32, cx: &mut App) {
        let engine = &AppState::global(cx).engine;
        let patch = engine.show().patch();

        let Some(fixture) = patch.fixture(fid) else { return };
        let fixture_type = fixture.fixture_type(patch);

        let Some(mut gdtf_attribute) = self.channel_function.attribute(fixture_type) else {
            return;
        };

        if let Some(main_attribute) =
            gdtf_attribute.main_attribute(&fixture_type.attribute_definitions)
        {
            gdtf_attribute = main_attribute;
        }

        let Some(attribute) =
            gdtf_attribute.name.as_ref().and_then(|name| Attribute::from_str(&name).ok())
        else {
            return;
        };

        let value: AttributeValue = self.value_relative_to_function_range(value).into();

        AppState::update_global(cx, |state, _| {
            state
                .engine
                .exec(Command::ProgrammerSetAttribute { fid, attribute: attribute.clone(), value })
                .map_err(|err| log::error!("failed to execute command: {err}"))
                .ok();
        });
    }

    fn value_relative_to_function_range(&self, value: f32) -> f32 {
        let Range { start: fn_from, end: fn_to } = self.function_value_range();
        map(value, 0.0, 1.0, fn_from, fn_to)
    }

    fn function_value_range(&self) -> Range<f32> {
        let dmx_from = AttributeValue::from(self.channel_function.dmx_from).as_f32();
        let dmx_to = self
            .next_channel_function
            .as_ref()
            .map(|ncf| AttributeValue::from(ncf.dmx_from))
            .unwrap_or(AttributeValue::MAX.into())
            .as_f32();
        dmx_from..dmx_to
    }
}

impl Render for ChannelFunctionEditor {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let channel_set_buttons = div().w_full().flex().flex_wrap().gap_2().children(
            self.channel_function
                .channel_sets
                .clone()
                .into_iter()
                .filter(|channel_set| {
                    channel_set.name.as_ref().is_some_and(|name| !name.is_empty())
                })
                .map(|channel_set| {
                    let name: SharedString = channel_set
                        .name
                        .as_ref()
                        .map(ToString::to_string)
                        .unwrap_or("<unnamed channel set>".to_string())
                        .into();

                    button(name.clone(), None, name).on_click(cx.listener(move |this, _, _, cx| {
                        this.function_relative_value_field.update(cx, |field, cx| {
                            let fn_range = this.function_value_range();
                            let value = AttributeValue::from(channel_set.dmx_from).as_f32();
                            let value = map(value, fn_range.start, fn_range.end, 0.0, 1.0);
                            field.set_value(value, cx);
                            field.submit(cx);
                        });
                    }))
                }),
        );

        div()
            .size_full()
            .p_2()
            .flex()
            .gap_2()
            .child(section("Value").size_full().child(self.function_relative_value_field.clone()))
            .child(v_divider(cx))
            .child(section("Channel Sets").min_w_1_3().max_w_1_3().child(channel_set_buttons))
    }
}

fn map(value: f32, a_start: f32, a_end: f32, b_start: f32, b_end: f32) -> f32 {
    if (a_end - a_start).abs() < f32::EPSILON {
        b_start
    } else {
        b_start + ((value - a_start) / (a_end - a_start)) * (b_end - b_start)
    }
}

fn feature_groups_for_fids<'a>(fids: &[FixtureId], patch: &'a Patch) -> Vec<&'a FeatureGroup> {
    let mut feature_groups = Vec::new();
    for fid in fids {
        let Some(fixture) = patch.fixture(*fid) else { continue };
        for feature_group in fixture.feature_groups(patch) {
            if !feature_groups.contains(&feature_group) {
                feature_groups.push(feature_group);
            }
        }
    }
    feature_groups
}

fn channels_for_fids<'a>(
    fids: &[FixtureId],
    feature: &Feature,
    patch: &'a Patch,
) -> Vec<&'a LogicalChannel> {
    let mut channels = Vec::new();
    for fid in fids {
        let Some(fixture) = patch.fixture(*fid) else { continue };
        let fixture_type = fixture.fixture_type(patch);
        let logical_channels = fixture
            .dmx_mode(patch)
            .dmx_channels
            .iter()
            .flat_map(|dmx_channel| &dmx_channel.logical_channels)
            .filter(|logical_channel| {
                logical_channel.attribute(fixture_type).is_some_and(|attribute| {
                    attribute
                        .feature(&fixture_type.attribute_definitions)
                        .is_some_and(|f| f == feature)
                })
            });
        for logical_channel in logical_channels {
            if !channels.contains(&logical_channel) {
                channels.push(logical_channel);
            }
        }
    }
    channels
}
