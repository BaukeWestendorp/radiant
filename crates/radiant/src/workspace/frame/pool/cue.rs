use gpui::*;
use show::{AnyAssetId, AssetPool, Cue};
use ui::StyledExt;

use super::PoolDelegate;

pub struct CuePoolFrameDelegate {
    asset_pool: Model<AssetPool<Cue>>,
}

impl CuePoolFrameDelegate {
    pub fn new(asset_pool: Model<AssetPool<Cue>>) -> Self {
        Self { asset_pool }
    }
}

impl PoolDelegate for CuePoolFrameDelegate {
    fn title(&self, _cx: &mut WindowContext) -> String {
        "Cues".to_string()
    }

    fn render_pool_item(
        &mut self,
        id: AnyAssetId,
        cx: &mut WindowContext,
    ) -> Option<impl IntoElement> {
        let Some(cue) = self.asset_pool.read(cx).get(&id.into()) else {
            return None;
        };

        let label = cue.label.clone();

        Some(
            div()
                .size_full()
                .center_flex()
                .child(div().my_auto().child(label))
                .overflow_hidden(),
        )
    }
}
