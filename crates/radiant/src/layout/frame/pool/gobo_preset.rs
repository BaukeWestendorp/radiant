use crate::show::{
    Show,
    asset::{AssetId, Preset},
    attr::GoboAttr,
};
use gpui::{ReadGlobal, div, prelude::*};

use super::{Pool, PoolDelegate};

pub struct GoboPresetPool {}

impl GoboPresetPool {
    pub fn new() -> Self {
        Self {}
    }
}

impl PoolDelegate for GoboPresetPool {
    type Item = Preset<GoboAttr>;

    fn title(&self, _cx: &mut Context<Pool<Self>>) -> &str {
        "Gobo Presets"
    }

    fn render_cell_content(
        &mut self,
        asset_id: AssetId<Self::Item>,
        _w: &mut gpui::Window,
        cx: &mut Context<Pool<Self>>,
    ) -> Option<impl IntoElement> {
        let Some(gobo_preset) =
            Show::global(cx).assets.gobo_presets.get(&asset_id).map(|eg| eg.read(cx))
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
                .child(gobo_preset.label.clone()),
        )
    }
}
