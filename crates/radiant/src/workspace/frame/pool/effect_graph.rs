use gpui::*;
use ui::StyledExt;

use crate::showfile::{AnyAssetId, Showfile};

use super::PoolDelegate;

pub struct EffectGraphPoolFrameDelegate;

impl EffectGraphPoolFrameDelegate {
    pub fn new() -> Self {
        Self
    }
}

impl PoolDelegate for EffectGraphPoolFrameDelegate {
    fn title(&self, _cx: &mut WindowContext) -> &str {
        "Effect Graphs"
    }

    fn render_pool_item(
        &mut self,
        id: AnyAssetId,
        cx: &mut WindowContext,
    ) -> Option<impl IntoElement> {
        let Some(graph) = Showfile::global(cx).assets().effect_graph(&id.into()) else {
            return None;
        };

        Some(
            div()
                .size_full()
                .center_flex()
                .child(div().my_auto().child(graph.label.clone()))
                .overflow_hidden(),
        )
    }

    fn on_click_item(&mut self, _id: AnyAssetId, _cx: &mut WindowContext) {
        // FIXME: All graph editors in current window and set them to the clicked graph.
    }
}
