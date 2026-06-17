use gpui::{
    AnyElement, App, Bounds, Div, ElementId, Entity, FontWeight, MouseButton, SharedString,
    Stateful, Window, div, prelude::*, px,
};
use rd_engine::{
    FixtureCollection,
    cmd::Command,
    event::Event,
    gdtf::{
        Name,
        attr::{Attribute, AttributeName, Feature, PhysicalUnit},
        dmx::DmxValue,
    },
    patch::Fixture,
    pipeline::{AttributeInfo, AttributeSource},
    value::{AttributeValue, ClampedValue},
};
use rd_ui::{ActiveTheme as _, Button, HslaExt as _, TileDelegate, h_flex, v_flex};
use std::collections::HashMap;

use crate::engine::EngineAppExt;

#[derive(Default)]
struct Selection {
    feature: Option<(Name, Name)>,
    encoder_attributes: Vec<Attribute>,
    selected_attribute: Option<usize>,
}

impl Selection {
    pub fn select_feature(&mut self, fg_name: Name, f_name: Name, fixture: &Fixture) {
        self.feature = Some((fg_name.clone(), f_name.clone()));

        let gdtf = fixture.gdtf();
        let dmx_mode = fixture.dmx_mode();

        self.encoder_attributes = dmx_mode
            .attributes(gdtf)
            .filter(|attr| {
                attr.feature_group(gdtf).is_some_and(|fg| fg.name() == &fg_name)
                    && attr.feature(gdtf).is_some_and(|f| f.name() == &f_name)
            })
            .cloned()
            .collect();

        self.selected_attribute = if self.encoder_attributes.is_empty() { None } else { Some(0) };
    }

    pub fn clear_selection(&mut self) {
        self.feature = None;
        self.encoder_attributes.clear();
        self.selected_attribute = None;
    }

    pub fn select_attribute(&mut self, ix: usize) {
        self.selected_attribute = Some(ix);
    }

    pub fn selected_attribute(&self) -> Option<&Attribute> {
        self.selected_attribute.and_then(|ix| self.encoder_attributes.get(ix))
    }
}

pub struct AttributeEditorTile {
    fixture: Entity<Option<Fixture>>,
    selection: Entity<Selection>,
    fg_buckets: Entity<Vec<(Name, Vec<Feature>)>>,
    attribute_values: Entity<HashMap<AttributeName, Option<AttributeInfo>>>,
}

