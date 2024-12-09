use gpui::*;
use show::{AnyAssetId, AssetPool, EffectGraph};
use ui::StyledExt;

use super::PoolDelegate;

pub struct EffectGraphPoolFrameDelegate {
    window: Model<show::Window>,
    asset_pool: Model<AssetPool<EffectGraph>>,
}

impl EffectGraphPoolFrameDelegate {
    pub fn new(window: Model<show::Window>, asset_pool: Model<AssetPool<EffectGraph>>) -> Self {
        Self { window, asset_pool }
    }
}

impl PoolDelegate for EffectGraphPoolFrameDelegate {
    fn title(&self, _cx: &mut WindowContext) -> String {
        "Effect Graphs".to_string()
    }

    fn render_cell_content(
        &mut self,
        id: AnyAssetId,
        cx: &mut WindowContext,
    ) -> Option<impl IntoElement> {
        let Some(graph) = self.asset_pool.read(cx).get(&id.into()) else {
            return None;
        };

        let label = graph.label.clone();

        Some(
            div()
                .size_full()
                .center_flex()
                .child(div().my_auto().child(label))
                .overflow_hidden(),
        )
    }

    fn on_select(&mut self, id: AnyAssetId, cx: &mut WindowContext) {
        self.window.update(cx, |window, cx| {
            window.set_selected_effect_graph(Some(id.into()), cx);
            cx.notify();
        });
    }
}
