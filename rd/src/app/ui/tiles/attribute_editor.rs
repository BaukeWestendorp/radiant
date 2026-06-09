use std::collections::HashMap;

use gpui::{
    AnyElement, App, Bounds, Entity, FontWeight, MouseButton, SharedString, Window, div,
    prelude::*, px,
};
use rd_engine::{
    FixtureCollection,
    cmd::Command,
    gdtf::{
        Gdtf, Name,
        attr::AttributeName,
        dmx::{DmxMode, LogicalChannel},
    },
    value::{AttributeValue, ClampedValue},
};
use rd_ui::{
    ActiveTheme as _, Button, HslaExt, Scrollable, ScrollableState, TileDelegate, h_flex, todo,
    v_flex,
};

use crate::engine::EngineManager;

pub struct AttributeEditorTile {
    selected_attribute: Entity<Option<AttributeName>>,
    selected_cf: Entity<Option<Name>>,
    channel_sets_scrollable: Entity<ScrollableState>,
}

impl AttributeEditorTile {
    pub fn new(_window: &mut Window, cx: &mut App) -> Self {
        let selected_attribute = cx.new(|_| None);
        let selected_cf = cx.new(|_| None);

        cx.observe(&EngineManager::selection(cx).clone(), {
            let selected_attribute = selected_attribute.clone();
            let selected_cf = selected_cf.clone();
            move |_, cx| {
                selected_attribute.write(cx, None);
                selected_cf.write(cx, None);
            }
        })
        .detach();

        Self {
            selected_attribute,
            selected_cf,
            channel_sets_scrollable: cx.new(|_| ScrollableState::new()),
        }
    }

    fn render_attribute_tree(
        &self,
        gdtf: &Gdtf,
        dmx_mode: &DmxMode,
        _window: &mut Window,
        cx: &App,
    ) -> impl IntoElement {
        let tree = generate_lc_tree(gdtf, dmx_mode);

        div().flex().size_full().bg(cx.theme().bg_secondary).children(tree.iter().map({
            let selected_attribute = self.selected_attribute.clone();
            let selected_cf = self.selected_cf.clone();
            move |(feature_name, lcs)| {
                v_flex()
                    .h_full()
                    // FIXME: Use cell size instead.
                    .w(px(80.0))
                    .text_sm()
                    .border_color(cx.theme().border_primary)
                    .border_r_1()
                    .bg(cx.theme().bg_primary)
                    .child(
                        div()
                            .font_weight(FontWeight::BOLD)
                            .text_center()
                            .bg(cx.theme().bg_tertiary)
                            .border_b_1()
                            .border_color(cx.theme().border_tertiary)
                            .child(feature_name.to_string()),
                    )
                    .children(
                        lcs.iter()
                            .filter_map(|lc| {
                                Some((
                                    lc,
                                    lc.attribute(gdtf)?.name().clone(),
                                    lc.attribute(gdtf)?.pretty_name().to_string(),
                                ))
                            })
                            .map({
                                let selected_attribute = selected_attribute.clone();
                                let selected_cf = selected_cf.clone();
                                move |(_, attribute_name, pretty_name)| {
                                    let selected_attribute = selected_attribute.clone();
                                    let selected_cf = selected_cf.clone();
                                    let is_selected = selected_attribute
                                        .read(cx)
                                        .as_ref()
                                        .is_some_and(|sel| attribute_name == *sel);
                                    let (bg, border) = if is_selected {
                                        (cx.theme().bg_selected, cx.theme().border_selected)
                                    } else {
                                        (cx.theme().bg_secondary, cx.theme().border_secondary)
                                    };

                                    div()
                                        .id(attribute_name.to_string())
                                        .text_center()
                                        .bg(bg)
                                        .border_b_1()
                                        .border_color(border)
                                        .when(cx.theme().shadow, |e| e.shadow_xs())
                                        .hover(|e| e.bg(bg.hover()))
                                        .active(|e| {
                                            e.bg(bg.active()).top(cx.theme().button_depression)
                                        })
                                        .child(pretty_name.to_string())
                                        .on_mouse_down(MouseButton::Left, {
                                            let selected_attribute = selected_attribute.clone();
                                            let selected_cf = selected_cf.clone();
                                            move |_, _, cx| {
                                                selected_attribute
                                                    .write(cx, Some(attribute_name.clone()));
                                                // FIXME: Select initial channel function.
                                                selected_cf.write(cx, None);
                                            }
                                        })
                                }
                            }),
                    )
            }
        }))
    }