impl AttributeEditorTile {
    pub fn new(_window: &mut Window, cx: &mut App) -> Self {
        let fixture: Entity<Option<Fixture>> = cx.new(|_| None);
        let selection = cx.new(|_| Selection::default());
        let fg_buckets = cx.new(|_| Vec::new());
        let attribute_values = cx.new(|_| HashMap::new());

        cx.on_engine_event({
            let fixture = fixture.clone();
            let attr_selection = selection.clone();
            let fg_buckets = fg_buckets.clone();
            let attribute_values = attribute_values.clone();

            move |event, cx| {
                let snapshot = cx.engine_snapshot();
                let patch = snapshot.patch();
                let selection_state = snapshot.selection();

                match event {
                    Event::SelectionChanged => {
                        let unique = selection_state.unique_dmx_modes(&patch);
                        match unique.len() {
                            0 => {
                                fixture.write(cx, None);
                                attr_selection.update(cx, |sel, cx| {
                                    sel.clear_selection();
                                    cx.notify()
                                });
                                fg_buckets.write(cx, Vec::new());
                            }
                            1 => {
                                if let Some(new_fixture) = selection_state.fixtures(&patch).next().cloned() {
                                    let mut new_fg_buckets: Vec<(Name, Vec<Feature>)> = vec![
                                        (Name::new("Dimmer"), Vec::new()),
                                        (Name::new("Position"), Vec::new()),
                                        (Name::new("Gobo"), Vec::new()),
                                        (Name::new("Color"), Vec::new()),
                                        (Name::new("Beam"), Vec::new()),
                                        (Name::new("Focus"), Vec::new()),
                                        (Name::new("Control"), Vec::new()),
                                        (Name::new("Shapers"), Vec::new()),
                                        (Name::new("Video"), Vec::new()),
                                    ];

                                    let gdtf = new_fixture.gdtf();
                                    let dmx_mode = new_fixture.dmx_mode();
                                    for attribute in dmx_mode.attributes(gdtf) {
                                        if attribute.name() == &AttributeName::NoFeature {
                                            continue;
                                        };

                                        let Some(feature) = attribute.feature(gdtf) else { continue };
                                        let Some(feature_group) = attribute.feature_group(gdtf) else {
                                            continue;
                                        };

                                        let bucket_ix = match feature_group.name().as_str() {
                                            "Dimmer" => 0,
                                            "Position" => 1,
                                            "Gobo" => 2,
                                            "Color" => 3,
                                            "Beam" => 4,
                                            "Focus" => 5,
                                            "Control" => 6,
                                            "Shapers" => 7,
                                            "Video" => 8,
                                            _ => continue,
                                        };

                                        let (_, features) = &mut new_fg_buckets[bucket_ix];
                                        if !features.iter().any(|f| f.name() == feature.name()) {
                                            features.push(feature.clone());
                                        }
                                    }

                                    let first_feature =
                                        new_fg_buckets.iter().find_map(|(fg_name, features)| {
                                            features
                                                .first()
                                                .map(|feature| (fg_name.clone(), feature.name().clone()))
                                        });

                                    attr_selection.update(cx, |selection, cx| {
                                        if let Some((fg_name, f_name)) = first_feature {
                                            selection.select_feature(fg_name, f_name, &new_fixture);
                                        } else {
                                            selection.clear_selection();
                                        }
                                        cx.notify();
                                    });

                                    fixture.write(cx, Some(new_fixture));
                                    fg_buckets.write(cx, new_fg_buckets);
                                } else {
                                    fixture.write(cx, None);
                                    attr_selection.update(cx, |sel, cx| {
                                        sel.clear_selection();
                                        cx.notify()
                                    });
                                    fg_buckets.write(cx, Vec::new());
                                }
                            }
                            _ => {
                                log::warn!(
                                    "Editing multiple GDTF DMX modes simultaneously is not yet supported"
                                );
                                fixture.write(cx, None);
                                attr_selection.update(cx, |sel, cx| {
                                    sel.clear_selection();
                                    cx.notify()
                                });
                                fg_buckets.write(cx, Vec::new());
                            }
                        }
                    }
                    Event::EncoderChanged { encoder_ix, value } => {
                       if  let Some(attribute) = attr_selection.read(cx).encoder_attributes.get(*encoder_ix) {

                           let fixture_ids = cx.engine_snapshot().selection().fixture_ids().to_vec();
                           let value = AttributeValue::Clamped(ClampedValue::new(*value));
                           cx.try_execute_engine_cmd(Command::ProgrammerSet { fixtures: FixtureCollection::Multiple(fixture_ids.clone()), attribute: attribute.name().clone(), value });

                           if let Some(fixture) =
                           selection_state.fixtures(&patch).next() {
                               if let Some(ag) = attribute.activation_group(fixture.gdtf()) {
                                   let activated_attributes = fixture.gdtf().attributes().iter().filter(|attr| attr.activation_group(fixture.gdtf()) == Some(ag));
                                   for attr in activated_attributes {
                                       cx.try_execute_engine_cmd(Command::ProgrammerActivate { fixtures: FixtureCollection::Multiple(fixture_ids.clone()), attribute: attr.name().clone() });
                                   }
                               }
                           }

                        }
                    }
                    _ => (),
                }

                let programmer = snapshot.programmer();
                if let Some(current_fixture) = fixture.read(cx).clone() {
                    let gdtf = current_fixture.gdtf();
                    let dmx_mode = current_fixture.dmx_mode();
                    let mut new_values = HashMap::new();

                    for attribute in dmx_mode.attributes(gdtf) {
                        if attribute.name() == &AttributeName::NoFeature {
                            continue;
                        }

                        let values = selection_state
                            .fixture_ids()
                            .iter()
                            .filter_map(|fixture_id| {
                                Some((
                                    fixture_id,
                                    snapshot.pipeline().attribute_info(
                                        fixture_id,
                                        attribute.name(),
                                        &programmer,
                                    )?,
                                ))
                            })
                            .collect::<Vec<_>>();

                        let value = if values.len() > 0
                            && values.iter().all(|(_, v)| values[0].1.value == v.value)
                        {
                            Some(values[0].1.clone())
                        } else {
                            None
                        };

                        new_values.insert(attribute.name().clone(), value);
                    }

                    attribute_values.update(cx, |values, cx| {
                        if *values != new_values {
                            *values = new_values;
                            cx.notify();
                        }
                    });
                } else {
                    attribute_values.update(cx, |values, cx| {
                        if !values.is_empty() {
                            values.clear();
                            cx.notify();
                        }
                    });
                }
            }
        })
        .detach();

        Self { fixture, selection, fg_buckets, attribute_values }
    }

