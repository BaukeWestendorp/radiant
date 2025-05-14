use gpui::{ReadGlobal, UpdateGlobal, div, prelude::*};
use show::{
    Show,
    asset::{AssetId, EffectGraph},
    layout::MainFrameKind,
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

    fn title(&self, _cx: &mut Context<Pool<Self>>) -> &str {
        "Effect Graphs"
    }

    fn render_cell_content(
        &mut self,
        asset_id: AssetId<Self::Item>,
        _w: &mut gpui::Window,
        cx: &mut Context<Pool<Self>>,
    ) -> Option<impl IntoElement> {
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

    fn on_select(&mut self, asset_id: AssetId<Self::Item>, cx: &mut Context<Pool<Self>>) {
        Show::update_global(cx, |show, cx| {
            show.layout.update(cx, |layout, cx| {
                for frame in &mut layout.main_window.frames {
                    match &mut frame.kind {
                        MainFrameKind::EffectGraphEditor(effect_graph) => *effect_graph = asset_id,
                        _ => {}
                    }
                }
                cx.notify();
            });
            cx.notify();
        })
    }
}
