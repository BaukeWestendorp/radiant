use gpui::*;
use ui::{container, theme::ActiveTheme, ContainerKind};

use super::{FrameDelegate, FrameView};

// TODO: Reimplement

pub struct EffectGraphEditorFrameDelegate {
    // graph_editor: View<GraphEditorView<EffectGraphDefinition>>,
}

impl EffectGraphEditorFrameDelegate {
    pub fn new(_cx: &mut WindowContext) -> Self {
        // let graph_model = cx.new_model(|cx| {
        //     Showfile::global(cx)
        //         .assets()
        //         .effect_graph(&EffectGraphId(1))
        //         .unwrap()
        //         .graph
        //         .clone()
        // });

        Self {
            // graph_editor: GraphEditorView::build(graph_model, cx),
        }
    }
}

impl FrameDelegate for EffectGraphEditorFrameDelegate {
    fn title(&mut self, _cx: &mut ViewContext<FrameView<Self>>) -> &str {
        "Effect Graph Editor"
    }

    fn render_content(&mut self, cx: &mut ViewContext<FrameView<Self>>) -> impl IntoElement {
        container(ContainerKind::Element, cx)
            .size_full()
            .border_color(cx.theme().frame_header_border)
            // .child(self.graph_editor.clone())
            .child("editor")
    }
}
