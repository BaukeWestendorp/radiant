use crate::effect_graph;
use frames::{Frame, FrameWrapper};
use gpui::*;

pub use graph_editor::*;

mod graph_editor;

pub enum MainFrame {
    EffectGraphEditor(Entity<GraphEditor<effect_graph::GraphDef>>),
}

impl Frame for MainFrame {
    fn render_content(&mut self, _cx: &mut Context<FrameWrapper<Self>>) -> impl IntoElement {
        match self {
            MainFrame::EffectGraphEditor(editor) => editor.clone(),
        }
    }
}
