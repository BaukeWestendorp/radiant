use gpui::{
    AnyElement, App, Bounds, Div, ElementId, Entity, FontWeight, SharedString, Stateful, Window,
    div, prelude::*, px,
};
use rd_engine::{
    gdtf::{
        Name,
        attr::{Attribute, AttributeName, Feature},
        dmx::DmxValue,
    },
    patch::Fixture,
    pipeline::{AttributeInfo, AttributeSource},
};
use rd_ui::{ActiveTheme as _, HslaExt as _, TileDelegate, h_flex, v_flex};

use crate::engine::EngineManager;

pub struct AttributeEditorTile {
    fixture: Entity<Option<Fixture>>,
    selected_feature: Entity<Option<(Name, Name)>>,
    fg_buckets: Entity<Vec<(Name, Vec<Feature>)>>,
}

impl AttributeEditorTile {
    pub fn new(_window: &mut Window, cx: &mut App) -> Self {
        let fixture = cx.new(|_| None);
        let selected_feature = cx.new(|_| None);
        let fg_buckets = cx.new(|_| Vec::new());

        cx.observe(&EngineManager::selection(cx).clone(), {
            let fixture = fixture.clone();
            let selected_feature = selected_feature.clone();
            let fg_buckets = fg_buckets.clone();
            move |_, cx| {
                let snapshot = EngineManager::snapshot(cx);
                let unique = snapshot.selection().unique_dmx_modes(snapshot.patch());
                match unique.len() {
                    0 => {
                        fixture.write(cx, None);
                        selected_feature.write(cx, None);
                        fg_buckets.write(cx, Vec::new());
                    }
                    1 => {
                        let Some(new_fixture) =
                            snapshot.selection().fixtures(snapshot.patch()).next().cloned()
                        else {
                            fixture.write(cx, None);
                            selected_feature.write(cx, None);
                            fg_buckets.write(cx, Vec::new());
                            return;
                        };

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
                        selected_feature.write(cx, first_feature);

                        fixture.write(cx, Some(new_fixture));
                        fg_buckets.write(cx, new_fg_buckets);
                    }
                    _ => {
                        log::warn!(
                            "Editing multiple GDTF DMX modes simultaneously is not yet supported"
                        );
                        fixture.write(cx, None);
                        selected_feature.write(cx, None);
                        fg_buckets.write(cx, Vec::new());
                    }
                }
            }
        })
        .detach();

        Self { fixture, selected_feature, fg_buckets }
    }

    fn render_tab_group(
        &self,
        fg_name: &Name,
        features: &[Feature],
        selected_feature: Entity<Option<(Name, Name)>>,
        window: &Window,
        cx: &App,
    ) -> impl IntoElement {
        let is_selected = selected_feature
            .read(cx)
            .as_ref()
            .is_some_and(|(sel_fg_name, _)| sel_fg_name == fg_name);

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
                    let selected_feature = selected_feature.clone();
                    let fg_name = fg_name.clone();
                    let first_feature_name = features.first().map(|f| f.name().clone());
                    move |_, _, cx| {
                        if let Some(first_feature_name) = &first_feature_name {
                            selected_feature
                                .write(cx, Some((fg_name.clone(), first_feature_name.clone())));
                        }
                    }
                }),
            )
            .child(div().flex().gap_px().w_full().children(features.iter().map(|feature| {
                let is_selected =
                    selected_feature.read(cx).as_ref().is_some_and(|(sel_fg_name, sel_f_name)| {
                        sel_fg_name == fg_name && sel_f_name == feature.name()
                    });

                self.render_tab_item(
                    format!("{}-{}", fg_name, feature.name()),
                    feature.name().to_string(),
                    is_selected,
                    false,
                    window,
                    cx,
                )
                .on_click({
                    let selected_feature = selected_feature.clone();
                    let fg_name = fg_name.clone();
                    let f_name = feature.name().clone();
                    move |_, _, cx| {
                        selected_feature.write(cx, Some((fg_name.clone(), f_name.clone())));
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
        let Some((fg_name, f_name)) = self.selected_feature.read(cx) else {
            return gpui::Empty.into_any_element();
        };

        let gdtf = fixture.gdtf();
        let dmx_mode = fixture.dmx_mode();

        let attributes = dmx_mode.attributes(gdtf).filter(|attr| {
            attr.feature_group(gdtf).is_some_and(|fg| fg.name() == fg_name)
                && attr.feature(gdtf).is_some_and(|f| f.name() == f_name)
        });

        let encoders = div().size_full().flex().children(attributes.map(|attribute| {
            div().w(px(80.0) * 2).h_full().child(
                div().size_full().p_1().child(self.render_encoder(fixture, attribute, window, cx)),
            )
        }));

        div().size_full().child(encoders).into_any_element()
    }

    fn render_encoder(
        &self,
        fixture: &Fixture,
        attribute: &Attribute,
        window: &Window,
        cx: &App,
    ) -> impl IntoElement {
        let header = h_flex()
            .px_2()
            .bg(cx.theme().bg_tertiary)
            .border_b_1()
            .border_color(cx.theme().border_primary)
            .font_weight(FontWeight::BOLD)
            .child(attribute.pretty_name().to_string());

        div()
            .size_full()
            .bg(cx.theme().bg_secondary)
            .border_1()
            .border_color(cx.theme().border_primary)
            .child(header)
            .child(self.render_encoder_content(fixture, attribute, window, cx))
    }

    fn render_encoder_content(
        &self,
        fixture: &Fixture,
        attribute: &Attribute,
        window: &Window,
        cx: &App,
    ) -> impl IntoElement {
        let snapshot = EngineManager::snapshot(cx);
        let values = snapshot
            .selection()
            .fixture_ids()
            .iter()
            .filter_map(|fixture_id| {
                Some((
                    fixture_id,
                    snapshot.pipeline().attribute_info(
                        fixture_id,
                        attribute.name(),
                        snapshot.programmer(),
                    )?,
                ))
            })
            .collect::<Vec<_>>();

        let value_indicator = self.render_value_indicator(
            if values.len() > 0 && values.iter().all(|(_, v)| values[0].1.value == v.value) {
                Some(values[0].1)
            } else {
                None
            },
            fixture,
            attribute,
            window,
            cx,
        );

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

                let value_text = attribute.physical_unit().format_value(value.as_f32());
                let channel_set_label = current_cs.and_then(|cs| {
                    cs.name().map(|name| {
                        div().text_color(cx.theme().fg_secondary).child(name.to_string())
                    })
                });

                // FIXME: This does not update when the pipeline changes output. This also means that
                // if you're unlucky with the framedraw, it might not update properly when moving a fader for example.
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
                self.render_tab_group(fg_name, features, self.selected_feature.clone(), window, cx)
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
