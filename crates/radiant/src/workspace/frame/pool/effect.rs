use gpui::*;
use show::{AnyAssetId, AssetPool, Effect};
use ui::StyledExt;

use super::PoolDelegate;

pub struct EffectPoolFrameDelegate {
    asset_pool: Model<AssetPool<Effect>>,
}

impl EffectPoolFrameDelegate {
    pub fn new(asset_pool: Model<AssetPool<Effect>>) -> Self {
        Self { asset_pool }
    }
}

impl PoolDelegate for EffectPoolFrameDelegate {
    fn title(&self, _cx: &mut WindowContext) -> String {
        "Effects".to_string()
    }

    fn render_pool_item(
        &mut self,
        id: AnyAssetId,
        cx: &mut WindowContext,
    ) -> Option<impl IntoElement> {
        let Some(effect) = self.asset_pool.read(cx).get(&id.into()) else {
            return None;
        };

        let label = effect.label.clone();

        Some(
            div()
                .size_full()
                .center_flex()
                .child(div().my_auto().child(label))
                .overflow_hidden(),
        )
    }
}
