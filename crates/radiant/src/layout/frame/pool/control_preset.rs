use crate::show::{
    Show,
    asset::{AssetId, Preset},
    attr::ControlAttr,
};
use gpui::{ReadGlobal, div, prelude::*};

use super::{Pool, PoolDelegate};

pub struct ControlPresetPool {}

impl ControlPresetPool {
    pub fn new() -> Self {
        Self {}
    }
}

impl PoolDelegate for ControlPresetPool {
    type Item = Preset<ControlAttr>;

    fn title(&self, _cx: &mut Context<Pool<Self>>) -> &str {
        "Control Presets"
    }

    fn render_cell_content(
        &mut self,
        asset_id: AssetId<Self::Item>,
        _w: &mut gpui::Window,
        cx: &mut Context<Pool<Self>>,
    ) -> Option<impl IntoElement> {
        let Some(control_preset) =
            Show::global(cx).assets.control_presets.get(&asset_id).map(|eg| eg.read(cx))
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
                .child(control_preset.label.clone()),
        )
    }
}
