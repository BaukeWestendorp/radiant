use gpui::*;
use show::{AnyAssetId, AssetPool, CueList};
use ui::StyledExt;

use super::PoolDelegate;

pub struct CueListPoolFrameDelegate {
    window: Model<show::Window>,
    asset_pool: Model<AssetPool<CueList>>,
}

impl CueListPoolFrameDelegate {
    pub fn new(window: Model<show::Window>, asset_pool: Model<AssetPool<CueList>>) -> Self {
        Self { window, asset_pool }
    }
}

impl PoolDelegate for CueListPoolFrameDelegate {
    fn title(&self, _cx: &mut WindowContext) -> String {
        "Cue Lists".to_string()
    }

    fn render_cell_content(
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

    fn on_select(&mut self, id: AnyAssetId, cx: &mut WindowContext) {
        self.window.update(cx, |window, cx| {
            window.set_selected_cuelist(Some(id.into()), cx);
            cx.notify();
        });
    }
}
