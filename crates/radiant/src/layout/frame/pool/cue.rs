use crate::show::{
    Show,
    asset::{AssetId, Cue},
};
use gpui::{ReadGlobal, div, prelude::*};

use super::{Pool, PoolDelegate};

pub struct CuePool {}

impl CuePool {
    pub fn new() -> Self {
        Self {}
    }
}

impl PoolDelegate for CuePool {
    type Item = Cue;

    fn title(&self, _cx: &mut Context<Pool<Self>>) -> &str {
        "Cues"
    }

    fn render_cell_content(
        &mut self,
        asset_id: AssetId<Self::Item>,
        _w: &mut gpui::Window,
        cx: &mut Context<Pool<Self>>,
    ) -> Option<impl IntoElement> {
        let Some(cue) = Show::global(cx).assets.cues.get(&asset_id).map(|eg| eg.read(cx)) else {
            return None;
        };

        Some(
            div()
                .h_full()
                .flex()
                .flex_col()
                .justify_center()
                .text_center()
                .child(cue.label.clone()),
        )
    }
}
