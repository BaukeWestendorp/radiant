use frames::{Frame, FrameWrapper};
use gpui::*;
use show::{Show, assets::EffectGraphDef};

pub use graph_editor::GraphEditor;

mod graph_editor;

pub enum MainFrame {
    EffectGraphEditor(Entity<GraphEditor<EffectGraphDef>>),
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
                    .get(&(*effect_graph_id).into())
                    .unwrap()
                    .clone();

                MainFrame::EffectGraphEditor(
                    cx.new(|cx| super::GraphEditor::new(graph, window, cx)),
                )
            }
        }
    }
}

impl Frame for MainFrame {
    fn render(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<FrameWrapper<Self>>,
    ) -> impl IntoElement {
        match self {
            MainFrame::EffectGraphEditor(entity) => entity.clone().into_any_element(),
        }
    }
}
