use crate::show::{
    Show,
    asset::{AssetId, Preset},
    attr::FocusAttr,
};
use gpui::{ReadGlobal, div, prelude::*};

use super::{Pool, PoolDelegate};

pub struct FocusPresetPool {}

impl FocusPresetPool {
    pub fn new() -> Self {
        Self {}
    }
}

impl PoolDelegate for FocusPresetPool {
    type Item = Preset<FocusAttr>;

    fn title(&self, _cx: &mut Context<Pool<Self>>) -> &str {
        "Focus Presets"
    }

    fn render_cell_content(
        &mut self,
        asset_id: AssetId<Self::Item>,
        _w: &mut gpui::Window,
        cx: &mut Context<Pool<Self>>,
    ) -> Option<impl IntoElement> {
        let Some(focus_preset) =
            Show::global(cx).assets.focus_presets.get(&asset_id).map(|eg| eg.read(cx))
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
                .child(focus_preset.label.clone()),
        )
    }
}
