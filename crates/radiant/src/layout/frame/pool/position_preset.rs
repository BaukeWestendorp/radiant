use crate::show::{
    Show,
    asset::{AssetId, Preset},
    attr::PositionAttr,
};
use gpui::{ReadGlobal, div, prelude::*};

use super::{Pool, PoolDelegate};

pub struct PositionPresetPool {}

impl PositionPresetPool {
    pub fn new() -> Self {
        Self {}
    }
}

impl PoolDelegate for PositionPresetPool {
    type Item = Preset<PositionAttr>;

    fn title(&self, _cx: &mut Context<Pool<Self>>) -> &str {
        "Position Presets"
    }

    fn render_cell_content(
        &mut self,
        asset_id: AssetId<Self::Item>,
        _w: &mut gpui::Window,
        cx: &mut Context<Pool<Self>>,
    ) -> Option<impl IntoElement> {
        let Some(position_preset) =
            Show::global(cx).assets.position_presets.get(&asset_id).map(|eg| eg.read(cx))
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
                .child(position_preset.label.clone()),
        )
    }
}
