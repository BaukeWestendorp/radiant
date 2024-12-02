use gpui::*;
use show::{AnyAssetId, AssetPool, EffectGraph};
use ui::StyledExt;

use super::PoolDelegate;

pub struct EffectGraphPoolFrameDelegate {
    asset_pool: Model<AssetPool<EffectGraph>>,
}

impl EffectGraphPoolFrameDelegate {
    pub fn new(asset_pool: Model<AssetPool<EffectGraph>>) -> Self {
        Self { asset_pool }
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
        let Some(graph) = self.asset_pool.read(cx).get(&id.into()) else {
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
