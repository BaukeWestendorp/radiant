use gpui::{ReadGlobal, div, prelude::*};
use show::{
    Show,
    assets::{AssetId, EffectGraph},
};

use super::{Pool, PoolDelegate};

pub struct EffectGraphPool {}

impl EffectGraphPool {
    pub fn new() -> Self {
        Self {}
    }
}

impl PoolDelegate for EffectGraphPool {
    type Item = EffectGraph;

    fn title(&self, _cx: &mut Context<Pool<Self>>) -> &str
    where
        Self: Sized,
    {
        "Effect Graphs"
    }

    fn render_cell_content(
        &mut self,
        asset_id: AssetId<Self::Item>,
        _w: &mut gpui::Window,
        cx: &mut Context<Pool<Self>>,
    ) -> Option<impl IntoElement>
    where
        Self: Sized,
    {
        let Some(effect_graph) =
            Show::global(cx).assets.effect_graphs.get(&asset_id).map(|eg| eg.read(cx))
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
                .child(effect_graph.label.clone()),
        )
    }
}
