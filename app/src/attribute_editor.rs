use gpui::prelude::*;
use gpui::{App, Entity, ReadGlobal, Window, div};
use radiant::object::FixtureGroup;
use radiant::patch::FeatureGroup;

use crate::app::AppState;
use ui::{Orientation, Tab, TabView};

pub struct AttributeEditor {
    tab_view: Entity<TabView>,
}

impl AttributeEditor {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let mut create_editor = |fg| cx.new(|_| FeatureGroupEditor::new(fg));
        let tabs = vec![
            Tab::new("dimmer", "Dimmer", create_editor(FeatureGroup::Dimmer).into()),
            Tab::new("position", "Position", create_editor(FeatureGroup::Position).into()),
            Tab::new("gobo", "Gobo", create_editor(FeatureGroup::Gobo).into()),
            Tab::new("color", "Color", create_editor(FeatureGroup::Color).into()),
            Tab::new("beam", "Beam", create_editor(FeatureGroup::Beam).into()),
            Tab::new("focus", "Focus", create_editor(FeatureGroup::Focus).into()),
            Tab::new("control", "Control", create_editor(FeatureGroup::Control).into()),
            Tab::new("shapers", "Shapers", create_editor(FeatureGroup::Shapers).into()),
            Tab::new("video", "Video", create_editor(FeatureGroup::Video).into()),
        ];

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
    feature_group: FeatureGroup,
}

impl FeatureGroupEditor {
    pub fn new(feature_group: FeatureGroup) -> Self {
        Self { feature_group }
    }

    fn fixture_group<'a>(&self, cx: &'a App) -> &'a FixtureGroup {
        let show = AppState::global(cx).engine.show();
        show.fixture_group(1).unwrap()
    }
}

impl Render for FeatureGroupEditor {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let show = AppState::global(cx).engine.show();

        let attrs = self
            .fixture_group(cx)
            .supported_attributes(show.patch())
            .into_iter()
            .filter(|attr| attr.feature_group() == Some(self.feature_group));

        div().children(attrs.map(|attr| format!("{:?}", attr)))
    }
}
