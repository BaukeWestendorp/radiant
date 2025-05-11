use gpui::{Entity, prelude::*};
use show::assets::{Asset, EffectGraph};

use super::{Pool, PoolDelegate};

pub struct EffectGraphPool {}

impl EffectGraphPool {
    pub fn new() -> Self {
        Self {}
    }
}

impl PoolDelegate for EffectGraphPool {
    type Item = Entity<Asset<EffectGraph>>;

    fn title(&self, _cx: &mut Context<Pool<Self>>) -> &str
    where
        Self: Sized,
    {
        "Effect Graphs"
    }

    fn render_cell_content(
        &mut self,
        asset_id: show::assets::AssetId<Self::Item>,
        w: &mut gpui::Window,
        cx: &mut Context<Pool<Self>>,
    ) -> impl IntoElement
    where
        Self: Sized,
    {
        "cell content"
    }
}
