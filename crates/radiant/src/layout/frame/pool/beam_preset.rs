use crate::show::{
    Show,
    asset::{AssetId, Preset},
    attr::BeamAttr,
};
use gpui::{ReadGlobal, div, prelude::*};

use super::{Pool, PoolDelegate};

pub struct BeamPresetPool {}

impl BeamPresetPool {
    pub fn new() -> Self {
        Self {}
    }
}

impl PoolDelegate for BeamPresetPool {
    type Item = Preset<BeamAttr>;

    fn title(&self, _cx: &mut Context<Pool<Self>>) -> &str {
        "Beam Presets"
    }

    fn render_cell_content(
        &mut self,
        asset_id: AssetId<Self::Item>,
        _w: &mut gpui::Window,
        cx: &mut Context<Pool<Self>>,
    ) -> Option<impl IntoElement> {
        let Some(beam_preset) =
            Show::global(cx).assets.beam_presets.get(&asset_id).map(|eg| eg.read(cx))
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
                .child(beam_preset.label.clone()),
        )
    }
}
