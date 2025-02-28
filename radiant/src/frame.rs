use frames::{Frame, FrameWrapper};
use gpui::*;

mod effect_graph_editor;

pub use effect_graph_editor::*;

pub enum MainFrame {
    EffectGraphEditor(Entity<EffectGraphEditor>),
}

impl Frame for MainFrame {
    fn render_content(&mut self, _cx: &mut Context<FrameWrapper<Self>>) -> impl IntoElement {
        match self {
            MainFrame::EffectGraphEditor(editor) => editor.clone(),
        }
    }
}
