use crate::show::asset::effect::EffectGraphDef;
use crate::show::{self, Show};
use crate::ui::frame::{Frame, FrameWrapper};
use gpui::*;
use pool::{
    CuePool, DimmerPresetPool, EffectGraphPool, ExecutorPool, FixtureGroupPool, Pool, SequencePool,
};

pub use graph_editor::GraphEditor;

use crate::ui::vw::VirtualWindow;

mod graph_editor;
mod pool;

pub enum MainFrame {
    EffectGraphEditor(Entity<VirtualWindow<GraphEditor<EffectGraphDef>>>),

    EffectGraphPool(Entity<Pool<EffectGraphPool>>),
    FixtureGroupPool(Entity<Pool<FixtureGroupPool>>),

    CuePool(Entity<Pool<CuePool>>),
    SequencePool(Entity<Pool<SequencePool>>),
    ExecutorPool(Entity<Pool<ExecutorPool>>),

    DimmerPresetPool(Entity<Pool<DimmerPresetPool>>),
}

impl MainFrame {
    pub fn from_show(
        frame: &show::layout::Frame<show::layout::MainFrameKind>,
        w: &mut Window,
        cx: &mut App,
    ) -> Self {
        match &frame.kind {
            show::layout::MainFrameKind::EffectGraphEditor(effect_graph_id) => {
                let graph =
                    Show::global(cx).assets.effect_graphs.get(&effect_graph_id).unwrap().clone();

                MainFrame::EffectGraphEditor(
                    cx.new(|cx| VirtualWindow::new(GraphEditor::new(graph, w, cx))),
                )
            }
            show::layout::MainFrameKind::Pool(kind) => match kind {
                show::layout::PoolKind::EffectGraphs => MainFrame::EffectGraphPool(
                    cx.new(|cx| Pool::new(EffectGraphPool::new(), frame.bounds.size, cx)),
                ),
                show::layout::PoolKind::FixtureGroups => MainFrame::FixtureGroupPool(
                    cx.new(|cx| Pool::new(FixtureGroupPool::new(), frame.bounds.size, cx)),
                ),

                show::layout::PoolKind::Cues => MainFrame::CuePool(
                    cx.new(|cx| Pool::new(CuePool::new(), frame.bounds.size, cx)),
                ),
                show::layout::PoolKind::Sequences => MainFrame::SequencePool(
                    cx.new(|cx| Pool::new(SequencePool::new(), frame.bounds.size, cx)),
                ),
                show::layout::PoolKind::Executors => MainFrame::ExecutorPool(
                    cx.new(|cx| Pool::new(ExecutorPool::new(), frame.bounds.size, cx)),
                ),

                show::layout::PoolKind::DimmerPresets => MainFrame::DimmerPresetPool(
                    cx.new(|cx| Pool::new(DimmerPresetPool::new(), frame.bounds.size, cx)),
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
            MainFrame::FixtureGroupPool(pool) => pool.clone().into_any_element(),

            MainFrame::CuePool(pool) => pool.clone().into_any_element(),
            MainFrame::SequencePool(pool) => pool.clone().into_any_element(),
            MainFrame::ExecutorPool(pool) => pool.clone().into_any_element(),

            MainFrame::DimmerPresetPool(pool) => pool.clone().into_any_element(),
        }
    }
}
