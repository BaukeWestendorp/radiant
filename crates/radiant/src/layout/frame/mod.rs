use crate::showfile::{effect_graph, layout};
use flow::gpui::editor::GraphEditorView;
use frames::{Frame, FrameContainer, FrameWrapper};
use gpui::*;

pub use debug_frame::DebugFrame;
pub use graph_editor::GraphEditor;

mod debug_frame;
mod graph_editor;

const DEFAULT_FRAME_SIZE: Pixels = px(80.0);

pub enum MainFrame {
    EffectGraphEditor(Entity<GraphEditor<effect_graph::GraphDef>>),
    Debug(Entity<DebugFrame>),
}

impl Frame for MainFrame {
    fn render(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<FrameWrapper<Self>>,
    ) -> impl IntoElement {
        match self {
            MainFrame::EffectGraphEditor(entity) => entity.clone().into_any_element(),
            MainFrame::Debug(entity) => entity.clone().into_any_element(),
        }
    }
}

pub fn container_from_layout_main_window(
    main_window: &layout::Window,
    window: &mut Window,
    cx: &mut Context<FrameContainer<MainFrame>>,
) -> FrameContainer<MainFrame> {
    let container = frames::FrameContainer::new(main_window.size, DEFAULT_FRAME_SIZE);
    for frame in &main_window.frames {
        match &frame.kind {
            layout::K::EffectGraphEditor => {
                let editor = cx.new(|cx| GraphEditor::new(graph, window, cx));
                container.add_frame(MainFrame::EffectGraphEditor(editor), frame.bounds, cx);
            }
            _ => {}
        }
    }
    container
}
