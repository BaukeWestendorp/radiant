use gpui::*;
use ui::StyledExt;

use crate::showfile::{AnyAssetId, Showfile};

use super::PoolDelegate;

pub struct EffectPoolFrameDelegate;

impl EffectPoolFrameDelegate {
    pub fn new() -> Self {
        Self
    }
}

impl PoolDelegate for EffectPoolFrameDelegate {
    fn title(&self, _cx: &mut WindowContext) -> &str {
        "Effects"
    }

    fn render_pool_item(
        &mut self,
        id: AnyAssetId,
        cx: &mut WindowContext,
    ) -> Option<impl IntoElement> {
        let Some(graph) = Showfile::global(cx).assets().effect(&id.into()) else {
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
}
