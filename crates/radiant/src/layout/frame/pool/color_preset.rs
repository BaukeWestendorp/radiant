use crate::show::{
    Show,
    asset::{AssetId, Preset},
    attr::ColorAttr,
};
use gpui::{ReadGlobal, div, prelude::*};

use super::{Pool, PoolDelegate};

pub struct ColorPresetPool {}

impl ColorPresetPool {
    pub fn new() -> Self {
        Self {}
    }
}

impl PoolDelegate for ColorPresetPool {
    type Item = Preset<ColorAttr>;

    fn title(&self, _cx: &mut Context<Pool<Self>>) -> &str {
        "Color Presets"
    }

    fn render_cell_content(
        &mut self,
        asset_id: AssetId<Self::Item>,
        _w: &mut gpui::Window,
        cx: &mut Context<Pool<Self>>,
    ) -> Option<impl IntoElement> {
        let Some(color_preset) =
            Show::global(cx).assets.color_presets.get(&asset_id).map(|eg| eg.read(cx))
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
                .child(color_preset.label.clone()),
        )
    }
}