    fn render_value_editor(
        &self,
        gdtf: &Gdtf,
        dmx_mode: &DmxMode,
        _window: &mut Window,
        cx: &App,
    ) -> impl IntoElement {
        let Some(attribute_name) = self.selected_attribute.read(cx) else {
            return div();
        };

        let Some(logical_channel) = dmx_mode.logical_channel(attribute_name) else {
            return div();
        };

        let cf_picker = v_flex()
            .w(px(120.0))
            .h_full()
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .text_center()
                    .bg(cx.theme().bg_tertiary)
                    .border_b_1()
                    .border_color(cx.theme().border_tertiary)
                    .child(
                        logical_channel
                            .attribute(gdtf)
                            .clone()
                            .map(|attr| attr.pretty_name().to_string())
                            .unwrap_or("Ch Fn".to_owned()),
                    ),
            )
            .child(
                div().id("cf-picker").size_full().overflow_scroll().children(
                    logical_channel
                        .channel_functions()
                        .iter()
                        .filter(|cf| {
                            cf.attribute(gdtf)
                                .is_some_and(|a| *a.name() != AttributeName::NoFeature)
                        })
                        .map({
                            let selected_cf = self.selected_cf.clone();
                            move |cf| {
                                let is_selected = selected_cf
                                    .read(cx)
                                    .as_ref()
                                    .is_some_and(|sel| cf.name() == sel);
                                let (bg, border) = if is_selected {
                                    (cx.theme().bg_selected, cx.theme().border_selected)
                                } else {
                                    (cx.theme().bg_secondary, cx.theme().border_secondary)
                                };

                                let cf_name = cf.name().clone();
                                div()
                                    .id(cf_name.to_string())
                                    .text_center()
                                    .bg(bg)
                                    .border_b_1()
                                    .border_color(border)
                                    .when(cx.theme().shadow, |e| e.shadow_xs())
                                    .hover(|e| e.bg(bg.hover()))
                                    .active(|e| e.bg(bg.active()).top(cx.theme().button_depression))
                                    .child(cf_name.to_string())
                                    .on_mouse_down(MouseButton::Left, {
                                        let selected_cf = selected_cf.clone();
                                        move |_, _, cx| {
                                            selected_cf.write(cx, Some(cf_name.clone()));
                                        }
                                    })
                            }
                        }),
                ),
            );

        let channel_sets = self
            .selected_cf
            .read(cx)
            .as_ref()
            .and_then(|cf_name| logical_channel.channel_function(&cf_name))
            .map(|cf| cf.channel_sets())
            .map(|channel_sets| {
                div().flex().flex_wrap().flex_shrink_1().gap_2().p_2().children(
                    channel_sets
                        .iter()
                        .filter(|cs| cs.name().is_some_and(|name| !name.as_str().is_empty()))
                        .map(|cs| {
                            let cs_name = cs.name().unwrap().to_string();
                            let value = ClampedValue::from(cs.dmx_from());
                            let attribute_name = attribute_name.clone();
                            Button::new(cs_name.clone()).child(cs_name.clone()).on_click(
                                move |_, _, cx| {
                                    let fixtures = FixtureCollection::Multiple(
                                        EngineManager::snapshot(cx)
                                            .selection()
                                            .fixture_ids()
                                            .to_vec(),
                                    );

                                    EngineManager::execute(
                                        cx,
                                        Command::ProgrammerSet {
                                            fixtures,
                                            attribute: attribute_name.clone(),
                                            value: AttributeValue::Clamped(value),
                                        },
                                    );
                                },
                            )
                        }),
                )
            });

        h_flex()
            .size_full()
            .text_sm()
            .child(
                div()
                    .h_full()
                    .border_r_1()
                    .border_color(cx.theme().border_primary)
                    .child(cf_picker),
            )
            .child(
                Scrollable::new("channel-sets", self.channel_sets_scrollable.clone())
                    .children(channel_sets)
                    .size_full(),
            )
    }
}

impl TileDelegate for AttributeEditorTile {
    fn title(&self, _cx: &App) -> SharedString {
        "Attribute Editor".into()
    }

    fn render_content(&self, _bounds: Bounds<u32>, window: &mut Window, cx: &App) -> AnyElement {
        let snapshot = EngineManager::snapshot(cx);
        let dmx_modes = snapshot.selection().unique_dmx_modes(snapshot.patch());
        let (gdtf, dmx_mode) = match dmx_modes.len() {
            0 => return div().into_any_element(), // FIXME: Add text indicating no selection.
            1 => dmx_modes[0],
            _ => return todo(cx).into_any_element(),
        };

        h_flex()
            .size_full()
            .child(self.render_attribute_tree(gdtf, dmx_mode, window, cx))
            .child(
                div()
                    .w_full()
                    .h_full()
                    .border_l_1()
                    .border_color(cx.theme().border_primary)
                    .child(self.render_value_editor(gdtf, dmx_mode, window, cx)),
            )
            .into_any_element()
    }
}

fn generate_lc_tree<'a>(
    gdtf: &'a Gdtf,
    dmx_mode: &'a DmxMode,
) -> Vec<(Name, Vec<&'a LogicalChannel>)> {
    let mut groups = HashMap::<Name, Vec<&'a LogicalChannel>>::new();

    for lc in dmx_mode.logical_channels() {
        let Some(attribute) = lc.attribute(gdtf) else { continue };
        let Some(feature) = attribute.feature(gdtf) else { continue };

        groups.entry(feature.name().clone()).or_default().push(lc);
    }

    let mut tree: Vec<(Name, Vec<&LogicalChannel>)> = groups.into_iter().collect();

    tree.sort_by_key(|(f_name, _)| {
        let x = dmx_mode
            .logical_channels()
            .find_map(|lc| {
                let attr = lc.attribute(gdtf)?;
                if attr.feature(gdtf)?.name() == f_name {
                    let fg_name = attr.feature_group(gdtf)?.name();
                    match fg_name.as_str() {
                        "Dimmer" => Some(0),
                        "Position" => Some(1),
                        "Gobo" => Some(2),
                        "Color" => Some(3),
                        "Beam" => Some(4),
                        "Focus" => Some(5),
                        "Control" => Some(6),
                        "Shapers" => Some(7),
                        "Video" => Some(8),
                        _ => Some(u8::MAX),
                    }
                } else {
                    None
                }
            })
            .unwrap_or(u8::MAX);

        (x, f_name.clone())
    });

    tree
}
