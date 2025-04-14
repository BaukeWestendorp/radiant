use crate::showfile::effect_graph;
use frames::{Frame, FrameWrapper};
use gpui::*;

pub use debugger::Debugger;
pub use graph_editor::GraphEditor;

mod debugger;
mod graph_editor;

pub enum MainFrame {
    EffectGraphEditor(Entity<GraphEditor<effect_graph::GraphDef>>),
    Debugger(Entity<Debugger>),
}

impl Frame for MainFrame {
    fn render(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<FrameWrapper<Self>>,
    ) -> impl IntoElement {
        match self {
            MainFrame::EffectGraphEditor(entity) => entity.clone().into_any_element(),
            MainFrame::Debugger(entity) => entity.clone().into_any_element(),
        }
    }
}
