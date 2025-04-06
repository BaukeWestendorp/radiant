use crate::effect_graph;
use frames::{Frame, FrameWrapper};
use gpui::*;

pub use debug_frame::DebugFrame;
pub use graph_editor::GraphEditor;

mod debug_frame;
mod graph_editor;

pub enum MainFrame {
    EffectGraphEditor(Entity<GraphEditor<effect_graph::GraphDef>>),
    DebugFrame(Entity<DebugFrame>),
}

impl Frame for MainFrame {
    fn render(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<FrameWrapper<Self>>,
    ) -> impl IntoElement {
        match self {
            MainFrame::EffectGraphEditor(entity) => entity.clone().into_any_element(),
            MainFrame::DebugFrame(entity) => entity.clone().into_any_element(),
        }
    }
}
