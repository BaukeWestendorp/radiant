use gpui::*;
use show::{AnyAssetId, AssetPool, Group};
use ui::StyledExt;

use super::PoolDelegate;

pub struct GroupPoolFrameDelegate {
    asset_pool: Model<AssetPool<Group>>,
}

impl GroupPoolFrameDelegate {
    pub fn new(asset_pool: Model<AssetPool<Group>>) -> Self {
        Self { asset_pool }
    }
}

impl PoolDelegate for GroupPoolFrameDelegate {
    fn title(&self, _cx: &mut WindowContext) -> String {
        "Groups".to_string()
    }

    fn render_pool_item(
        &mut self,
        id: AnyAssetId,
        cx: &mut WindowContext,
    ) -> Option<impl IntoElement> {
        let Some(group) = self.asset_pool.read(cx).get(&id.into()) else {
            return None;
        };

        let label = group.label.clone();

        Some(
            div()
                .size_full()
                .center_flex()
                .child(div().my_auto().child(label))
                .overflow_hidden(),
        )
    }
}
