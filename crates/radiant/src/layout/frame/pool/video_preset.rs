use crate::show::{
    Show,
    asset::{AssetId, Preset},
    attr::VideoAttr,
};
use gpui::{ReadGlobal, div, prelude::*};

use super::{Pool, PoolDelegate};

pub struct VideoPresetPool {}

impl VideoPresetPool {
    pub fn new() -> Self {
        Self {}
    }
}

impl PoolDelegate for VideoPresetPool {
    type Item = Preset<VideoAttr>;

    fn title(&self, _cx: &mut Context<Pool<Self>>) -> &str {
        "Video Presets"
    }

    fn render_cell_content(
        &mut self,
        asset_id: AssetId<Self::Item>,
        _w: &mut gpui::Window,
        cx: &mut Context<Pool<Self>>,
    ) -> Option<impl IntoElement> {
        let Some(video_preset) =
            Show::global(cx).assets.video_presets.get(&asset_id).map(|eg| eg.read(cx))
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
                .child(video_preset.label.clone()),
        )
    }
}
