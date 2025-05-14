use crate::show::{
    Show,
    asset::{AssetId, FixtureGroup},
};
use gpui::{ReadGlobal as _, div, prelude::*};

use super::{Pool, PoolDelegate};

pub struct FixtureGroupPool {}

impl FixtureGroupPool {
    pub fn new() -> Self {
        Self {}
    }
}

impl PoolDelegate for FixtureGroupPool {
    type Item = FixtureGroup;

    fn title(&self, _cx: &mut Context<Pool<Self>>) -> &str {
        "Fixture Groups"
    }

    fn render_cell_content(
        &mut self,
        asset_id: AssetId<Self::Item>,
        _w: &mut gpui::Window,
        cx: &mut Context<Pool<Self>>,
    ) -> Option<impl IntoElement> {
        let Some(fixture_group) =
            Show::global(cx).assets.fixture_groups.get(&asset_id).map(|eg| eg.read(cx))
        else {
            return None;
        };

        Some(
            div()
                .h_full()
                .flex()
                .flex_col()
                .justify_center()
                .text_center()
                .child(fixture_group.label.clone()),
        )
    }
}
