use gpui::prelude::*;
use gpui::{ElementId, Entity, ReadGlobal, Window, div};
use radiant::object::FixtureGroupId;
use radiant::patch::{AttributeInfo, FeatureGroup};

use crate::app::AppState;
use ui::{Disableable, Orientation, Tab, TabView, button, interactive_container, section};

pub struct AttributeEditor {
    tab_view: Entity<TabView>,
}

impl AttributeEditor {
    pub fn new(
        fixture_group_id: FixtureGroupId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let show = AppState::global(cx).engine.show();
        let fixture_group = show.fixture_group(fixture_group_id);
        let supported_feature_groups =
            fixture_group.map(|fg| fg.supported_feature_groups(show.patch())).unwrap_or_default();

        let tabs = FeatureGroup::ALL
            .into_iter()
            .map(|fg| {
                let name = fg.to_string();
                let editor = cx.new(|cx| FeatureGroupEditor::new(fixture_group_id, fg, window, cx));
                let has_fg = supported_feature_groups.contains(&fg);
                Tab::new(name.clone(), name, editor.into()).disabled(!has_fg)
            })
            .collect();

        Self {
            tab_view: cx.new(|cx| {
                let mut tab_view = TabView::new(tabs, window, cx);

                tab_view.set_orientation(Orientation::Vertical);

                tab_view
            }),
        }
    }
}

impl Render for AttributeEditor {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(self.tab_view.clone())
    }
}

struct FeatureGroupEditor {
    tab_view: Entity<TabView>,
}

impl FeatureGroupEditor {
    pub fn new(
        fixture_group_id: FixtureGroupId,
        feature_group: FeatureGroup,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let show = AppState::global(cx).engine.show();
        let tabs = match show.fixture_group(fixture_group_id) {
            Some(fg) => fg
                .supported_attributes(show.patch())
                .into_iter()
                .filter(|(attr, _)| attr.feature_group() == Some(feature_group))
                .map(|(attr, info)| {
                    Tab::new(
                        attr.to_string(),
                        attr.to_string(),
                        cx.new(|_| SingleAttributeEditor::new(info)).into(),
                    )
                })
                .collect(),
            None => Vec::new(),
        };

        let tab_view = cx.new(|cx| TabView::new(tabs, window, cx));

        Self { tab_view }
    }
}

impl Render for FeatureGroupEditor {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(self.tab_view.clone())
    }
}

struct SingleAttributeEditor {
    attribute_info: AttributeInfo,
}

impl SingleAttributeEditor {
    pub fn new(attribute_info: AttributeInfo) -> Self {
        Self { attribute_info }
    }
}

impl Render for SingleAttributeEditor {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let channel_set_buttons = div().flex().flex_wrap().gap_1().children(
            self.attribute_info.channel_sets().iter().map(|set| {
                button(ElementId::Name(set.name.clone().into()), None, set.name.clone()).on_click({
                    let from = set.from.clone();
                    move |_, _, _| {
                        log::info!("SET CHANNEL SET VALUE AT {:?}", from);
                    }
                })
            }),
        );
        div().size_full().p_2().child(section("Channel Sets").child(channel_set_buttons))
    }
}
