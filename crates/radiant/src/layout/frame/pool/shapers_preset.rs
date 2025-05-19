use crate::show::{
    Show,
    asset::{AssetId, Preset},
    attr::ShapersAttr,
};
use gpui::{ReadGlobal, div, prelude::*};

use super::{Pool, PoolDelegate};

pub struct ShapersPresetPool {}

impl ShapersPresetPool {
    pub fn new() -> Self {
        Self {}
    }
}

impl PoolDelegate for ShapersPresetPool {
    type Item = Preset<ShapersAttr>;

    fn title(&self, _cx: &mut Context<Pool<Self>>) -> &str {
        "Shapers Presets"
    }

    fn render_cell_content(
        &mut self,
        asset_id: AssetId<Self::Item>,
        _w: &mut gpui::Window,
        cx: &mut Context<Pool<Self>>,
    ) -> Option<impl IntoElement> {
        let Some(shapers_preset) =
            Show::global(cx).assets.shapers_presets.get(&asset_id).map(|eg| eg.read(cx))
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
                .child(shapers_preset.label.clone()),
        )
    }
}