    fn render_tab_group(
        &self,
        fg_name: &Name,
        features: &[Feature],
        selection: Entity<Selection>,
        window: &Window,
        cx: &App,
    ) -> impl IntoElement {
        let is_selected = selection
            .read(cx)
            .feature
            .as_ref()
            .is_some_and(|(sel_fg_name, _)| sel_fg_name == fg_name);

        let active_fixture = self.fixture.read(cx).clone();

        v_flex()
            .w_full()
            .child(
                self.render_tab_item(
                    fg_name.to_string(),
                    fg_name.to_string(),
                    is_selected,
                    features.is_empty(),
                    window,
                    cx,
                )
                .on_click({
                    let selection = selection.clone();
                    let fg_name = fg_name.clone();
                    let first_feature_name = features.first().map(|f| f.name().clone());
                    let active_fixture = active_fixture.clone();
                    move |_, _, cx| {
                        if let (Some(first_f), Some(fixture)) =
                            (first_feature_name.clone(), active_fixture.clone())
                        {
                            selection.update(cx, |selection, cx| {
                                selection.select_feature(fg_name.clone(), first_f, &fixture);
                                cx.notify();
                            });
                        }
                    }
                }),
            )
            .child(div().flex().gap_px().w_full().children(features.iter().map(|feature| {
                let is_selected =
                    selection.read(cx).feature.as_ref().is_some_and(|(sel_fg_name, sel_f_name)| {
                        sel_fg_name == fg_name && sel_f_name == feature.name()
                    });

                let active_fixture = active_fixture.clone();
                self.render_tab_item(
                    format!("{}-{}", fg_name, feature.name()),
                    feature.name().to_string(),
                    is_selected,
                    false,
                    window,
                    cx,
                )
                .on_click({
                    let selection = selection.clone();
                    let fg_name = fg_name.clone();
                    let f_name = feature.name().clone();
                    move |_, _, cx| {
                        if let Some(fixture) = active_fixture.clone() {
                            selection.update(cx, |selection, cx| {
                                selection.select_feature(fg_name.clone(), f_name.clone(), &fixture);
                                cx.notify();
                            })
                        }
                    }
                })
            })))
    }

    fn render_tab_item(
        &self,
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        selected: bool,
        disabled: bool,
        _window: &Window,
        cx: &App,
    ) -> Stateful<Div> {
        let (bg, border) = if selected {
            (cx.theme().bg_selected, cx.theme().border_selected)
        } else {
            (cx.theme().bg_secondary, cx.theme().border_secondary)
        };

        div()
            .id(id)
            .w_full()
            .text_center()
            .bg(bg)
            .border_b_1()
            .border_color(border)
            .when(cx.theme().shadow, |e| e.shadow_xs())
            .when(disabled, |e| e.text_color(cx.theme().fg_tertiary))
            .when(!disabled, |e| {
                e.hover(|e| e.bg(bg.hover()))
                    .active(|e| e.bg(bg.active()).top(cx.theme().button_depression))
            })
            .child(label.into())
    }

    fn render_feature_contents(
        &self,
        fixture: &Fixture,
        window: &Window,
        cx: &App,
    ) -> impl IntoElement {
        let current_attribute_values = self.attribute_values.read(cx).clone();

        let encoders = div().size_full().flex().children(
            self.selection.read(cx).encoder_attributes.iter().enumerate().map(|(ix, attribute)| {
                let value = current_attribute_values.get(attribute.name()).cloned().flatten();

                div().w(px(80.0) * 2).h_full().child(
                    div()
                        .size_full()
                        .p_1()
                        .child(self.render_encoder(fixture, attribute, value, ix, window, cx)),
                )
            }),
        );

        let channel_functions = self.render_channel_sets(fixture, window, cx);

        div().flex().size_full().child(encoders).child(channel_functions).into_any_element()
    }

    fn render_encoder(
        &self,
        fixture: &Fixture,
        attribute: &Attribute,
        value: Option<AttributeInfo>,
        ix: usize,
        window: &Window,
        cx: &App,
    ) -> impl IntoElement {
        let border = match self.selection.read(cx).selected_attribute().as_ref() {
            Some(sel_attr) if sel_attr.name() == attribute.name() => cx.theme().border_selected,
            _ => cx.theme().border_primary,
        };

        let header = h_flex()
            .px_2()
            .bg(cx.theme().bg_tertiary)
            .border_b_1()
            .border_color(border)
            .font_weight(FontWeight::BOLD)
            .child(attribute.pretty_name().to_string());

        div()
            .size_full()
            .bg(cx.theme().bg_secondary)
            .border_1()
            .border_color(border)
            .child(header)
            .child(self.render_encoder_content(fixture, attribute, value, window, cx))
            .on_mouse_down(MouseButton::Left, {
                let selection = self.selection.clone();
                move |_, _, cx| {
                    selection.update(cx, |selection, cx| {
                        selection.select_attribute(ix);
                        cx.notify();
                    });
                }
            })
    }

    fn render_encoder_content(
        &self,
        fixture: &Fixture,
        attribute: &Attribute,
        value: Option<AttributeInfo>,
        window: &Window,
        cx: &App,
    ) -> impl IntoElement {
        let value_indicator = self.render_value_indicator(value, fixture, attribute, window, cx);

        v_flex().p_1().size_full().child(value_indicator)
    }

