use flow::gpui::GraphEditorView;
use gpui::*;
use ui::theme::ActiveTheme;

use crate::showfile::{EffectGraphDefinition, Showfile};

use super::{Frame, FrameDelegate};

pub struct EffectGraphEditorFrameDelegate {
    graph_editor: View<GraphEditorView<EffectGraphDefinition>>,
}

impl EffectGraphEditorFrameDelegate {
    pub fn new(cx: &mut WindowContext) -> Self {
        let graph_model = cx.new_model(|cx| {
            Showfile::global(cx)
                .assets()
                .effect_graph(&1)
                .unwrap()
                .clone()
        });

        Self {
            graph_editor: GraphEditorView::build(graph_model, cx),
        }
    }
}

impl FrameDelegate for EffectGraphEditorFrameDelegate {
    fn title(&mut self, _cx: &mut ViewContext<Frame<Self>>) -> &str
    where
        Self: Sized,
    {
        "Effect Graph Editor"
    }

    fn render_content(&mut self, cx: &mut ViewContext<Frame<Self>>) -> impl IntoElement
    where
        Self: Sized,
    {
        div()
            .size_full()
            .child(self.graph_editor.clone())
            .border_1()
            .border_color(cx.theme().border)
            .rounded(cx.theme().radius)
    }
}
