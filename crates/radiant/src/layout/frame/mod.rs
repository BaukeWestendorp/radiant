use frames::{Frame, FrameWrapper};
use gpui::*;
use pool::{Pool, effect_graph::EffectGraphPool};
use show::{
    Show,
    assets::{AssetId, EffectGraphDef},
};

pub use graph_editor::GraphEditor;

mod graph_editor;
mod pool;

pub enum MainFrame {
    EffectGraphEditor(Entity<GraphEditor<EffectGraphDef>>),
    EffectGraphPool(Entity<Pool<EffectGraphPool>>),
}

impl MainFrame {
    pub fn from_show(
        frame: &show::layout::Frame<show::layout::MainFrameKind>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        match &frame.kind {
            show::layout::MainFrameKind::EffectGraphEditor(effect_graph_id) => {
                let graph = Show::global(cx)
                    .assets
                    .effect_graphs
                    .get(&AssetId::new(*effect_graph_id))
                    .unwrap()
                    .clone();

                MainFrame::EffectGraphEditor(
                    cx.new(|cx| super::GraphEditor::new(graph, window, cx)),
                )
            }
            show::layout::MainFrameKind::Pool(kind) => match kind {
                show::layout::PoolKind::EffectGraphs => MainFrame::EffectGraphPool(
                    cx.new(|_| Pool::new(EffectGraphPool::new(), frame.bounds.size)),
                ),
            },
        }
    }
}

impl Frame for MainFrame {
    fn render(
        &mut self,
        _w: &mut Window,
        _cx: &mut Context<FrameWrapper<Self>>,
    ) -> impl IntoElement {
        match self {
            MainFrame::EffectGraphEditor(entity) => entity.clone().into_any_element(),
            MainFrame::EffectGraphPool(pool) => pool.clone().into_any_element(),
        }
    }
}