    fn render_value_indicator(
        &self,
        value: Option<AttributeInfo>,
        fixture: &Fixture,
        attribute: &Attribute,
        window: &Window,
        cx: &App,
    ) -> impl IntoElement {
        let value_indicator_base =
            h_flex().justify_between().w_full().h(window.line_height()).px_1();
        match value {
            Some(AttributeInfo { value, source }) => {
                let (border, bg) = match source {
                    AttributeSource::Default => (cx.theme().border_primary, cx.theme().bg_primary),
                    AttributeSource::Playback => {
                        (cx.theme().indicate.playback, cx.theme().indicate.playback.opacity(0.2))
                    }
                    AttributeSource::Programmer => (
                        cx.theme().indicate.programmer,
                        cx.theme().indicate.programmer.opacity(0.2),
                    ),
                    AttributeSource::Highlight => {
                        (cx.theme().indicate.highlight, cx.theme().indicate.highlight.opacity(0.2))
                    }
                };

                let current_cs =
                    fixture.dmx_mode().logical_channel(attribute.name()).and_then(|lc| {
                        lc.channel_set_for_value(DmxValue::from_u32(
                            (value.as_f32() * u32::MAX as f32) as u32,
                            false,
                        ))
                    });

                let value = cx
                    .engine_snapshot()
                    .pipeline()
                    .cache()
                    .get(&fixture.id(), attribute.name())
                    .map(|cf| value.to_physical_value(cf.min, cf.max))
                    .unwrap_or(value.as_f32());

                let value_text = match attribute.physical_unit() {
                    PhysicalUnit::None | PhysicalUnit::ColorComponent => {
                        format!("{:.0}%", value * 100.0)
                    }
                    unit => unit.format_value(value),
                };
                let channel_set_label = current_cs.and_then(|cs| {
                    cs.name().map(|name| {
                        div().text_color(cx.theme().fg_secondary).child(name.to_string())
                    })
                });

                // FIXME: Something still is a bit sluggish when updating the value with an encoder,
                //        especially when sending many programmer changes.
                value_indicator_base
                    .border_1()
                    .border_color(border)
                    .bg(bg)
                    .child(value_text)
                    .children(channel_set_label)
            }
            None => value_indicator_base
                .border_1()
                .border_color(cx.theme().border_tertiary)
                .bg(cx.theme().bg_tertiary),
        }
    }

    fn render_channel_sets(
        &self,
        fixture: &Fixture,
        _window: &Window,
        cx: &App,
    ) -> impl IntoElement {
        let mut channel_sets = Vec::new();

        if let Some(attribute) = &self.selection.read(cx).selected_attribute() {
            if let Some(lc) = fixture.dmx_mode().logical_channel(attribute.name()) {
                channel_sets.extend(
                    lc.channel_functions()
                        .iter()
                        .flat_map(|cf| cf.channel_sets())
                        .enumerate()
                        .filter_map(|(ix, cs)| cs.name().map(|name| (ix, name.clone(), cs)))
                        .map(|(ix, name, cs)| {
                            Button::new(("cs", ix)).child(name.to_string()).on_click({
                                let attribute_name = attribute.name().clone();
                                let value =
                                    AttributeValue::Clamped(ClampedValue::from(cs.dmx_from()));
                                move |_, _, cx| {
                                    let fixtures =
                                        cx.engine_snapshot().selection().fixture_ids().to_vec();

                                    cx.execute_engine_cmd(Command::ProgrammerSet {
                                        fixtures: FixtureCollection::Multiple(fixtures),
                                        attribute: attribute_name.clone(),
                                        value,
                                    });
                                }
                            })
                        }),
                );
            }
        }

        div()
            .size_full()
            .border_l_1()
            .border_color(cx.theme().border_primary)
            .child(h_flex().flex_wrap().gap_1().w_full().p_1().children(channel_sets))
    }
}

impl TileDelegate for AttributeEditorTile {
    fn title(&self, _cx: &App) -> SharedString {
        "Attribute Editor".into()
    }

    fn render_content(&self, _bounds: Bounds<u32>, window: &mut Window, cx: &App) -> AnyElement {
        let Some(fixture) = self.fixture.read(cx) else {
            return gpui::Empty.into_any_element();
        };

        let feature_selector = div().flex().gap_px().w_full().children(
            self.fg_buckets.read(cx).iter().map(|(fg_name, features)| {
                self.render_tab_group(fg_name, features, self.selection.clone(), window, cx)
            }),
        );

        let feature_contents = self.render_feature_contents(fixture, window, cx);

        div()
            .flex()
            .flex_col()
            .size_full()
            .text_sm()
            .child(feature_selector)
            .child(feature_contents)
            .into_any_element()
    }
}
